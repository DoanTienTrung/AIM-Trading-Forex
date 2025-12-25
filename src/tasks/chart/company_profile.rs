use crate::tasks::task_manager::{register_task, TaskHandle};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::slint_generatedAppWindow::{
    InsiderTransaction as UIInsiderTransaction, Officer as UIOfficer,
    SharedHolder as UISharedHolder, Subsidiary as UISubsidiary,
};
use crate::AppWindow;
use aim_data::aim::{
    fetch_insider_transactions_data, fetch_institution_data, fetch_officers_data,
    fetch_sharedholder_data, fetch_subsidiaries_data, InsiderTransaction as ApiInsiderTransaction,
    InstitutionData, Officer as ApiOfficer, SharedHolder as ApiSharedHolder,
    Subsidiary as ApiSubsidiary,
};

// Convert API data to UI data structures
fn convert_api_shareholder_to_ui(api_holder: &ApiSharedHolder) -> UISharedHolder {
    UISharedHolder {
        name: api_holder.name.clone().into(),
        shares: api_holder.shares as i32,
        percent: (api_holder.ownership * 100.0) as f32,
        updated_at: format_timestamp_to_date(api_holder.reported_at).into(),
    }
}

fn convert_api_subsidiary_to_ui(api_subsidiary: &ApiSubsidiary) -> UISubsidiary {
    UISubsidiary {
        name: api_subsidiary.company_name.clone().into(),
        charter_capital: api_subsidiary.charter_capital as f32 / 1_000_000_000.0, // Convert to billions
        ownership_percent: (api_subsidiary.ownership * 100.0) as f32,
    }
}

fn convert_api_officer_to_ui(api_officer: &ApiOfficer) -> UIOfficer {
    UIOfficer {
        name: api_officer.name.clone().into(),
        position: api_officer.position.clone().into(),
        shares: 0,                // The API doesn't provide shares info for officers
        updated_at: "N/A".into(), // The API doesn't provide updated_at for officers
    }
}

fn convert_api_insider_transaction_to_ui(
    api_transaction: &ApiInsiderTransaction,
) -> UIInsiderTransaction {
    UIInsiderTransaction {
        name: api_transaction.name.clone().into(),
        transaction_type: if api_transaction.transaction_type == 1 {
            "Mua".into()
        } else {
            "Bán".into()
        },
        registered_quantity: api_transaction.registered_volume.unwrap_or(0.0) as i32,
        executed_quantity: api_transaction.execution_volume.unwrap_or(0.0) as i32,
        registration_date: format_timestamp_to_date(api_transaction.start_date).into(),
        execution_date: format_timestamp_to_date(api_transaction.execution_date).into(),
        price_before_gd: 0.0, // API doesn't provide this data
        price_after_gd: 0.0,  // API doesn't provide this data
    }
}

// Helper function to convert timestamp to readable date
fn format_timestamp_to_date(timestamp: i64) -> String {
    use chrono::{TimeZone, Utc};

    match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%d/%m/%Y").to_string(),
        _ => "N/A".to_string(),
    }
}

// Helper function to decode HTML entities and convert HTML to plain text
fn decode_html_to_text(html: &str) -> String {
    let mut text = html.to_string();

    // Decode common HTML entities
    text = text.replace("&amp;", "&");
    text = text.replace("&lt;", "<");
    text = text.replace("&gt;", ">");
    text = text.replace("&quot;", "\"");
    text = text.replace("&#39;", "'");
    text = text.replace("&nbsp;", " ");
    text = text.replace("&ocirc;", "ô");
    text = text.replace("&acirc;", "â");
    text = text.replace("&iacute;", "í");
    text = text.replace("&ecirc;", "ê");
    text = text.replace("&agrave;", "à");
    text = text.replace("&aacute;", "á");
    text = text.replace("&atilde;", "ã");
    text = text.replace("&uacute;", "ú");
    text = text.replace("&ugrave;", "ù");
    text = text.replace("&otilde;", "õ");
    text = text.replace("&oacute;", "ó");
    text = text.replace("&ograve;", "ò");
    text = text.replace("&yacute;", "ý");
    text = text.replace("&ndash;", "–");
    text = text.replace("&mdash;", "—");

    // Remove HTML tags and convert to plain text
    let mut result = String::new();
    let mut in_tag = false;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '<' => {
                in_tag = true;
                // Check if this is a list item or break tag for formatting
                let tag_preview: String = chars.clone().take(10).collect();
                if tag_preview.to_lowercase().starts_with("li>") {
                    result.push_str("\n• ");
                } else if tag_preview.to_lowercase().starts_with("/li>") {
                    // Do nothing, already handled by <li>
                } else if tag_preview.to_lowercase().starts_with("ul>")
                    || tag_preview.to_lowercase().starts_with("/ul>")
                    || tag_preview.to_lowercase().starts_with("br")
                {
                    result.push('\n');
                }
            }
            '>' => {
                in_tag = false;
            }
            _ => {
                if !in_tag {
                    result.push(ch);
                }
            }
        }
    }

    // Clean up extra whitespace and newlines
    result = result
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    result
}

pub async fn spawn_company_profile_task(ui: &AppWindow) -> Vec<TaskHandle> {
    // Spawn separate tasks for each data type and collect their handles
    let mut handles = Vec::new();
    handles.push(spawn_shareholders_task(ui).await);
    handles.push(spawn_institution_task(ui).await);
    handles.push(spawn_subsidiaries_task(ui).await);
    handles.push(spawn_officers_task(ui).await);
    handles.push(spawn_insider_transactions_task(ui).await);

    handles
}

// Shareholders data task
async fn spawn_shareholders_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.company_profile.shareholders".to_string(),
        tx,
        "Shareholders Data Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let pre_stock_name = Arc::new(Mutex::new(String::from("")));
        let current_stock = Arc::new(Mutex::new(String::from("AAA")));
        let shareholders_cache = Arc::new(Mutex::new(std::collections::HashMap::<
            String,
            Vec<ApiSharedHolder>,
        >::new()));
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
            // Get current stock symbol from UI
            let current_stock_clone = Arc::clone(&current_stock);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let ui_current_stock = ui.get_current_stock().symbol;
                tokio::spawn(async move {
                    let mut stock = current_stock_clone.lock().await;
                    *stock = ui_current_stock.to_string();
                });
            });

            let stock_name = current_stock.lock().await.clone();
            let mut shareholders_data = Vec::new();

            // Check cache first
            {
                let cache_guard = shareholders_cache.lock().await;
                if let Some(cached_data) = cache_guard.get(&stock_name) {
                    shareholders_data = cached_data.clone();
                }
            }

            let mut pre_stock = pre_stock_name.lock().await;
            if *pre_stock != stock_name {
                *pre_stock = stock_name.clone();

                if shareholders_data.is_empty() {
                    log::info!("Fetching new shareholders data for {stock_name}");

                    // Fetch new data from API
                    match fetch_sharedholder_data(&stock_name).await {
                        Ok(api_data) => {
                            shareholders_data = api_data;

                            // Store in cache
                            shareholders_cache
                                .lock()
                                .await
                                .insert(stock_name.clone(), shareholders_data.clone());
                        }
                        Err(e) => {
                            log::error!("Failed to fetch shareholders data for {stock_name}: {e}");
                            continue;
                        }
                    }
                } else {
                    log::info!("Using cached shareholders data for {stock_name}");
                }

                // Sort shareholders data by ownership percentage (high to low)
                shareholders_data.sort_by(|a, b| {
                    b.ownership
                        .partial_cmp(&a.ownership)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                // Convert API data to UI data and update the UI
                let ui_shareholders: Vec<UISharedHolder> = shareholders_data
                    .iter()
                    .map(convert_api_shareholder_to_ui)
                    .collect();

                let ui_handle_clone = ui_handle.clone();
                let stock_name_clone = stock_name.clone();
                let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                    let shareholders_model = ModelRc::new(VecModel::from(ui_shareholders));
                    ui.set_shared_holders(shareholders_model);
                    log::info!("Updated shareholders data for {stock_name_clone}");
                });
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    task_handle
}

// Institution data task
async fn spawn_institution_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.company_profile.institution".to_string(),
        tx,
        "Institution Data Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
        let pre_stock_name = Arc::new(Mutex::new(String::from("")));
        let current_stock = Arc::new(Mutex::new(String::from("AAA")));
        let institution_cache = Arc::new(Mutex::new(std::collections::HashMap::<
            String,
            InstitutionData,
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

            // Get current stock symbol from UI
            let current_stock_clone = Arc::clone(&current_stock);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let ui_current_stock = ui.get_current_stock().symbol;
                tokio::spawn(async move {
                    let mut stock = current_stock_clone.lock().await;
                    *stock = ui_current_stock.to_string();
                });
            });

            let stock_name = current_stock.lock().await.clone();
            let mut institution_data: Option<InstitutionData> = None;

            // Check cache first
            {
                let cache_guard = institution_cache.lock().await;
                if let Some(cached_data) = cache_guard.get(&stock_name) {
                    institution_data = Some(cached_data.clone());
                }
            }

            let mut pre_stock = pre_stock_name.lock().await;
            if *pre_stock != stock_name {
                *pre_stock = stock_name.clone();

                if institution_data.is_none() {
                    log::info!("Fetching new institution data for {stock_name}");

                    // Fetch new data from API
                    match fetch_institution_data(&stock_name).await {
                        Ok(api_data) => {
                            institution_data = Some(api_data.clone());

                            // Store in cache
                            institution_cache
                                .lock()
                                .await
                                .insert(stock_name.clone(), api_data);
                        }
                        Err(e) => {
                            log::error!("Failed to fetch institution data for {stock_name}: {e}");
                            continue;
                        }
                    }
                } else {
                    log::info!("Using cached institution data for {stock_name}");
                }

                // Update the UI with institution data (this can be used for company overview)
                if let Some(data) = institution_data {
                    let ui_handle_clone = ui_handle.clone();
                    let stock_name_clone = stock_name.clone();
                    let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                        // Decode HTML content for history
                        let decoded_history = decode_html_to_text(&data.history);

                        // Create a simplified company overview text focusing on overview and history
                        let overview_text = format!(
                            "TỔNG QUAN CÔNG TY\n\n{}\n\nLỊCH SỬ PHÁT TRIỂN\n\n• Ngày thành lập: {}\n• Vốn điều lệ: {:.0} tỷ VND\n\nLịch sử phát triển:\n{}\n\n• Trụ sở chính: {}\n\n{}",
                            data.overview,
                            if let Some(establishment_date) = data.establishment_date {
                                establishment_date
                            } else {
                                "N/A".to_string()
                            },
                            data.charter_capital / 1_000_000_000.0, // Convert to billions
                            decoded_history,
                            data.head_quarters,
                            if data.is_listed {
                                format!("• Niêm yết tại: {}", data.exchange.as_ref().unwrap_or(&"N/A".to_string()))
                            } else {
                                "• Trạng thái: Chưa niêm yết".to_string()
                            }
                        );

                        ui.set_company_overview(overview_text.into());
                        log::info!("Updated company overview for {}: {}", stock_name_clone, data.company_name);
                    });
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    task_handle
}

// Subsidiaries data task
async fn spawn_subsidiaries_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.company_profile.subsidiaries".to_string(),
        tx,
        "Subsidiaries Data Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
        let pre_stock_name = Arc::new(Mutex::new(String::from("")));
        let current_stock = Arc::new(Mutex::new(String::from("AAA")));
        let subsidiaries_cache = Arc::new(Mutex::new(std::collections::HashMap::<
            String,
            Vec<ApiSubsidiary>,
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

            // Get current stock symbol from UI
            let current_stock_clone = Arc::clone(&current_stock);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let ui_current_stock = ui.get_current_stock().symbol;
                tokio::spawn(async move {
                    let mut stock = current_stock_clone.lock().await;
                    *stock = ui_current_stock.to_string();
                });
            });

            let stock_name = current_stock.lock().await.clone();
            let mut subsidiaries_data = Vec::new();

            // Check cache first
            {
                let cache_guard = subsidiaries_cache.lock().await;
                if let Some(cached_data) = cache_guard.get(&stock_name) {
                    subsidiaries_data = cached_data.clone();
                }
            }

            let mut pre_stock = pre_stock_name.lock().await;
            if *pre_stock != stock_name {
                *pre_stock = stock_name.clone();

                if subsidiaries_data.is_empty() {
                    log::info!("Fetching new subsidiaries data for {stock_name}");

                    // Fetch new data from API
                    match fetch_subsidiaries_data(&stock_name).await {
                        Ok(api_data) => {
                            subsidiaries_data = api_data;

                            // Store in cache
                            subsidiaries_cache
                                .lock()
                                .await
                                .insert(stock_name.clone(), subsidiaries_data.clone());
                        }
                        Err(e) => {
                            log::error!("Failed to fetch subsidiaries data for {stock_name}: {e}");
                            continue;
                        }
                    }
                } else {
                    log::info!("Using cached subsidiaries data for {stock_name}");
                }

                // Convert API data to UI data and update the UI
                let ui_subsidiaries: Vec<UISubsidiary> = subsidiaries_data
                    .iter()
                    .map(convert_api_subsidiary_to_ui)
                    .collect();

                let ui_handle_clone = ui_handle.clone();
                let stock_name_clone = stock_name.clone();
                let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                    let subsidiaries_model = ModelRc::new(VecModel::from(ui_subsidiaries));
                    ui.set_subsidiaries(subsidiaries_model);
                    log::info!("Updated subsidiaries data for {stock_name_clone}");
                });
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    task_handle
}

// Officers data task
async fn spawn_officers_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.company_profile.officers".to_string(),
        tx,
        "Officers Data Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let pre_stock_name = Arc::new(Mutex::new(String::from("")));
        let current_stock = Arc::new(Mutex::new(String::from("AAA")));
        let officers_cache = Arc::new(Mutex::new(std::collections::HashMap::<
            String,
            Vec<ApiOfficer>,
        >::new()));
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

            // Get current stock symbol from UI
            let current_stock_clone = Arc::clone(&current_stock);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let ui_current_stock = ui.get_current_stock().symbol;
                tokio::spawn(async move {
                    let mut stock = current_stock_clone.lock().await;
                    *stock = ui_current_stock.to_string();
                });
            });

            let stock_name = current_stock.lock().await.clone();
            let mut officers_data = Vec::new();

            // Check cache first
            {
                let cache_guard = officers_cache.lock().await;
                if let Some(cached_data) = cache_guard.get(&stock_name) {
                    officers_data = cached_data.clone();
                }
            }

            let mut pre_stock = pre_stock_name.lock().await;
            if *pre_stock != stock_name {
                *pre_stock = stock_name.clone();

                if officers_data.is_empty() {
                    log::info!("Fetching new officers data for {stock_name}");

                    // Fetch new data from API
                    match fetch_officers_data(&stock_name).await {
                        Ok(api_data) => {
                            officers_data = api_data;

                            // Store in cache
                            officers_cache
                                .lock()
                                .await
                                .insert(stock_name.clone(), officers_data.clone());
                        }
                        Err(e) => {
                            log::error!("Failed to fetch officers data for {stock_name}: {e}");
                            continue;
                        }
                    }
                } else {
                    log::info!("Using cached officers data for {stock_name}");
                }

                // Convert API data to UI data and update the UI
                let ui_officers: Vec<UIOfficer> = officers_data
                    .iter()
                    .map(convert_api_officer_to_ui)
                    .collect();

                let ui_handle_clone = ui_handle.clone();
                let stock_name_clone = stock_name.clone();
                let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                    let officers_model = ModelRc::new(VecModel::from(ui_officers));
                    ui.set_officers(officers_model);
                    log::info!("Updated officers data for {stock_name_clone}");
                });
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    task_handle
}

// Insider transactions data task
async fn spawn_insider_transactions_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.company_profile.insider_transactions".to_string(),
        tx,
        "Insider Transactions Data Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let pre_stock_name = Arc::new(Mutex::new(String::from("")));
        let current_stock = Arc::new(Mutex::new(String::from("AAA")));
        let insider_transactions_cache = Arc::new(Mutex::new(std::collections::HashMap::<
            String,
            Vec<ApiInsiderTransaction>,
        >::new()));
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

            // Get current stock symbol from UI
            let current_stock_clone = Arc::clone(&current_stock);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let ui_current_stock = ui.get_current_stock().symbol;
                tokio::spawn(async move {
                    let mut stock = current_stock_clone.lock().await;
                    *stock = ui_current_stock.to_string();
                });
            });

            let stock_name = current_stock.lock().await.clone();
            let mut insider_transactions_data = Vec::new();

            // Check cache first
            {
                let cache_guard = insider_transactions_cache.lock().await;
                if let Some(cached_data) = cache_guard.get(&stock_name) {
                    insider_transactions_data = cached_data.clone();
                }
            }

            let mut pre_stock = pre_stock_name.lock().await;
            if *pre_stock != stock_name {
                *pre_stock = stock_name.clone();

                if insider_transactions_data.is_empty() {
                    log::info!("Fetching new insider transactions data for {stock_name}");

                    // Fetch new data from API
                    match fetch_insider_transactions_data(&stock_name).await {
                        Ok(api_data) => {
                            insider_transactions_data = api_data;

                            // Store in cache
                            insider_transactions_cache
                                .lock()
                                .await
                                .insert(stock_name.clone(), insider_transactions_data.clone());
                        }
                        Err(e) => {
                            log::error!(
                                "Failed to fetch insider transactions data for {stock_name}: {e}"
                            );
                            continue;
                        }
                    }
                } else {
                    log::info!("Using cached insider transactions data for {stock_name}");
                }

                // Convert API data to UI data and update the UI
                let ui_insider_transactions: Vec<UIInsiderTransaction> = insider_transactions_data
                    .iter()
                    .map(convert_api_insider_transaction_to_ui)
                    .collect();

                let ui_handle_clone = ui_handle.clone();
                let stock_name_clone = stock_name.clone();
                let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                    let insider_transactions_model =
                        ModelRc::new(VecModel::from(ui_insider_transactions));
                    ui.set_insider_transactions(insider_transactions_model);
                    log::info!("Updated insider transactions data for {stock_name_clone}");
                });
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    task_handle
}
