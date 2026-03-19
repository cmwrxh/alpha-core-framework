use crate::middleware::{Middleware, MetricsLogger, execute_with_telemetry};
use std::sync::Arc;

/// The main engine that manages and runs tasks.
pub struct Orchestrator {
    pub name: String,
    // We use Arc (Atomic Reference Counter) so multiple threads can share the logger
    pub middleware: Arc<dyn Middleware>,
}

impl Orchestrator {
    /// Create a new engine with a default logger.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            middleware: Arc::new(MetricsLogger),
        }
    }

    /// Run a task with full telemetry and timing.
    pub async fn run_task<F, Fut>(&self, task_name: &str, task_logic: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        execute_with_telemetry(task_name, &*self.middleware, task_logic).await;
    }
}