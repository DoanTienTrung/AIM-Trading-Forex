use crate::create_simple_task;
use aim_data::explorer::aim::{fetch_icb_index_data_filtered, fetch_top_10_market_cap_data, IcbIndex, Top10MarketCap};
use slint::ComponentHandle;
use crate::tasks::task_manager::{register_task, TaskStatus};

// Placeholder conversion function - will be used when UI properties are ready
#[allow(dead_code)]
fn convert_top10_market_cap_to_ui(api_data: Vec<Top10MarketCap>) -> Vec<slint::SharedString> {
    api_data
        .iter()
        .map(|item| format!("{}: {} ({}%)", item.stock_code, item.market_cap, item.change_percent).into())
        .collect()
}

/// Task to fetch Top 10 Market Cap data
pub async fn spawn_top10_market_cap_task(ui: &crate::AppWindow) -> crate::tasks::task_manager::TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "quantitative.top10_market_cap".to_string(),
        tx,
        "Top 10 Market Cap Data".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = TaskStatus::Running;

        loop {
            // Check for task status updates
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("Top 10 Market Cap task status changed to: {:?}", status);
                    task_status = status;
                }
            }

            // Skip processing if not running
            if task_status != TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            // Fetch and log Top 10 Market Cap data
            match fetch_top_10_market_cap_data().await {
                Ok(data) => {
                    log::info!("✅ Fetched Top 10 Market Cap data: {} items", data.len());
                    for item in &data {
                        log::info!(
                            "  - {}: Market Cap={}, Price={}, Change={}%",
                            item.stock_code, item.market_cap, item.price, item.change_percent
                        );
                    }
                    // TODO: Update UI when UI properties are ready
                    // let ui_handle_clone = ui_handle.clone();
                    // let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                    //     // Update UI properties here
                    // });
                }
                Err(e) => {
                    log::error!("❌ Failed to fetch Top 10 Market Cap data: {}", e);
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(60000)).await; // Update every 60 seconds
        }
    });

    task_handle
}

/// Task to fetch ICB Index filtered data (industry_code 2 digits)
pub async fn spawn_icb_index_filtered_task(ui: &crate::AppWindow) -> crate::tasks::task_manager::TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "quantitative.icb_index_filtered".to_string(),
        tx,
        "ICB Index Filtered Data".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = TaskStatus::Running;

        loop {
            // Check for task status updates
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("ICB Index Filtered task status changed to: {:?}", status);
                    task_status = status;
                }
            }

            // Skip processing if not running
            if task_status != TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            // Fetch and calculate totals
            match fetch_icb_index_data_filtered().await {
                Ok(data) => {
                    let total_market_cap: i64 = data.iter().map(|x| x.market_cap).sum();
                    let total_volume: i64 = data.iter().map(|x| x.volume).sum();
                    let total_value: i64 = data.iter().map(|x| x.value).sum();
                    
                    log::info!("✅ Fetched ICB Index Filtered data: {} items", data.len());
                    log::info!("  - Total Market Cap: {} (tỷ VND)", total_market_cap as f64 / 1_000_000_000.0);
                    log::info!("  - Total Volume: {}", total_volume);
                    log::info!("  - Total Value: {} (tỷ VND)", total_value as f64 / 1_000_000_000.0);
                    
                    // Log first few items for debugging
                    for (i, item) in data.iter().take(5).enumerate() {
                        log::info!(
                            "  [{i}] {} ({}): Market Cap={}, Volume={}, Value={}",
                            item.icb_name, item.industry_code, item.market_cap, item.volume, item.value
                        );
                    }
                    
                    // TODO: Update UI when UI properties are ready
                    // let ui_handle_clone = ui_handle.clone();
                    // let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                    //     // Update UI properties here (overview.total_market_cap, etc.)
                    // });
                }
                Err(e) => {
                    log::error!("❌ Failed to fetch ICB Index Filtered data: {}", e);
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(3600000)).await; // Update every hour (3600 seconds)
        }
    });

    task_handle
}

