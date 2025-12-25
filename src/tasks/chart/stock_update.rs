use crate::interval_to_constant;
use crate::slint_generatedAppWindow::AppWindow;
use crate::tasks::task_manager::{register_task, TaskHandle};
use crate::tasks::ChartMetaData;
use aim_chart::Chart;
use aim_chart::CompanyInfo;
use aim_data::get_company_info;
use aim_data::get_quote;
use slint::ComponentHandle;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

/// Spawns a task to handle stock and timeframe updates
pub async fn spawn_stock_update_task(
    chart: Arc<Mutex<ChartMetaData>>,
    ui: &AppWindow,
) -> Vec<TaskHandle> {
    let chart_clone = Arc::clone(&chart);

    let mut handles = Vec::new();
    handles.push(spawn_current_stock_data_task(chart_clone.clone(), ui).await);
    handles.push(spawn_new_stock_data_task(chart, ui).await);
    handles
}

pub async fn spawn_new_stock_data_task(
    chart: Arc<Mutex<ChartMetaData>>,
    ui: &AppWindow,
) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let chart_clone = Arc::clone(&chart);
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.stock_update.new_stock".to_string(),
        tx,
        "New Stock Data Task".to_string(),
    )
    .await;

    tokio::spawn(async move {
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

            let chart_clone = Arc::clone(&chart_clone);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let mut ui_data = ui.get_ui_data();

                // Handle new stock or timeframe selection
                if ui_data.is_new_stock || ui_data.is_new_time_frame {
                    ui.set_is_chart_in_update(true);
                    ui_data.is_new_stock = false;
                    ui_data.is_new_time_frame = false;
                    let time_frame = ui_data.clone().time_frame;
                    ui.set_ui_data(ui_data);
                    let stock = ui.get_current_stock().symbol;

                    // Fetch and update chart data for new selection
                    let chart_clone = Arc::clone(&chart_clone);
                    let ui_handle = ui.as_weak();
                    tokio::spawn(async move {
                        // Check if chart with the same stock name already exists first
                        let chart_exists = {
                            let charts = chart_clone.lock().await;
                            charts
                                .data
                                .iter()
                                .any(|chart| chart.stock_name == stock.to_uppercase())
                        };

                        // If chart exists, just return without creating a new one
                        if chart_exists {
                            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                                ui.set_is_chart_in_update(false);
                            });
                            return;
                        }

                        let stock_data = if let Ok(data) =
                            get_quote(&[&stock], interval_to_constant(&time_frame), None, None)
                                .await
                        {
                            data
                        } else {
                            log::error!("Failed to fetch stock data for {stock}");
                            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                                ui.set_is_chart_in_update(false);
                            });
                            return;
                        };

                        if !stock_data.0.is_empty() {
                            let company_info =
                                if let Ok(stock_info) = get_company_info(&stock).await {
                                    CompanyInfo {
                                        roe: stock_info.data.company_financial_ratio.ratio[0]
                                            .roe
                                            .unwrap_or(0.0),
                                        roa: stock_info.data.company_financial_ratio.ratio[0]
                                            .roa
                                            .unwrap_or(0.0),
                                        pe: stock_info.data.company_financial_ratio.ratio[0]
                                            .pe
                                            .unwrap_or(0.0),
                                        pb: stock_info.data.company_financial_ratio.ratio[0]
                                            .pb
                                            .unwrap_or(0.0),
                                        eps: stock_info.data.company_financial_ratio.ratio[0]
                                            .eps
                                            .unwrap_or(0.0),
                                    }
                                } else {
                                    CompanyInfo::default()
                                };
                            let mut charts = chart_clone.lock().await;
                            charts.data.push(Chart::new_default(
                                stock.to_uppercase(),
                                stock_data.0[0].clone(),
                                company_info,
                            ));
                        }
                        let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                            ui.set_is_chart_in_update(false);
                        });
                    });
                }
            });
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    task_handle
}

/// Spawns a separate task to handle chart data updates for existing charts
pub async fn spawn_current_stock_data_task(
    chart: Arc<Mutex<ChartMetaData>>,
    ui: &AppWindow,
) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let chart_clone = Arc::clone(&chart);
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.stock_update.current_stock".to_string(),
        tx,
        "Current Stock Data Task".to_string(),
    )
    .await;

    // Task 2: Update existing charts with latest data
    tokio::spawn(async move {
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

            // if !super::is_trading_hours() {
            //     log::info!("Outside trading hours (9:00-15:00 Vietnam time), skipping data update");
            //     tokio::time::sleep(Duration::from_millis(500)).await;
            //     continue;
            // }
            let chart_clone = Arc::clone(&chart_clone);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let stock = ui.get_current_stock().symbol;

                tokio::spawn(async move {
                    // Check if chart with the same stock name already exists
                    let chart_exists = {
                        let charts = chart_clone.lock().await;
                        charts
                            .data
                            .iter()
                            .any(|chart| chart.stock_name == stock.to_uppercase())
                    };

                    // If chart exists, update it with latest data
                    if chart_exists {
                        if let Ok(chart_data_vec) =
                            get_quote(&[&stock], "ONE_DAY", None, None).await
                        {
                            let mut charts = chart_clone.lock().await;

                            // Update chart data for each tracked stock
                            for chart in charts.data.iter_mut() {
                                if let Some(updated_data) = chart_data_vec
                                    .0
                                    .iter()
                                    .find(|data| data.symbol == chart.stock_name)
                                {
                                    chart.update_candle_data(updated_data.clone());
                                }
                            }
                        }
                        if let Ok(stock_info) = get_company_info(&stock).await {
                            let company_info = CompanyInfo {
                                roe: stock_info.data.company_financial_ratio.ratio[0]
                                    .roe
                                    .unwrap_or(0.0),
                                roa: stock_info.data.company_financial_ratio.ratio[0]
                                    .roa
                                    .unwrap_or(0.0),
                                pe: stock_info.data.company_financial_ratio.ratio[0]
                                    .pe
                                    .unwrap_or(0.0),
                                pb: stock_info.data.company_financial_ratio.ratio[0]
                                    .pb
                                    .unwrap_or(0.0),
                                eps: stock_info.data.company_financial_ratio.ratio[0]
                                    .eps
                                    .unwrap_or(0.0),
                            };

                            let mut charts = chart_clone.lock().await;

                            // Update chart data for each tracked stock
                            for chart in charts.data.iter_mut() {
                                if stock.to_uppercase() == chart.stock_name {
                                    chart.update_company_info(company_info.clone());
                                }
                            }
                        }
                    }
                });
            });
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    task_handle
}
