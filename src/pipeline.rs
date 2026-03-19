use std::sync::Arc;
use tokio::sync::RwLock;

/// Core Orchestrator for AlphaCore Framework
pub struct Orchestrator {
    pub name: String,
    pub tasks: Arc<RwLock<Vec<String>>>,
}

impl Orchestrator {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_task(&self, task_name: &str) {
        let mut tasks = self.tasks.write().await;
        tasks.push(task_name.to_string());
        println!("Task '{}' added to pipeline '{}'", task_name, self.name);
    }

    pub async fn run(&self) {
        let tasks = self.tasks.read().await;
        println!("Starting execution of {} tasks...", tasks.len());
        for task in tasks.iter() {
            println!("Running: {}", task);
        }
    }
}