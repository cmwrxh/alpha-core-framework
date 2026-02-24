use std::process::Command;
use chrono::{Duration, Utc};
use rand::Rng; 
use rand::seq::SliceRandom;

fn main() {
    let messages = vec![
        "feat: implement asynchronous logging",
        "refactor: optimize memory allocation in core loop",
        "fix: resolve race condition in thread pool",
        "docs: update API documentation for v2.0",
        "test: add integration tests for auth module",
        "perf: reduce O(n) complexity to O(log n)",
        "chore: clean up deprecated helper functions",
        "feat: add support for environment-specific configs"
    ];

    let mut rng = rand::thread_rng();
    let total_days = 1095; // 3 years of history

    println!("Starting the 10x Rockstar transformation... this will take a moment.");

    for day in (0..total_days).rev() {
        let date = Utc::now() - Duration::days(day);
        let formatted_date = date.format("%Y-%m-%dT%H:%M:%S").to_string();
        
        if rng.gen_bool(0.8) {
            let commit_count = rng.gen_range(3..12); 
            for _ in 0..commit_count {
                let msg = messages.choose(&mut rng).unwrap();
                
                Command::new("git")
                    .args(["commit", "--allow-empty", "-m", msg])
                    .env("GIT_AUTHOR_DATE", &formatted_date)
                    .env("GIT_COMMITTER_DATE", &formatted_date)
                    .current_dir("..") 
                    .output()
                    .expect("Failed to execute git commit");
            }
        }
    }

    println!("--------------------------------------------------");
    println!("SUCCESS: 3 years of history backfilled.");
    println!("Your local repo is now packed with over 8,000 commits.");
    println!("Next step: Push to GitHub to turn your profile GREEN.");
    println!("--------------------------------------------------");
}