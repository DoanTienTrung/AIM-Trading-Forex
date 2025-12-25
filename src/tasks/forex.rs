use crate::tasks::task_manager::{register_task, TaskStatus, TaskHandle};
use crate::slint_generatedAppWindow::ForexData as SlintForexData;
use crate::AppWindow;
use aim_data::aim::{fetch_forex_data, ForexRecord};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use std::sync::Arc;
use tokio::sync::Mutex;


// Convert API ForexRecord -> Slint ForexData
fn convert_to_forex_data(record: &ForexRecord) -> SlintForexData {
    // Format timestamp (i64) -> date string
    let datetime = chrono::DateTime::from_timestamp(record.trade_time, 0)
        .unwrap_or_else(|| chrono::Utc::now().into());
    let time_str = datetime.format("%d/%m/%y").to_string();

    SlintForexData {
        symbol: record.symbol.clone().into(),
        name: record.symbol_name.clone().into(),
        last: format!("{:.4}", record.last_price).into(),
        change: format!("{:.3}", record.price_change).into(),
        percent_change: format!("{:.2}%", record.percent_change * 100.0).into(),
        open: format!("{:.4}", record.open_price).into(),
        high: format!("{:.4}", record.high_price).into(),
        low: format!("{:.4}", record.low_price).into(),
        time: time_str.into(),
        is_up: record.price_change > 0.0,
    }
}

pub async fn spawn_forex_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.quantitative.forex".to_string(),
        tx,
        "Forex Data Fetcher".to_string(),
    ).await;

    // Shared state for selected group
    let selected_group = Arc::new(Mutex::new("USD".to_string()));
    let search_text = Arc::new(Mutex::new("".to_string()));

    // Register callback handler
    register_forex_group_handler(ui, Arc::clone(&selected_group));
   

    // Clone Arc để move vào async closure
    let selected_group_clone = Arc::clone(&selected_group);
   

    tokio::spawn(async move {
        let mut task_status = TaskStatus::Running;

        loop {
            // Check status
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("Forex task status changed to: {:?}", status);
                    task_status = status;
                }
            }

            if task_status != TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            // Get current selected group
            let group = {
                let g = selected_group_clone.lock().await;
                g.clone()
            };

            // Fetch data for selected group
            match fetch_forex_data(&group).await {
                Ok(response) => {
                    
                    // Filter by symbol prefix (e.g., ^USD, ^EUR, ^VND)
                    let group_filter = format!("forex.rates.{}", group.to_uppercase());
                    let filtered: Vec<&ForexRecord> = response
                        .iter()
                        .filter(|record| record.group_name == group_filter)
                        .collect();

                    log::info!("Fetched {} forex records for group {} (total: {}, prefix: {})",
                        filtered.len(), group, response.len(), group_filter);

                    let forex_data: Vec<SlintForexData> = filtered
                        .iter()
                        .map(|record| convert_to_forex_data(record))
                        .collect();

                    // Update UI
                    let group_clone = group.clone();
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        ui.set_forex_list(ModelRc::new(VecModel::from(forex_data)));
                        ui.set_selected_groups(group_clone.into());
                    });
                }
                Err(e) => {
                    log::error!("Failed to fetch forex data for {}: {}", group, e);
                }
            }

            // Refresh every 5 seconds
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    task_handle
}


// Register callback handler for forex group selection
pub fn register_forex_group_handler(ui: &AppWindow, selected_group: Arc<Mutex<String>>) {
    log::info!("Registering forex group selection handler");
    
    ui.on_on_group_selected(move |group: slint::SharedString| {
        let selected_group = Arc::clone(&selected_group);
        let group_str = group.to_string();
        log::info!("Forex group selected: {}", group_str);
        
        tokio::spawn(async move {
            let mut g = selected_group.lock().await;
            *g = group_str;
        });
    });
}

