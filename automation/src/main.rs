use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

fn main() {
    // Path to where you cloned the repos in Phase 2
    let base_path = "/home/charlie/dev/github-automation"; 
    let repos = vec!["alpha-core-status", "rust-network-insights", "crate-dependency-watch"];

    for repo in repos {
        let path = format!("{}/{}", base_path, repo);
        if let Err(e) = run_pulse(&path) {
            eprintln!("Error updating {}: {}", repo, e);
        }
    }
}

fn run_pulse(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 1. Create a unique log entry
    let log_entry = format!("## System Pulse: {}\n- Node: AlphaCore-Main\n- Status: Operational\n\n", now);

    // 2. Write to the heartbeat file
    let mut file = OpenOptions::new().create(true).append(true).open(format!("{}/heartbeat.md", path))?;
    file.write_all(log_entry.as_bytes())?;

    // 3. Push to GitHub
    let git_commands = [
        vec!["add", "heartbeat.md"],
        vec!["commit", "-m", &format!("sys(pulse): heartbeat check {}", now)],
        vec!["push", "origin", "main"],
    ];

    for args in git_commands {
        Command::new("git").args(&args).current_dir(path).status()?;
    }
    Ok(())
}