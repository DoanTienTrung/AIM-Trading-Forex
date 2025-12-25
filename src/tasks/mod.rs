use crate::slint_generatedAppWindow;
use aim_chart::Chart;
use aim_data::explorer::vci::OrderList;
pub use cache_storage::spawn_cache_storage_task;
pub use chart::*;
pub use dashboard::*;
pub use market_watch::*;
pub use quantitative::*;
use slint_generatedAppWindow::{
    MarketWatchData as SlintMarketWatchData, StockData as SlintStockData,
};
use std::{fs::File, io::Write, path::PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
// MP layout
pub use mp::{spawn_rsi_task,register_rsi_sort_handler};
// forex
pub use forex::spawn_forex_task;

pub mod backend;
pub mod cache_storage;
pub mod chart;
pub mod dashboard;
pub mod market_watch;
pub mod quantitative;
pub mod task_manager;
pub mod world_index;
pub mod mp;
pub mod forex;

/// Simplified macro for non-stock-specific tasks (dashboard, market watch, etc.)
#[macro_export]
macro_rules! create_simple_task {
    (
        $task_fn:ident,
        $task_id:literal,
        $task_description:literal,
        $fetch_fn:ident,
        $ui_setter:ident,
        $data_type:ty,
        $ui_conversion:expr,
        $update_interval:literal
    ) => {
        pub async fn $task_fn(ui: &$crate::AppWindow) -> $crate::tasks::task_manager::TaskHandle {
            use slint::ComponentHandle;
            use $crate::tasks::task_manager::{register_task, TaskStatus};

            let ui_handle = ui.as_weak();
            let (tx, mut rx) = tokio::sync::mpsc::channel(10);
            let task_handle =
                register_task($task_id.to_string(), tx, $task_description.to_string()).await;

            tokio::spawn(async move {
                let mut task_status = TaskStatus::Running;

                loop {
                    // Check for task status updates
                    if let Ok(status) = rx.try_recv() {
                        if task_status != status {
                            log::info!(
                                "{} task status changed to: {:?}",
                                $task_description,
                                status
                            );
                            task_status = status;
                        }
                    }

                    // Skip processing if not running
                    if task_status != TaskStatus::Running {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        continue;
                    }

                    // Fetch and update data
                    match $fetch_fn().await {
                        Ok(data) => {
                            let ui_handle_clone = ui_handle.clone();
                            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                                let ui_data = $ui_conversion(data);
                                ui.$ui_setter(ui_data);
                                log::info!("Updated {} data", $task_description);
                            });
                        }
                        Err(e) => {
                            log::error!("Failed to fetch {} data: {}", $task_description, e);
                        }
                    }

                    tokio::time::sleep(std::time::Duration::from_millis($update_interval)).await;
                }
            });

            task_handle
        }
    };
}

#[repr(C)]
pub struct ChartMetaData {
    data: Vec<Chart>,
}

impl ChartMetaData {
    pub fn new(data: Vec<Chart>) -> Self {
        Self { data }
    }

    pub fn save(&self, mut file: File) {
        const VERSION: u32 = 1;

        let mut bytes = Vec::new();
        // Write version header
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        let count = self.data.len() as u32;
        bytes.extend_from_slice(&count.to_le_bytes());
        for (i, chart) in self.data.iter().enumerate() {
            let start_len = bytes.len();
            chart.write_to_bytes(&mut bytes);
            let written = bytes.len() - start_len;
            if written == 0 {
                log::error!("Chart #{} failed to serialize: {}", i, chart.stock_name);
            }
        }
        if let Err(e) = file.write_all(&bytes) {
            log::error!("Failed to write chart data to cache file: {e}");
        }
    }

    // Load charts from a file (manual deserialization, no external crate)
    pub fn load(path: &PathBuf) -> Self {
        let mut data = Vec::new();
        match std::fs::read(path) {
            Ok(bytes) => {
                let mut pos = 0;
                if bytes.len() < 8 {
                    log::error!("File too small to contain version and chart count: {path:?}");
                    return Self { data };
                }
                let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                pos += 4;
                if version != 1 {
                    log::error!("Unsupported chart cache version {version} in {path:?}");
                    return Self { data };
                }
                let count = u32::from_le_bytes([
                    bytes[pos],
                    bytes[pos + 1],
                    bytes[pos + 2],
                    bytes[pos + 3],
                ]) as usize;
                pos += 4;
                for i in 0..count {
                    let chart_bytes = &bytes[pos..];
                    match Chart::read_from_bytes(chart_bytes) {
                        Some((chart, used)) => {
                            if used == 0 {
                                log::error!(
                                    "Chart #{i} deserialized 0 bytes at pos {pos} in {path:?}"
                                );
                                break;
                            }
                            data.push(chart);
                            pos += used;
                        }
                        None => {
                            log::error!(
                                "Failed to deserialize chart #{i} at pos {pos} in {path:?}"
                            );
                            break;
                        }
                    }
                }
                log::info!("Loaded {} charts from {}", data.len(), path.display());
                for chart in &data {
                    log::info!("Chart loaded: {}", chart.stock_name);
                }
            }
            Err(e) => {
                log::error!("Failed to read chart file {path:?}: {e}");
            }
        }

        log::error!(
            "Chart data loaded from {}: {} charts",
            path.display(),
            data.len()
        );
        for chart in &data {
            log::info!("Chart loaded: {}", chart.stock_name);
        }
        Self { data }
    }

    // Get a simple hash of the chart data (no external crate)
    pub fn get_md5(&self) -> String {
        // Use a simple FNV-1a hash for demonstration
        let mut hash: u64 = 0xcbf29ce484222325;
        for chart in &self.data {
            let bytes = chart.to_bytes();
            for b in bytes {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        format!("{hash:016x}")
    }
}

pub enum DataUpdate {
    MarketWatchData(Vec<SlintMarketWatchData>),
    StockData(Vec<SlintStockData>),
    OrdList(OrderList),
    CustomList(Vec<String>),
}

#[derive(Clone, Debug)]
pub struct ReturnRowPure {
    pub ticker: String,
    pub values: Vec<f32>,
}

// Return matrix cache key
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ReturnMatrixKey {
    pub period: String,
    pub group: String,
    pub month: String,
}

// Return matrix cache value
#[derive(Clone, Debug)]
pub struct ReturnMatrixCache {
    pub rows: Vec<ReturnRowPure>,
    pub columns: Vec<String>,
}

// Global cache for return matrix
pub type ReturnMatrixCacheMap = Arc<RwLock<HashMap<ReturnMatrixKey, ReturnMatrixCache>>>;

// Spawn task to preload all return matrix combinations
pub async fn spawn_return_matrix_cache_task() -> ReturnMatrixCacheMap {
    let cache: ReturnMatrixCacheMap = Arc::new(RwLock::new(HashMap::new()));
    let cache_clone = Arc::clone(&cache);

    tokio::spawn(async move {
        let periods = vec!["daily", "weekly", "monthly", "quarterly"];
        let groups = vec!["VNINDEX", "HNXINDEX", "UPCOM", "VN30", "HNX30"];
        let months = vec![
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];

        log::info!("Starting return matrix cache preload...");

        for period in &periods {
            for group in &groups {
                if *period == "daily" {
                    // For daily, load all months
                    for month in &months {
                        let key = ReturnMatrixKey {
                            period: period.to_string(),
                            group: group.to_string(),
                            month: month.to_string(),
                        };

                        match aim_data::aim::fetch_return_matrix(period, group).await {
                            Ok(data) => {
                                let (rows, columns) = parse_return_matrix(&data, period, month);
                                let cache_value = ReturnMatrixCache { rows, columns };
                                cache_clone.write().await.insert(key.clone(), cache_value);
                                log::info!("Cached return matrix: {}_{} ({})", period, group, month);
                            }
                            Err(e) => {
                                log::error!("Failed to cache return matrix {}_{} ({}): {:?}", period, group, month, e);
                            }
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                } else {
                    // For non-daily periods, use empty month
                    let key = ReturnMatrixKey {
                        period: period.to_string(),
                        group: group.to_string(),
                        month: String::new(),
                    };

                    match aim_data::aim::fetch_return_matrix(period, group).await {
                        Ok(data) => {
                            let (rows, columns) = parse_return_matrix(&data, period, "");
                            let cache_value = ReturnMatrixCache { rows, columns };
                            cache_clone.write().await.insert(key.clone(), cache_value);
                            log::info!("Cached return matrix: {}_{}", period, group);
                        }
                        Err(e) => {
                            log::error!("Failed to cache return matrix {}_{}: {:?}", period, group, e);
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        }

        log::info!("Return matrix cache preload completed!");
    });

    cache
}

// Get cached return matrix data
pub async fn get_cached_return_matrix(
    cache: &ReturnMatrixCacheMap,
    period: &str,
    group: &str,
    month: &str,
) -> Option<ReturnMatrixCache> {
    let key = ReturnMatrixKey {
        period: period.to_string(),
        group: group.to_string(),
        month: if period == "daily" { month.to_string() } else { String::new() },
    };
    cache.read().await.get(&key).cloned()
}

/// Parse return matrix API data into rows based on period
/// API data format: { id, type, symbol, value_pct, date }
/// Returns: (rows, columns) where rows contain year/time and values for each period
/// Example for monthly: time="2025", values=[Jan, Feb, Mar, ..., Dec]
/// Example for quarterly: time="2025", values=[Q1, Q2, Q3, Q4]
pub fn parse_return_matrix(
    api_data: &[aim_data::aim::ReturnMatrixAPI],
    period: &str, month: &str,
) -> (Vec<ReturnRowPure>, Vec<String>) {
    use std::collections::{BTreeMap};

    // Helper function to calculate day of year
    fn day_of_year(day: u32, month: u32, year: u32) -> u32 {
        let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        
        // Check for leap year
        if is_leap_year(year) {
            days_in_month[1] = 29;
        }
        
        let mut day_count = day;
        for i in 0..(month - 1) as usize {
            day_count += days_in_month[i];
        }
        day_count
    }
    
    // Helper function to check if year is leap year
    fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    // Parse month filter for daily period
    let month_filter: Option<u32> = if period == "daily" {
        match month {
            "1" | "January" => Some(1),
            "2" | "February" => Some(2),
            "3" | "March" => Some(3),
            "4" | "April" => Some(4),
            "5" | "May" => Some(5),
            "6" | "June" => Some(6),
            "7" | "July" => Some(7),
            "8" | "August" => Some(8),
            "9" | "September" => Some(9),
            "10" | "October" => Some(10),
            "11" | "November" => Some(11),
            "12" | "December" => Some(12),
            _ => Some(1), // Default to January
        }
    } else {
        None
    };

    // Parse dates and extract year and period index
    let mut year_data: BTreeMap<String, BTreeMap<usize, f32>> = BTreeMap::new();
    
    for entry in api_data {
        let date = entry.date.clone(); // format: "31/12/2024"
        let value = entry.value_pct as f32;

        // Parse date: "31/12/2024" -> day=31, month=12, year=2024
        if let Some((day_str, rest)) = date.split_once('/') {
            if let Some((month_str, year_str)) = rest.split_once('/') {
                let day: u32 = day_str.parse().unwrap_or(1);
                let month_num: u32 = month_str.parse().unwrap_or(1);
                let year_num: u32 = year_str.parse().unwrap_or(2024);
                let year = year_str.to_string();
                
                // For daily period, filter by selected month
                if period == "daily" {
                    if let Some(filter_month) = month_filter {
                        if month_num != filter_month {
                            continue;
                        }
                    }
                }
                
                let period_index = match period {
                    "quarterly" => {
                        // Month 1-3 -> Q1 (index 0), 4-6 -> Q2 (index 1), etc.
                        ((month_num - 1) / 3) as usize
                    }
                    "monthly" => {
                        // Month 1-12 -> index 0-11
                        (month_num - 1) as usize
                    }
                    "weekly" => {
                        // Calculate week number based on day of year
                        // Day 1-7 -> Week 0, Day 8-14 -> Week 1, etc.
                        let day_num = day_of_year(day, month_num, year_num);
                        ((day_num - 1) / 7) as usize
                    }
                    "daily" => {
                        // For daily period, use day of month as index (1-31 -> 0-30)
                        (day - 1) as usize
                    }
                    _ => (month_num - 1) as usize,
                };

                year_data
                    .entry(year)
                    .or_insert_with(BTreeMap::new)
                    .insert(period_index, (value*100.0).round()/100.0); // Round to 2 decimal places
            }
        }
    }

    // Build columns based on period type
    let columns: Vec<String> = match period {
        "quarterly" => vec!["Q1".to_string(), "Q2".to_string(), "Q3".to_string(), "Q4".to_string()],
        "monthly" => vec![
            "January".to_string(), "February".to_string(), "March".to_string(),
            "April".to_string(), "May".to_string(), "June".to_string(),
            "July".to_string(), "August".to_string(), "September".to_string(),
            "October".to_string(), "November".to_string(), "December".to_string()
        ],
        "weekly" => (1..=52).map(|w| format!("Week {}", w)).collect(),
        "daily" => {
            // Get number of days in the selected month
            let days_in_month = match month_filter {
                Some(2) => if is_leap_year(2024) { 29 } else { 28 }, // Use current year or default
                Some(4) | Some(6) | Some(9) | Some(11) => 30,
                _ => 31, // Months 1, 3, 5, 7, 8, 10, 12
            };
            (1..=days_in_month).map(|d| format!("{}", d)).collect()
        },
        _ => vec![],
    };

    let num_periods = columns.len();

    // Build rows: each row is a year with values for each period
    let mut rows: Vec<ReturnRowPure> = year_data
        .into_iter()
        .map(|(year, period_values)| {
            let values: Vec<f32> = (0..num_periods)
                .map(|i| *period_values.get(&i).unwrap_or(&0.0))
                .collect();
            ReturnRowPure {
                ticker: year, // ticker field now holds the year
                values,
            }
        })
        .collect();

    // Sort rows by year descending (newest first)
    rows.sort_by(|a, b| b.ticker.cmp(&a.ticker));

    // Calculate average row
    if !rows.is_empty() {
        let mut avg_values = vec![0.0f32; num_periods];
        let count = rows.len() as f32;
        
        for row in &rows {
            for (i, &val) in row.values.iter().enumerate() {
                avg_values[i] += val;
            }
        }
        
        for val in &mut avg_values {
            *val /= count;
            *val = (*val * 100.0).round() / 100.0; // Round to 2 decimal places
        }
        
        rows.push(ReturnRowPure {
            ticker: "Average".to_string(),
            values: avg_values,
        });
    }

    (rows, columns)
}
