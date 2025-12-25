use super::convert_to_market_data;
use super::convert_to_stock_data;
use crate::slint_generatedAppWindow::{
    MarketWatchData as SlintMarketWatchData, OrderList as SlintOrderList,
    StockData as SlintStockData,
};
use crate::tasks::chart::create_sector_groups;
use crate::tasks::chart::is_trading_hours;
use crate::tasks::chart::sort_market_watch;
use crate::tasks::chart::ALL_STOCK_LIST;
use crate::tasks::task_manager::TaskStatus;
use crate::tasks::task_manager::{register_task, TaskHandle};
use crate::tasks::DataUpdate;
use aim_data::explorer::vci::VCIOderBook;
use aim_data::get_market_watch;
use aim_data::get_order_list;
use chrono::Timelike;
use slint::Weak;
use slint::{ComponentHandle, ModelRc};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use super::VN30_LIST;

/// Spawns a task to handle market watch data updates
pub async fn spawn_data_update_task(
    ui: &crate::slint_generatedAppWindow::AppWindow,
    custom_list: Arc<Mutex<Vec<String>>>,
) -> Vec<TaskHandle> {
    let mut handles = Vec::new();
    let (tx_data_update, rx_data_update) = mpsc::channel::<DataUpdate>(10);
    let ui_handle = ui.as_weak();

    // Create individual task handles for each sub-task
    handles.push(spawn_custom_list_polling_task(tx_data_update.clone(), custom_list).await);
    handles.push(spawn_stock_data_polling_task(tx_data_update.clone()).await);
    handles.push(spawn_market_watch_polling_task(tx_data_update.clone()).await);
    handles.push(spawn_order_list_polling_task(tx_data_update.clone(), ui_handle.clone()).await);
    handles.push(spawn_ui_update_task(ui_handle, rx_data_update).await);

    handles
}

// Individual task spawning functions with task manager integration
async fn spawn_custom_list_polling_task(
    tx_data: mpsc::Sender<DataUpdate>,
    custom_list: Arc<Mutex<Vec<String>>>,
) -> TaskHandle {
    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.data_update.custom_list".to_string(),
        tx,
        "Custom List Polling Task".to_string(),
    )
    .await;

    tokio::spawn(async move {
        polling_custom_list(tx_data, rx, custom_list).await;
    });

    task_handle
}

async fn spawn_stock_data_polling_task(tx: mpsc::Sender<DataUpdate>) -> TaskHandle {
    let (tx_task, rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.data_update.stock_data".to_string(),
        tx_task,
        "Stock Data Polling Task".to_string(),
    )
    .await;

    tokio::spawn(async move {
        polling_all_stock_data(tx, rx).await;
    });

    task_handle
}

async fn spawn_market_watch_polling_task(tx: mpsc::Sender<DataUpdate>) -> TaskHandle {
    let (tx_status, rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.data_update.market_watch".to_string(),
        tx_status,
        "Market Watch Polling Task".to_string(),
    )
    .await;

    tokio::spawn(async move {
        polling_market_watch(tx, rx).await;
    });

    task_handle
}

async fn spawn_order_list_polling_task(
    tx: mpsc::Sender<DataUpdate>,
    ui_handle: Weak<crate::slint_generatedAppWindow::AppWindow>,
) -> TaskHandle {
    let (tx_task, rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.data_update.order_list".to_string(),
        tx_task,
        "Order List Polling Task".to_string(),
    )
    .await;

    tokio::spawn(async move {
        polling_order_list(tx, rx, ui_handle).await;
    });

    task_handle
}

async fn spawn_ui_update_task(
    ui_handle: Weak<crate::slint_generatedAppWindow::AppWindow>,
    rx: mpsc::Receiver<DataUpdate>,
) -> TaskHandle {
    let (tx, rx_task) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.data_update.ui_update".to_string(),
        tx,
        "UI Update Task".to_string(),
    )
    .await;

    tokio::spawn(async move {
        update_ui_with_data(ui_handle, rx, rx_task).await;
    });

    task_handle
}

async fn polling_custom_list(
    tx: mpsc::Sender<DataUpdate>,
    mut rx: mpsc::Receiver<TaskStatus>,
    custom_list: Arc<Mutex<Vec<String>>>,
) {
    let mut previous_custom_list: Option<Vec<String>> = None;
    let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
    loop {
        if let Ok(status) = rx.try_recv() {
            if task_status != status {
                log::info!("Cache storage task status changed to: {:?}", status);
                task_status = status;
            }
        }
        if task_status != crate::tasks::task_manager::TaskStatus::Running {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            continue;
        }
        let custom_symbols = custom_list.lock().await;
        let custom_symbol_slices: Vec<String> = custom_symbols.iter().cloned().collect();
        drop(custom_symbols);
        // Check if custom list changed (this should always be checked regardless of trading hours)
        let custom_list_changed = {
            if let Some(ref previous) = previous_custom_list {
                // Compare current custom symbols with previous ones
                // Check if length is different or if any symbol is missing
                previous.len() != custom_symbol_slices.len()
                    || !previous
                        .iter()
                        .all(|symbol| custom_symbol_slices.contains(symbol))
                    || !custom_symbol_slices
                        .iter()
                        .all(|symbol| previous.contains(symbol))
            } else {
                // First time - consider it changed if we have any custom symbols
                !custom_symbol_slices.is_empty()
            }
        };

        if custom_list_changed {
            previous_custom_list = Some(custom_symbol_slices.clone());
            tx.send(DataUpdate::CustomList(custom_symbol_slices))
                .await
                .ok();
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn polling_order_list(
    tx: mpsc::Sender<DataUpdate>,
    mut rx: mpsc::Receiver<TaskStatus>,
    ui_handle: Weak<crate::slint_generatedAppWindow::AppWindow>,
) {
    let current_stock = Arc::new(Mutex::new(String::from("AAA")));
    let mut previous_order_list: Option<Vec<VCIOderBook>> = None;
    let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
    loop {
        if let Ok(status) = rx.try_recv() {
            if task_status != status {
                log::info!("Cache storage task status changed to: {:?}", status);
                task_status = status;
            }
        }
        if task_status != crate::tasks::task_manager::TaskStatus::Running {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            continue;
        }
        // Get current stock symbol
        let current_stock_str = {
            let current = current_stock.lock().await;
            current.clone()
        };

        let current_stock_clone = Arc::clone(&current_stock);
        let ui_handle_clone = ui_handle.clone();
        let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
            let symbol = ui.get_current_stock().symbol.to_string();
            let current_stock_clone = Arc::clone(&current_stock_clone);
            // Spawn a new async task to update the mutex asynchronously
            tokio::spawn(async move {
                let mut current = current_stock_clone.lock().await;
                *current = symbol;
            });
        });

        log::info!("Polling order list for stock: {current_stock_str}");
        let (order_changed, order_list) = match get_order_list(&current_stock_str).await {
            Ok(order_list) => {
                if let Some(previous) = &previous_order_list {
                    if previous.is_empty()
                        || order_list.is_empty()
                        || previous[0].id != order_list[0].id
                    {
                        previous_order_list = Some(order_list.clone());
                        (true, order_list)
                    } else {
                        (false, previous.clone())
                    }
                } else {
                    previous_order_list = Some(order_list.clone());
                    (true, order_list)
                }
            }
            Err(e) => {
                log::error!("Failed to fetch order list: {e}. try again ...");
                continue;
            }
        };

        if order_changed {
            previous_order_list = Some(order_list.clone());
            tx.send(DataUpdate::OrdList(order_list)).await.ok();
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn polling_all_stock_data(tx: mpsc::Sender<DataUpdate>, mut rx: mpsc::Receiver<TaskStatus>) {
    let mut previous_stock_data: Option<Vec<SlintStockData>> = None;
    let mut is_first_update = true; // Track if this is the first update
    let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
    loop {
        if let Ok(status) = rx.try_recv() {
            if task_status != status {
                log::info!("Cache storage task status changed to: {:?}", status);
                task_status = status;
            }
        }
        if task_status != crate::tasks::task_manager::TaskStatus::Running {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            continue;
        }
        // Check if we're in trading hours (9:00 AM - 3:00 PM Vietnam time)
        let is_trading_hours = is_trading_hours();

        // If not in trading hours and we've already done the first update,
        // only continue if custom list changed, otherwise skip data fetching
        if !is_trading_hours && !is_first_update {
            log::info!("Outside trading hours (9:00-15:00 Vietnam time), skipping data update");
            tokio::time::sleep(Duration::from_millis(100)).await;
            continue;
        }

        // Fetch market data
        let market_watch_data = match get_market_watch(&ALL_STOCK_LIST).await {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to fetch market watch data: {e}. try again ...");
                continue;
            }
        };
        let mut all_stock_data: Vec<SlintStockData> = market_watch_data
            .0
            .iter()
            .map(convert_to_stock_data)
            .collect();

        // Check if market watch data has changed
        let market_watch_changed =
            has_stock_data_changed(&previous_stock_data, &mut all_stock_data);

        if market_watch_changed {
            is_first_update = false;
            previous_stock_data = Some(all_stock_data.clone());
            tx.send(DataUpdate::StockData(all_stock_data)).await.ok();
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn polling_market_watch(tx: mpsc::Sender<DataUpdate>, mut rx: mpsc::Receiver<TaskStatus>) {
    let mut previous_market_watch_data: Option<Vec<SlintMarketWatchData>> = None;
    let mut is_first_update = true; // Track if this is the first update
    let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
    loop {
        if let Ok(status) = rx.try_recv() {
            if task_status != status {
                log::info!("Cache storage task status changed to: {:?}", status);
                task_status = status;
            }
        }
        if task_status != crate::tasks::task_manager::TaskStatus::Running {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            continue;
        }
        // Check if we're in trading hours (9:00 AM - 3:00 PM Vietnam time)
        let is_trading_hours = super::is_trading_hours();

        // If not in trading hours and we've already done the first update,
        // only continue if custom list changed, otherwise skip data fetching
        if !is_trading_hours && !is_first_update {
            log::info!("Outside trading hours (9:00-15:00 Vietnam time), skipping data update");
            tokio::time::sleep(Duration::from_millis(100)).await;
            continue;
        }

        // Fetch market data
        let market_watch_data = match get_market_watch(&VN30_LIST).await {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to fetch market watch data: {e}. try again ...");
                continue;
            }
        };
        let market_watch_stock_data: Vec<SlintMarketWatchData> = market_watch_data
            .0
            .iter()
            .map(convert_to_market_data)
            .collect();

        // Check if market watch data has changed
        let market_watch_changed =
            has_market_watch_changed(&previous_market_watch_data, &market_watch_stock_data);

        if market_watch_changed {
            is_first_update = false;
            previous_market_watch_data = Some(market_watch_stock_data.clone());
            tx.send(DataUpdate::MarketWatchData(market_watch_stock_data))
                .await
                .ok();
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn update_ui_with_data(
    ui_handle: Weak<crate::slint_generatedAppWindow::AppWindow>,
    mut rx: mpsc::Receiver<DataUpdate>,
    mut rx_status: mpsc::Receiver<TaskStatus>,
) {
    let mut previous_stock_data: Option<Vec<SlintStockData>> = None;
    let mut previous_custom_list: Option<Vec<String>> = None;
    let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
    loop {
        if let Ok(status) = rx_status.try_recv() {
            if task_status != status {
                log::info!("Cache storage task status changed to: {:?}", status);
                task_status = status;
            }
        }
        if task_status != crate::tasks::task_manager::TaskStatus::Running {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            continue;
        }
        if let Ok(update) = rx.try_recv() {
            match update {
                DataUpdate::MarketWatchData(data) => {
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        // Check if there's an active sort and apply it to maintain sort order
                        let sort_column = ui.get_market_watch_sort_column();
                        let sort_ascending = ui.get_market_watch_sort_ascending();
                        let show_percentage = ui.get_market_watch_show_percentage();

                        let sorted_data = if sort_column >= 0 {
                            // Apply current sort to the new data
                            sort_market_watch(&data, sort_column, sort_ascending, show_percentage)
                        } else {
                            // No sort applied, use data as-is
                            data
                        };

                        ui.set_market_watch_data(ModelRc::new(slint::VecModel::from(sorted_data)));
                    });
                }
                DataUpdate::StockData(data) => {
                    previous_stock_data = Some(data.clone());
                    let previous_custom_list_clone = previous_custom_list.clone();
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        let sort_type = ui.get_sort_type();
                        let grouped_stock_data = if let Some(previous) = &previous_custom_list_clone
                        {
                            create_sector_groups(sort_type, data, previous.clone())
                        } else {
                            create_sector_groups(sort_type, data, vec![])
                        };
                        ui.set_stock_groups(ModelRc::new(slint::VecModel::from(
                            grouped_stock_data,
                        )));
                    });
                }
                DataUpdate::OrdList(data) => {
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        let slint_order_list: Vec<SlintOrderList> = data
                            .iter()
                            .map(|order| {
                                let time_str = match order.timestamp.parse::<i64>() {
                                    Ok(ts) => chrono::DateTime::from_timestamp(ts, 0)
                                        .map(|dt| {
                                            format!(
                                                "{:02}:{:02}:{:02}",
                                                dt.hour() + 7,
                                                dt.minute(),
                                                dt.second()
                                            )
                                        })
                                        .unwrap_or_else(|| order.timestamp.clone()),
                                    Err(_) => order.timestamp.clone(),
                                };
                                SlintOrderList {
                                    match_type: order.match_type.clone().into(),
                                    price: (order.price as f32) / 1000.0,
                                    time: time_str.into(),
                                    vol: order.volume as i32,
                                }
                            })
                            .collect();
                        ui.set_order_list(ModelRc::new(slint::VecModel::from(slint_order_list)));
                    });
                }
                DataUpdate::CustomList(items) => {
                    previous_custom_list = Some(items.clone());
                    let previous_stock_data_clone = previous_stock_data.clone();
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        if let Some(previous) = previous_stock_data_clone {
                            let sort_type = ui.get_sort_type();
                            let grouped_stock_data =
                                super::create_sector_groups(sort_type, previous, items.clone());
                            ui.set_stock_groups(ModelRc::new(slint::VecModel::from(
                                grouped_stock_data,
                            )));
                        } else {
                            // no thing update
                        }
                    });
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// Checks if market watch data has changed
fn has_market_watch_changed(
    previous: &Option<Vec<SlintMarketWatchData>>,
    current: &[SlintMarketWatchData],
) -> bool {
    match previous {
        Some(prev_data) => {
            if prev_data.len() != current.len() {
                return true;
            }

            // Compare each item
            prev_data.iter().zip(current.iter()).any(|(prev, curr)| {
                prev.symbol != curr.symbol
                    || prev.match_price != curr.match_price
                    || prev.change != curr.change
                    || prev.change_percent != curr.change_percent
                    || prev.volume != curr.volume
            })
        }
        None => true,
    }
}

/// Checks if stock data has changed
fn has_stock_data_changed(
    previous: &Option<Vec<SlintStockData>>,
    current: &mut [SlintStockData],
) -> bool {
    match previous {
        Some(prev_data) => {
            if prev_data.len() != current.len() {
                return true;
            }
            let mut changed = false;
            // Compare each item
            for (prev, curr) in prev_data.iter().zip(current.iter_mut()) {
                if prev.symbol != curr.symbol
                    || prev.price != curr.price
                    || prev.change != curr.change
                    || prev.change_percent != curr.change_percent
                    || prev.volume != curr.volume
                {
                    if prev.price > curr.price {
                        curr.is_changed = -1;
                    } else if prev.price < curr.price {
                        curr.is_changed = 1;
                    } else {
                        curr.is_changed = 0;
                    }
                    changed = true;
                } else {
                    curr.is_changed = 0;
                }
            }
            changed
        }
        None => true,
    }
}
