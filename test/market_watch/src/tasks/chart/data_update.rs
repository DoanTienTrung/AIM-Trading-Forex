use super::convert_to_market_data;
use crate::aim_data::get_market_watch;
use crate::slint_generatedAppWindow::{
    MarketWatchData as SlintMarketWatchData
};
use crate::tasks::chart::sort_market_watch;
use crate::tasks::DataUpdate;
use slint::Weak;
use slint::{ComponentHandle, ModelRc};
use std::time::Duration;
use tokio::sync::mpsc;

use super::VN30_LIST;

/// Spawns a task to handle market watch data updates
pub fn spawn_data_update_task(
    ui: &crate::slint_generatedAppWindow::AppWindow,
) {
    let (tx_data_update, rx_data_update) = mpsc::channel::<DataUpdate>(10);
    let ui_handle = ui.as_weak();
    tokio::spawn(polling_market_watch(tx_data_update.clone()));
    tokio::spawn(update_ui_with_data(ui_handle, rx_data_update));
}

async fn polling_market_watch(tx: mpsc::Sender<DataUpdate>) {
    let mut previous_market_watch_data: Option<Vec<SlintMarketWatchData>> = None;
    let mut is_first_update = true; // Track if this is the first update
    loop {
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
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn update_ui_with_data(
    ui_handle: Weak<crate::slint_generatedAppWindow::AppWindow>,
    mut rx: mpsc::Receiver<DataUpdate>,
) {
    loop {
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
                _ => {
                    // nothing to do
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
