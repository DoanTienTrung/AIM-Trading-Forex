use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Status of a task
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Running,
    Paused,
}

/// Handle for controlling a specific task
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TaskHandle {
    pub id: String,
    pub status: TaskStatus,
    pub tx: tokio::sync::mpsc::Sender<TaskStatus>,
}

impl TaskHandle {
    pub fn new(id: String, tx: tokio::sync::mpsc::Sender<TaskStatus>) -> Self {
        Self {
            id,
            status: TaskStatus::Running,
            tx,
        }
    }

    /// Change status of the task
    pub async fn change_status(&mut self, status: TaskStatus) {
        self.status = status.clone();
        let _ = self.tx.send(status).await;
    }
}

/// Manager for all spawned tasks
pub struct TaskManager {
    tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
}

struct TaskInfo {
    handle: TaskHandle,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new task
    pub async fn register_task(
        &self,
        id: String,
        tx: tokio::sync::mpsc::Sender<TaskStatus>,
        description: String,
    ) -> TaskHandle {
        let task_handle = TaskHandle::new(id.clone(), tx);
        let task_info = TaskInfo {
            handle: task_handle.clone(),
        };

        let mut tasks = self.tasks.write().await;
        tasks.insert(id.clone(), task_info);
        log::info!("Registered task: {} - {}", id, description);

        task_handle
    }

    /// Pause tasks by category (e.g., "dashboard", "chart", "market_watch")
    pub async fn pause_tasks_by_category(&self, category: &str) {
        let mut tasks = self.tasks.write().await;
        for (id, task_info) in tasks.iter_mut() {
            if id.starts_with(category) {
                task_info.handle.change_status(TaskStatus::Paused).await;
            }
        }
    }

    /// Resume tasks by category (e.g., "dashboard", "chart", "market_watch")
    pub async fn resume_tasks_by_category(&self, category: &str) {
        let mut tasks = self.tasks.write().await;
        for (id, task_info) in tasks.iter_mut() {
            if id.starts_with(category) {
                task_info.handle.change_status(TaskStatus::Running).await;
            }
        }
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

// Global task manager instance
lazy_static::lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = TaskManager::new();
}

/// Convenience functions for global task manager
pub async fn register_task(
    id: String,
    tx: tokio::sync::mpsc::Sender<TaskStatus>,
    description: String,
) -> TaskHandle {
    TASK_MANAGER.register_task(id, tx, description).await
}
