use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::path::PathBuf;

mod docker;
mod config;
mod health;
mod browser;

/// condash: ephemeral monitoring for Docker containers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the Docker container to monitor
    container: String,
}

/// Cleanup guard that ensures compose_down runs on drop
struct ComposeGuard {
    temp_dir: PathBuf,
    active: bool,
}

impl ComposeGuard {
    fn new(temp_dir: PathBuf) -> Self {
        ComposeGuard {
            temp_dir,
            active: true,
        }
    }
    
    fn cleanup(&mut self) -> Result<()> {
        if self.active {
            docker::compose_down(&self.temp_dir)?;
            self.active = false;
        }
        Ok(())
    }
}

impl Drop for ComposeGuard {
    fn drop(&mut self) {
        if self.active {
            // Best effort cleanup on drop
            let _ = docker::compose_down(&self.temp_dir);
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("{}", "🔍 Validating container...".bright_blue());
    docker::validate_container(&args.container)?;
    println!("   {} Found container: {}", "✓".green(), args.container);
    
    println!("{}", "🔌 Checking port availability...".bright_blue());
    docker::check_ports_available()?;
    println!("   {} Ports 3000, 9090, 8080 are available", "✓".green());
    
    println!("{}", "🚀 Starting monitoring stack...".bright_blue());
    let temp_dir = config::setup_temp_dir(&args.container)?;
    
    docker::compose_up(&temp_dir)?;
    
    // Create cleanup guard - will run compose_down on ANY exit
    let mut guard = ComposeGuard::new(temp_dir.clone());
    
    println!("   {} cAdvisor  {} Prometheus  {} Grafana", 
             "✓".green(), "✓".green(), "✓".green());
    
    println!("{}", "⏳ Waiting for Grafana...".bright_blue());
    health::wait_for_grafana()?;
    
    let url = format!("http://localhost:3000/d/condash?var-container={}", args.container);
    println!("{} Opening dashboard: {}", "🌐".bright_blue(), url);
    browser::open_dashboard(&url)?;
    
    println!("{}", "Press Ctrl+C to stop monitoring and clean up.".bright_yellow());
    
    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;
    
    // Wait for Ctrl+C
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    println!("\n{}", "🧹 Shutting down...".bright_blue());
    guard.cleanup()?;
    println!("   {}", "✓ Done.".green());
    
    Ok(())
}