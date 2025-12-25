use std::sync::Arc;

use crate::tasks::task_manager::{register_task, TaskHandle};
use chrono::Datelike;
use slint::ComponentHandle;
use tokio::sync::Mutex;

use crate::tasks::backend::{convert_financial_data_to_overview, FinanceListExt};
use aim_data::explorer::aim::{
    fetch_balance_sheet_data, fetch_cash_flow_gt_sheet_data, fetch_cash_flow_tt_sheet_data,
    fetch_income_statement_sheet_data,
};
use aim_data::explorer::aim::{fetch_financial_data, FinanceSheetData};

use crate::AppWindow;

/// Finance-specific task pattern macro for tasks that need quarterly data fetching
/// This macro generates a task function that:
/// - Fetches data for multiple quarters (current and previous)
/// - Implements caching per stock symbol
/// - Handles fallback to default periods
/// - Updates UI with quarterly financial data
macro_rules! create_finance_task {
    (
        $task_fn:ident,
        $task_id:literal,
        $task_description:literal,
        $fetch_fn:ident,
        $ui_setter:ident,
        $cache_name:ident,
        $data_type:ty
    ) => {
        pub async fn $task_fn(ui: &crate::AppWindow) -> crate::tasks::task_manager::TaskHandle {
            use crate::tasks::chart::finance_sheet::{
                FinanceListExt, QuarterPeriod, DEFAULT_PERIODS,
            };
            use crate::tasks::task_manager::{register_task, TaskStatus};
            use slint::ComponentHandle;
            use std::sync::Arc;
            use tokio::sync::Mutex;

            let ui_handle = ui.as_weak();
            let (tx, mut rx) = tokio::sync::mpsc::channel(10);
            let task_handle =
                register_task($task_id.to_string(), tx, $task_description.to_string()).await;

            tokio::spawn(async move {
                let pre_stock_name = Arc::new(Mutex::new(String::from("")));
                let current_stock = Arc::new(Mutex::new(String::from("AAA")));
                let $cache_name = Arc::new(Mutex::new(std::collections::HashMap::<
                    String,
                    Vec<Vec<$data_type>>,
                >::new()));
                let mut task_status = TaskStatus::Running;

                loop {
                    if let Ok(status) = rx.try_recv() {
                        if task_status != status {
                            log::info!("Finance task status changed to: {:?}", status);
                            task_status = status;
                        }
                    }
                    if task_status != TaskStatus::Running {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        continue;
                    }

                    let current_stock_clone = Arc::clone(&current_stock);
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        let ui_current_stock = ui.get_current_stock().symbol;
                        tokio::spawn(async move {
                            let mut a = current_stock_clone.lock().await;
                            *a = ui_current_stock.to_string();
                        });
                    });

                    let stock_name = current_stock.lock().await.clone();
                    let mut data = Vec::new();

                    // Check cache first
                    {
                        let cache_guard = $cache_name.lock().await;
                        if let Some(cached_data) = cache_guard.get(&stock_name) {
                            data = cached_data.clone();
                        }
                    }

                    if !data.is_empty() {
                        let mut pre_stock = pre_stock_name.lock().await;
                        if *pre_stock != stock_name {
                            *pre_stock = stock_name.clone();
                            // Update the UI with cached data
                            let ui_handle_clone = ui_handle.clone();
                            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                                let finance_list = FinanceListExt::from_data(data);
                                ui.$ui_setter(finance_list);
                                log::info!(
                                    "Updated {} data from cache for {}",
                                    $task_description,
                                    stock_name
                                );
                            });
                        }
                    } else {
                        let mut pre_stock = pre_stock_name.lock().await;
                        *pre_stock = stock_name.clone();
                        log::info!("Fetching new {} data for {}", $task_description, stock_name);

                        let mut data_vec = Vec::new();
                        let mut current_period = QuarterPeriod::current_quarter();
                        let max_quarters = 5;
                        let mut quarters_fetched = 0;
                        let mut consecutive_empty_quarters = 0;
                        let max_consecutive_empty = 3;

                        // Fetch data for multiple quarters
                        while quarters_fetched < max_quarters
                            && consecutive_empty_quarters < max_consecutive_empty
                        {
                            let period_str = current_period.to_period();
                            match $fetch_fn(&stock_name, &period_str).await {
                                Ok(data) if !data.is_empty() => {
                                    data_vec.push(data);
                                    consecutive_empty_quarters = 0;
                                    quarters_fetched += 1;
                                    log::info!(
                                        "Successfully fetched {} data for {} - {}",
                                        $task_description,
                                        stock_name,
                                        period_str
                                    );
                                }
                                _ => {
                                    consecutive_empty_quarters += 1;
                                    log::warn!(
                                        "Failed to fetch {} data for {} - {}",
                                        $task_description,
                                        stock_name,
                                        period_str
                                    );
                                }
                            }
                            current_period = current_period.previous_quarter();
                        }

                        // Fallback to default periods if no data found
                        if data_vec.is_empty() {
                            for period in DEFAULT_PERIODS.iter() {
                                if let Ok(data) = $fetch_fn(&stock_name, period).await {
                                    if !data.is_empty() {
                                        data_vec.push(data);
                                    }
                                }
                            }
                        }

                        // Store in cache
                        $cache_name
                            .lock()
                            .await
                            .insert(stock_name.clone(), data_vec.clone());

                        // Update the UI
                        let ui_handle_clone = ui_handle.clone();
                        let stock_name_clone = stock_name.clone();
                        let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                            let finance_list = FinanceListExt::from_data(data_vec);
                            ui.$ui_setter(finance_list);
                            log::info!(
                                "Updated {} data for {}",
                                $task_description,
                                stock_name_clone
                            );
                        });
                    }

                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
            });

            task_handle
        }
    };
}

pub const DEFAULT_PERIODS: [&str; 5] = ["Q22024", "Q32024", "Q42024", "Q12025", "Q22025"];

enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

pub struct QuarterPeriod {
    quarter: Quarter,
    year: i32,
}

impl QuarterPeriod {
    pub fn current_quarter() -> Self {
        let now = chrono::offset::Utc::now();
        let quarter = match (now.month() - 1) / 3 {
            0 => Quarter::Q1,
            1 => Quarter::Q2,
            2 => Quarter::Q3,
            _ => Quarter::Q4,
        };
        QuarterPeriod::new(quarter, now.year())
    }

    pub fn previous_quarter(&self) -> Self {
        let (prev_quarter, prev_year) = match self.quarter {
            Quarter::Q1 => (Quarter::Q4, self.year - 1),
            Quarter::Q2 => (Quarter::Q1, self.year),
            Quarter::Q3 => (Quarter::Q2, self.year),
            Quarter::Q4 => (Quarter::Q3, self.year),
        };
        QuarterPeriod::new(prev_quarter, prev_year)
    }

    fn new(quarter: Quarter, year: i32) -> Self {
        QuarterPeriod { quarter, year }
    }

    pub fn to_period(&self) -> String {
        match self.quarter {
            Quarter::Q1 => format!("Q1{}", self.year),
            Quarter::Q2 => format!("Q2{}", self.year),
            Quarter::Q3 => format!("Q3{}", self.year),
            Quarter::Q4 => format!("Q4{}", self.year),
        }
    }
}

// Keep the existing macro for now - will update later
create_finance_task!(
    spawn_balance_sheet_data_task_new,
    "chart.finance_sheet.balance_sheet",
    "Balance Sheet",
    fetch_balance_sheet_data,
    set_balance_sheet,
    balance_cache,
    FinanceSheetData
);

create_finance_task!(
    spawn_income_statement_task_new,
    "chart.finance_sheet.income_statement",
    "Income Statement",
    fetch_income_statement_sheet_data,
    set_income_statement,
    income_cache,
    FinanceSheetData
);

create_finance_task!(
    spawn_cash_flow_tt_task_new,
    "chart.finance_sheet.cash_flow_tt",
    "Cash Flow TT",
    fetch_cash_flow_tt_sheet_data,
    set_cash_flow_tt_statement,
    cash_flow_tt_cache,
    FinanceSheetData
);

create_finance_task!(
    spawn_cash_flow_gt_task_new,
    "chart.finance_sheet.cash_flow_gt",
    "Cash Flow GT",
    fetch_cash_flow_gt_sheet_data,
    set_cash_flow_gt_statement,
    cash_flow_gt_cache,
    FinanceSheetData
);

pub async fn spawn_balance_sheet_task(ui: &AppWindow) -> Vec<TaskHandle> {
    // Spawn 5 separate tasks for each financial sheet type and collect handles
    let mut handles = Vec::new();
    handles.push(spawn_overview_task(ui).await);
    handles.push(spawn_balance_sheet_data_task_new(ui).await);
    handles.push(spawn_income_statement_task_new(ui).await);
    handles.push(spawn_cash_flow_tt_task_new(ui).await);
    handles.push(spawn_cash_flow_gt_task_new(ui).await);

    handles
}

// Overview data task
async fn spawn_overview_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.finance_sheet.overview".to_string(),
        tx,
        "Financial Overview Data Task".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
        let pre_stock_name = Arc::new(Mutex::new(String::from("")));
        let current_stock = Arc::new(Mutex::new(String::from("AAA")));
        let overview_cache = Arc::new(Mutex::new(std::collections::HashMap::<
            String,
            Vec<Vec<FinanceSheetData>>,
        >::new()));

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
            let current_stock_clone = Arc::clone(&current_stock);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let ui_current_stock = ui.get_current_stock().symbol;
                tokio::spawn(async move {
                    let mut a = current_stock_clone.lock().await;
                    *a = ui_current_stock.to_string();
                });
            });

            let stock_name = current_stock.lock().await.clone();
            let mut overview_data = Vec::new();

            // Check cache first
            {
                let cache_guard = overview_cache.lock().await;
                if let Some(cached_data) = cache_guard.get(&stock_name) {
                    overview_data = cached_data.clone();
                }
            }

            if !overview_data.is_empty() {
                let mut pre_stock = pre_stock_name.lock().await;
                if *pre_stock != stock_name {
                    *pre_stock = stock_name.clone();
                    // Update the UI with cached data
                    let ui_handle_clone = ui_handle.clone();
                    let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                        let overview_finance_list = FinanceListExt::from_data(overview_data);
                        ui.set_overview_data(overview_finance_list);
                        log::info!("Updated overview data from cache for {stock_name}");
                    });
                }
            } else {
                let mut pre_stock = pre_stock_name.lock().await;
                *pre_stock = stock_name.clone();
                log::info!("Fetching new overview data for {stock_name}");

                // Fetch overview data
                let overview_data_raw = fetch_financial_data(&stock_name).await.unwrap_or_default();
                let overview_data = convert_financial_data_to_overview(overview_data_raw);

                // Store in cache
                overview_cache
                    .lock()
                    .await
                    .insert(stock_name.clone(), overview_data.clone());

                // Update the UI
                let ui_handle_clone = ui_handle.clone();
                let stock_name_clone = stock_name.clone();
                let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                    let overview_finance_list = FinanceListExt::from_data(overview_data);
                    ui.set_overview_data(overview_finance_list);
                    log::info!("Updated overview data for {stock_name_clone}");
                });
            }

            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    });

    task_handle
}

// All manual task functions have been replaced by the create_finance_task macro
