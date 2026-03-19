use alpha_core_framework::pipeline::Orchestrator;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // 1. Initialize the AlphaCore Engine
    let engine = Orchestrator::new("Nairobi-Lab-Alpha-01");
    
    println!("--- AlphaCore Framework: Online ---");

    // 2. Run a simulated "Data Fetch" task
    engine.run_task("fetch_market_data", || async {
        // Simulate a network delay (like fetching from an API)
        sleep(Duration::from_millis(800)).await;
        println!("   [Internal] Data received from remote source.");
    }).await;

    // 3. Run a simulated "SIMD Process" task
    engine.run_task("process_vector_data", || async {
        // Simulate heavy CPU work
        sleep(Duration::from_millis(450)).await;
        println!("   [Internal] 16,384 records processed via SIMD.");
    }).await;

    println!("--- All Tasks Complete ---");
}