use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use dirs_next::cache_dir;
use tokio::sync::Mutex;

use crate::tasks::task_manager::{register_task, TaskHandle};
use crate::tasks::ChartMetaData;

/// Spawns a task to handle cache storage updates
/// Returns a TaskHandle for controlling the task
pub async fn spawn_cache_storage_task(
    chart: Arc<Mutex<ChartMetaData>>,
    custom_list: Arc<Mutex<Vec<String>>>,
) -> TaskHandle {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    // Register the task with the task manager
    let task_handle = register_task(
        "system.cache_storage".to_string(),
        tx,
        "Cache Storage Manager".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut pre_md5 = "".to_string();
        let mut pre_custom_list = Vec::new();
        let base_cache = cache_dir().expect("Could not find cache directory");
        let app_cache_dir = base_cache.join("Aim");
        std::fs::create_dir_all(&app_cache_dir).unwrap();
        let cache_file: PathBuf = app_cache_dir.join("cache.bin");
        let user_list: PathBuf = app_cache_dir.join("user_list.json");
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
        loop {
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("Cache storage task status changed to: {:?}", status);
                    task_status = status;
                }
            }
            if task_status != crate::tasks::task_manager::TaskStatus::Running {
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }

            let md5 = chart.lock().await.get_md5();
            if md5 != pre_md5 {
                // If the MD5 hash has changed, update the cache
                pre_md5 = md5;
                let file = File::create(&cache_file).unwrap();
                chart.lock().await.save(file);
            }

            // check if the user list has changed
            let custom_list_clone = custom_list.lock().await.clone();
            if pre_custom_list.len() != custom_list_clone.len() {
                // If the user list has changed, update the JSON file
                pre_custom_list = custom_list_clone.clone();
                // Save to user_list.json
                match serde_json::to_string(&custom_list_clone) {
                    Ok(json) => {
                        if let Err(e) = std::fs::write(&user_list, json) {
                            log::error!("Failed to write user_list.json: {e}");
                        } else {
                            log::info!("user_list.json updated");
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to serialize user list to JSON: {e}");
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await; // Update every 100 milliseconds
        }
    });

    task_handle
}
