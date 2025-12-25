use crate::slint_generatedAppWindow::VolumeData;
use aim_data::aim::{PropTradingData, fetch_kqgd_nn_chart_data, fetch_kqgd_td_chart_data};

/// Dual-source task pattern macro for tasks that fetch from two different sources
/// This macro generates a task function that:
/// - Fetches data from two different API endpoints
/// - Converts each dataset using the provided conversion function
/// - Updates two different UI setters
macro_rules! create_dual_source_task {
    (
        $task_fn:ident,
        $task_id:literal,
        $task_description:literal,
        $fetch_fn1:ident,
        $fetch_fn2:ident,
        $ui_setter1:ident,
        $ui_setter2:ident,
        $data_type:ty,
        $ui_conversion:expr,
        $update_interval:literal
    ) => {
        pub async fn $task_fn(ui: &crate::AppWindow) {
            use slint::ComponentHandle;

            let ui_handle = ui.as_weak();

            tokio::spawn(async move {

                loop {
                    // Fetch and update first data source
                    match $fetch_fn1().await {
                        Ok(data1) => {
                            let ui_data1 = $ui_conversion(&data1);
                            let ui_handle_clone = ui_handle.clone();
                            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                                let model = slint::VecModel::from(ui_data1);
                                let model_rc = slint::ModelRc::new(model);
                                ui.$ui_setter1(model_rc);
                                log::info!("Updated {} data (source 1)", $task_description);
                            });
                        }
                        Err(e) => {
                            log::error!(
                                "Failed to fetch {} data (source 1): {}",
                                $task_description,
                                e
                            );
                        }
                    }

                    // Fetch and update second data source
                    match $fetch_fn2().await {
                        Ok(data2) => {
                            let ui_data2 = $ui_conversion(&data2);
                            let ui_handle_clone = ui_handle.clone();
                            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                                let model = slint::VecModel::from(ui_data2);
                                let model_rc = slint::ModelRc::new(model);
                                ui.$ui_setter2(model_rc);
                                log::info!("Updated {} data (source 2)", $task_description);
                            });
                        }
                        Err(e) => {
                            log::error!(
                                "Failed to fetch {} data (source 2): {}",
                                $task_description,
                                e
                            );
                        }
                    }

                    tokio::time::sleep(std::time::Duration::from_millis($update_interval)).await;
                }
            });
        }
    };
}

/// Converts PropTradingData to UI VolumeData format
fn convert_prop_trading_to_volume_data(api_data: &[PropTradingData]) -> Vec<VolumeData> {
    let mut result: Vec<VolumeData> = api_data
        .iter()
        .map(|item| {
            // Convert timestamp to readable date format (GMT+7)
            let time_str = {
                // Convert timestamp to GMT+7 timezone
                let date = chrono::DateTime::from_timestamp(item.trading_date, 0);
                match date {
                    Some(dt) => {
                        // Convert to GMT+7 (UTC+7)
                        let gmt_plus_7 = dt + chrono::Duration::hours(7);
                        gmt_plus_7.format("%d/%m/%Y").to_string()
                    }
                    None => "N/A".to_string(),
                }
            };

            VolumeData {
                // Convert volumes to negative for sell (as per trading volume chart convention)
                sell_volume: -(item.sell_val as f32),
                buy_volume: item.buy_val as f32,
                time: time_str.into(),
            }
        })
        .collect();

    // Reverse the order so most recent data appears first
    result.reverse();
    result
}

// Use the dual source task pattern from tasks/mod.rs
create_dual_source_task!(
    spawn_trading_volume_task,
    "dashboard.trading_volume",
    "Trading Volume Data",
    fetch_kqgd_td_chart_data,
    fetch_kqgd_nn_chart_data,
    set_td_data,
    set_nn_data,
    Vec<PropTradingData>,
    convert_prop_trading_to_volume_data,
    30000 // Update every 30 seconds for trading volume data
);
