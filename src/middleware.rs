use std::time::{Instant, Duration};

/// The trait that all AlphaCore middleware must implement.
/// 'Send + Sync' ensures this can safely move between threads.
pub trait Middleware: Send + Sync {
    fn name(&self) -> &'static str;
    fn on_start(&self, task_name: &str);
    fn on_end(&self, task_name: &str, duration: Duration);
}

/// A standard Logger middleware to track task performance.
pub struct MetricsLogger;

impl Middleware for MetricsLogger {
    fn name(&self) -> &'static str {
        "MetricsLogger"
    }

    fn on_start(&self, task_name: &str) {
        println!("[AlphaCore] 🚀 Starting task: {}", task_name);
    }

    fn on_end(&self, task_name: &str, duration: Duration) {
        println!(
            "[AlphaCore] ✅ Completed task: {} (took {:?})",
            task_name, duration
        );
    }
}

/// A wrapper to execute tasks with middleware timing.
/// This is the "Engine Room" that actually times the work.
pub async fn execute_with_telemetry<F, Fut>(task_name: &str, middleware: &dyn Middleware, f: F) 
where 
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    middleware.on_start(task_name);
    let start = Instant::now();
    
    f().await;
    
    let duration = start.elapsed();
    middleware.on_end(task_name, duration);
}