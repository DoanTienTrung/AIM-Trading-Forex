mod dashboard;

pub use dashboard::*;
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
        pub async fn $task_fn(ui: &$crate::AppWindow) {
            use slint::ComponentHandle;

            let ui_handle = ui.as_weak();

            tokio::spawn(async move {

                loop {
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
        }
    };
}
