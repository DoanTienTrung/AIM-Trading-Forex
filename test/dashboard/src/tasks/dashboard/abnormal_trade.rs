use crate::create_simple_task;
use crate::slint_generatedAppWindow::AbnormalTradeData;
use aim_data::aim::{AbnormalTrade, fetch_abnormal_trade_data};
use slint::{ModelRc, VecModel};

fn convert_abnormal_trade_to_ui_data(api_data: Vec<AbnormalTrade>) -> ModelRc<AbnormalTradeData> {
    let ui_data: Vec<AbnormalTradeData> = api_data
        .iter()
        .map(|item| {
            // Convert timestamp (seconds) to readable time format HH:MM:SS in GMT+7
            let time_str = if item.timestamp > 0 {
                // Add 7 hours (25200 seconds) for GMT+7 timezone
                let gmt_plus_7_timestamp = item.timestamp + 25200;
                let seconds = gmt_plus_7_timestamp % 86400; // Get seconds within the day
                let hours = seconds / 3600;
                let minutes = (seconds % 3600) / 60;
                let secs = seconds % 60;
                format!("{:02}:{:02}:{:02}", hours, minutes, secs)
            } else {
                "00:00:00".to_string()
            };

            AbnormalTradeData {
                symbol: item.ticker.clone().into(),
                r#type: match item.match_type.as_str() {
                    "s" => "Sell".to_string().into(),
                    "b" => "Buy".to_string().into(),
                    _ => item.match_type.clone().into(),
                },
                volume: item.volume as i32,
                price: (item.price as f32) / 1000.0,
                time: time_str.into(),
            }
        })
        .collect();

    let model = VecModel::from(ui_data);
    ModelRc::new(model)
}

// Use the generic simple task pattern from tasks/mod.rs
create_simple_task!(
    spawn_abnormal_trade_task,
    "dashboard.abnormal_trade",
    "Abnormal Trade Data",
    fetch_abnormal_trade_data,
    set_abnormal_trade_data,
    Vec<AbnormalTrade>,
    convert_abnormal_trade_to_ui_data,
    5000 // Update every 5 seconds for abnormal trade data
);
