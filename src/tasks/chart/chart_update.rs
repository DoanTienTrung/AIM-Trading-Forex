use crate::tasks::task_manager::{register_task, TaskHandle};
use crate::tasks::{chart::is_trading_hours, ChartMetaData};
use aim_data::get_quote;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

/// Spawns a task to handle real-time stock data updates
pub async fn spawn_chart_update_task(chart: Arc<Mutex<ChartMetaData>>) -> TaskHandle {
    let chart_clone = Arc::clone(&chart);
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.chart_update".to_string(),
        tx,
        "Chart Data Update Task".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
        let mut stock_names: Vec<String> = vec!["AAA".to_string()];
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
            if !is_trading_hours() {
                log::info!("Outside trading hours (9:00-15:00 Vietnam time), skipping data update");
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }
            // Fetch updated chart data for all tracked stocks
            let stock_name_slices: Vec<&str> = stock_names.iter().map(|s| s.as_str()).collect();
            if let Ok(chart_data_vec) = get_quote(&stock_name_slices, "ONE_DAY", None, None).await {
                let mut charts = chart_clone.lock().await;
                // Update list of tracked stocks
                let new_stock_names: Vec<String> = charts
                    .data
                    .iter()
                    .map(|chart| chart.stock_name.clone())
                    .collect();
                stock_names = new_stock_names.clone();

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
            tokio::time::sleep(Duration::from_millis(5000)).await;
        }
    });

    task_handle
}
