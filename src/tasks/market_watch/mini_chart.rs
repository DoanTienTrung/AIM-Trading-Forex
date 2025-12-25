use crate::slint_generatedAppWindow::AppWindow;
use aim_chart::convert_candlesticks;
use aim_chart::mini_chart::mini_chart_render;
use aim_data::get_quote;
use chrono::{Datelike, Duration, Local, NaiveDate, Utc, Weekday};
use slint::{ComponentHandle, Model};

/// Mini chart task pattern macro for market watch mini charts
/// This macro generates a task function that spawns a chart loop for a specific symbol
macro_rules! create_mini_chart_task {
    (
        $task_fn:ident,
        $task_id:literal,
        $task_description:literal,
        $symbol:literal,
        $ui_type:literal
    ) => {
        pub async fn $task_fn(ui: &crate::AppWindow) -> crate::tasks::task_manager::TaskHandle {
            use crate::tasks::task_manager::register_task;

            let ui_handle = ui.as_weak();
            let (tx, rx) = tokio::sync::mpsc::channel(10);
            let task_handle =
                register_task($task_id.to_string(), tx, $task_description.to_string()).await;

            tokio::spawn(async move {
                println!("Mini chart {} task started!", $symbol);
                crate::tasks::market_watch::mini_chart::spawn_chart_loop(
                    ui_handle, $symbol, $ui_type, rx,
                )
                .await;
            });

            task_handle
        }
    };
}
// Unused imports removed

async fn get_reference_price(symbol: &str, current_date: NaiveDate) -> f32 {
    // Try to get the previous trading day's close price
    let mut prev_date = current_date - Duration::days(1);
    let mut attempts = 0;

    // Try up to 7 days back to find the previous trading day
    while attempts < 7 {
        // Skip weekends
        if prev_date.weekday() != Weekday::Sat && prev_date.weekday() != Weekday::Sun {
            let day_start = prev_date
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Local)
                .unwrap()
                .with_timezone(&Utc);
            let day_end = prev_date
                .and_hms_opt(23, 59, 59)
                .unwrap()
                .and_local_timezone(Local)
                .unwrap()
                .with_timezone(&Utc);

            println!(
                "Mini chart {}: Fetching reference price from {}...",
                symbol,
                prev_date.format("%Y-%m-%d")
            );
            match get_quote(&[symbol], "ONE_MINUTE", Some(day_start), Some(day_end)).await {
                Ok(resp) => {
                    if !resp.0.is_empty() {
                        if let Some(ohlc) = resp.0.first() {
                            let candlesticks = ohlc.to_candlesticks();
                            if !candlesticks.is_empty() {
                                let last_close = candlesticks.last().unwrap().close as f32; // Convert to display format
                                println!(
                                    "Mini chart {}: Got reference price {:.2} from {}",
                                    symbol,
                                    last_close,
                                    prev_date.format("%Y-%m-%d")
                                );
                                return last_close;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "Mini chart {}: Failed to get reference price from {}: {}",
                        symbol,
                        prev_date.format("%Y-%m-%d"),
                        e
                    );
                }
            }
        }

        prev_date -= Duration::days(1);
        attempts += 1;
    }

    // Default reference price if no previous data found
    let default_price = match symbol {
        "VNINDEX" => 1600.0,
        "VN30" => 1500.0,
        "HNX30" => 500.0,
        "HNXIndex" => 250.0,
        _ => 1000.0,
    };
    println!("Mini chart {symbol}: No reference price found, using default {default_price:.2}");
    default_price
}

// Use the mini chart task pattern from tasks/mod.rs
create_mini_chart_task!(
    spawn_mini_chart_vnindex_task,
    "market_watch.mini_chart.vnindex",
    "VNINDEX Mini Chart Task",
    "VNINDEX",
    "vnindex"
);

create_mini_chart_task!(
    spawn_mini_chart_vn30_task,
    "market_watch.mini_chart.vn30",
    "VN30 Mini Chart Task",
    "VN30",
    "vn30"
);

create_mini_chart_task!(
    spawn_mini_chart_hnx30_task,
    "market_watch.mini_chart.hnx30",
    "HNX30 Mini Chart Task",
    "HNX30",
    "hnx30"
);

create_mini_chart_task!(
    spawn_mini_chart_hnxindex_task,
    "market_watch.mini_chart.hnxindex",
    "HNXIndex Mini Chart Task",
    "HNXIndex",
    "hnxindex"
);

pub async fn spawn_chart_loop(
    ui_handle: slint::Weak<AppWindow>,
    symbol: &str,
    ui_type: &str,
    mut rx: tokio::sync::mpsc::Receiver<crate::tasks::task_manager::TaskStatus>,
) {
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
        // get the latest data for the given symbol (latest trading day only)
        // Get today's date for filtering, with fallback to previous trading days
        let now = Local::now();

        let mut attempt_date = now.date_naive();
        let mut data_found = false;
        let mut attempts = 0;

        // Try up to 7 days back to find trading data (skip weekends)
        while !data_found && attempts < 7 {
            // Skip weekends for Vietnamese stock market
            if attempt_date.weekday() != Weekday::Sat && attempt_date.weekday() != Weekday::Sun {
                let day_start = attempt_date
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .with_timezone(&Utc);
                let day_end = attempt_date
                    .and_hms_opt(23, 59, 59)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .with_timezone(&Utc);

                println!(
                    "Mini chart: Fetching {} data for {}...",
                    symbol,
                    attempt_date.format("%Y-%m-%d")
                );
                match get_quote(&[symbol], "ONE_MINUTE", Some(day_start), Some(day_end)).await {
                    Ok(resp) => {
                        println!(
                            "Mini chart: Got response for {} with timeframe ONE_MINUTE - {} items",
                            symbol,
                            resp.0.len()
                        );
                        if !resp.0.is_empty() {
                            if let Some(ohlc) = resp.0.first() {
                                let candlesticks = ohlc.to_candlesticks();
                                println!(
                                    "Mini chart: Got {} candlesticks for {}",
                                    candlesticks.len(),
                                    symbol
                                );
                                if !candlesticks.is_empty() {
                                    let candle_data = convert_candlesticks(false, candlesticks);

                                    // Get reference price (previous day's close price)
                                    let ref_price = get_reference_price(symbol, attempt_date).await;

                                    let symbol_clone = symbol.to_string();
                                    let ui_type_clone = ui_type.to_string();
                                    let candle_data_clone = candle_data.clone();
                                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                                        // Create the image inside the event loop to avoid thread safety issues
                                        let image = mini_chart_render(ref_price, candle_data_clone.clone());

                                        // Calculate market data from candle data
                                        let current_price = candle_data_clone.last().map(|c| c.close()).unwrap_or(0.0);
                                        let change = current_price - ref_price;
                                        let percentage = if ref_price > 0.0 { (change / ref_price) * 100.0 } else { 0.0 };

                                        // Calculate total volume from all candle data
                                        let total_volume: i64 = candle_data_clone.iter()
                                            .map(|c| c.volume() as i64)
                                            .sum();

                                        // Format number (total volume count) with commas - this goes to the number field
                                        let number_str = {
                                            let num_str = total_volume.to_string();
                                            let mut result = String::new();
                                            let chars: Vec<char> = num_str.chars().rev().collect();
                                            for (i, c) in chars.iter().enumerate() {
                                                if i > 0 && i % 3 == 0 {
                                                    result.push(',');
                                                }
                                                result.push(*c);
                                            }
                                            result.chars().rev().collect::<String>()
                                        };

                                        // Format volume string (for display) - this goes to the volume field
                                        let volume_str = if total_volume > 1_000_000 {
                                            format!("{:.1} TỶ", total_volume as f64 / 1_000_000.0)
                                        } else if total_volume > 1_000 {
                                            format!("{:.1}K", total_volume as f64 / 1_000.0)
                                        } else {
                                            total_volume.to_string()
                                        };

                                        println!("Mini chart: Updated {symbol_clone} - Price: {current_price:.2}, Change: {change:.2} ({percentage:.2}%), Total Volume: {number_str}, Volume Display: {volume_str}");

                                        // Update the index_data array with complete market data
                                        let index_data = ui.get_index_data();
                                        for i in 0..index_data.row_count() {
                                            if let Some(mut row) = index_data.row_data(i) {
                                                if row.symbol.as_str().to_lowercase() == ui_type_clone.as_str() {
                                                    row.image = image;
                                                    row.number = number_str.into(); // Total volume count
                                                    row.volume = volume_str.into();  // Formatted volume display
                                                    row.price = current_price;
                                                    row.change = change;
                                                    row.percentage = percentage;
                                                    index_data.set_row_data(i, row);
                                                    break;
                                                }
                                            }
                                        }
                                        ui.set_index_data(index_data);
                                    });
                                    data_found = true;
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!(
                            "Mini chart: Failed to get quote data for {} on {}: {}",
                            symbol,
                            attempt_date.format("%Y-%m-%d"),
                            e
                        );
                    }
                }
            }

            // Move to previous day
            attempt_date -= Duration::days(1);
            attempts += 1;
        }

        if !data_found {
            println!("Mini chart {symbol}: No trading data found in the last 7 days");
        }

        tokio::time::sleep(std::time::Duration::from_secs(60)).await; // Adjust the interval as needed
    }
}
