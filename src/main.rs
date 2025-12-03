use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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
    docker::compose_down(&temp_dir)?;
    println!("   {}", "✓ Done.".green());
    
    Ok(())
}