use crate::slint_generatedAppWindow::AppWindow;
use crate::tasks::task_manager::TASK_MANAGER;
use slint::ComponentHandle;

// Global mutex to prevent concurrent page changes
static PAGE_CHANGE_MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// Initialize page manager and get initial page state
pub async fn initialize_page_manager(ui: &AppWindow) {
    // Get the initial active page from UI
    let initial_page = ui.get_active_page();
    log::info!(
        "Initializing page manager with current page: {}",
        initial_page
    );

    // Set initial task state based on current page
    handle_page_change(initial_page).await;
    log::info!("Page-aware task manager initialized successfully");
}

/// Manual page change handler - use this for programmatic page changes
pub async fn handle_page_change(page: i32) {
    let _lock = PAGE_CHANGE_MUTEX.lock().await;
    log::info!("Manual page change to page {} (protected)", page);
    activate_page_tasks(page).await;
}

/// Start active UI page monitoring with debouncing
pub async fn start_page_monitoring(ui: &AppWindow) {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10); // Increased buffer
    let ui_handle = ui.as_weak();

    // Task to monitor UI page changes with reduced frequency
    tokio::spawn(async move {
        loop {
            let tx_clone = tx.clone();

            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let pre_page = ui.get_prev_page();
                let page = ui.get_active_page();
                if pre_page != page && tx_clone.try_send(page).is_ok() {
                    ui.set_prev_page(page);
                }
            });

            tokio::time::sleep(std::time::Duration::from_millis(10)).await; // Reduced frequency
        }
    });

    // Task to handle page changes with debouncing
    tokio::spawn(async move {
        let mut current_page = -1i32;

        log::info!("Starting UI active-page monitoring with debouncing...");

        while let Some(active_page) = rx.recv().await {
            log::info!("Detected page change to {}", active_page);
            if current_page != active_page {
                log::info!(
                    "UI active-page changed from {} to {} (debounced)",
                    current_page,
                    active_page
                );

                // Use mutex to prevent concurrent page changes
                let _lock = PAGE_CHANGE_MUTEX.lock().await;

                // First pause tasks from the previous page
                if current_page >= 0 {
                    pause_page_tasks(current_page).await;
                }

                // Wait a small delay to ensure tasks are properly paused
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                // Then activate tasks for the new page
                activate_page_tasks(active_page).await;

                // Small delay before releasing lock
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;

                current_page = active_page;
            }
        }
        log::info!("UI page monitor task ended");
    });
}

/// Pause tasks for a specific page
async fn pause_page_tasks(page: i32) {
    log::info!("🔴 PAUSING tasks for page {}", page);
    match page {
        0 => {
            // Leaving dashboard - pause dashboard tasks only (keep system running)
            log::info!("  └─ Pausing dashboard category tasks");
            TASK_MANAGER.pause_tasks_by_category("dashboard").await;
        }
        1 => {
            // Leaving market watch - pause market watch tasks only (keep system running)
            log::info!("  └─ Pausing market_watch category tasks");
            TASK_MANAGER.pause_tasks_by_category("market_watch").await;
        }
        2 => {
            // Leaving chart - pause chart tasks only (keep system running)
            log::info!("  └─ Pausing chart category tasks");
            TASK_MANAGER.pause_tasks_by_category("chart").await;
        }
        _ => {
            // Leaving other pages - might need to pause any remaining active tasks
            log::info!(
                "  └─ No specific tasks to pause for page {} (other pages)",
                page
            );
        }
    }
    log::info!("🔴 PAUSE completed for page {}", page);
}

/// Activate tasks for a specific page
async fn activate_page_tasks(page: i32) {
    log::info!("🟢 ACTIVATING tasks for page {}", page);
    match page {
        0 => {
            // Entering dashboard - activate dashboard tasks
            log::info!("  └─ Resuming dashboard + system category tasks");
            TASK_MANAGER.resume_tasks_by_category("dashboard").await;
        }
        1 => {
            // Entering market watch - activate market watch tasks
            log::info!("  └─ Resuming market_watch + system category tasks");
            TASK_MANAGER.resume_tasks_by_category("market_watch").await;
        }
        2 => {
            // Entering chart - activate chart tasks
            log::info!("  └─ Resuming chart + system category tasks");
            TASK_MANAGER.resume_tasks_by_category("chart").await;
            TASK_MANAGER.resume_tasks_by_category("system").await;
        }
        _ => {
            // Entering other pages - only keep system tasks running
            log::info!("  └─ Resuming system category tasks only");
        }
    }
    log::info!("🟢 ACTIVATION completed for page {}", page);
}
