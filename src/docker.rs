use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Validate that a Docker container exists and is running
pub fn validate_container(container_name: &str) -> Result<()> {
    let output = Command::new("docker")
        .args(["inspect", container_name])
        .output()
        .context("Failed to execute docker inspect")?;
    
    if !output.status.success() {
        anyhow::bail!("Container '{}' not found or not accessible", container_name);
    }
    
    Ok(())
}

/// Start the monitoring stack using docker compose
pub fn compose_up(temp_dir: &Path) -> Result<()> {
    let compose_file = temp_dir.join("docker-compose.yml");
    
    let output = Command::new("docker")
        .args(["compose", "-f", compose_file.to_str().unwrap(), "up", "-d"])
        .output()
        .context("Failed to start docker compose")?;
    
    if !output.status.success() {
        anyhow::bail!("Docker compose up failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}

/// Stop and remove the monitoring stack
pub fn compose_down(temp_dir: &Path) -> Result<()> {
    let compose_file = temp_dir.join("docker-compose.yml");
    
    let output = Command::new("docker")
        .args(["compose", "-f", compose_file.to_str().unwrap(), "down", "-v"])
        .output()
        .context("Failed to stop docker compose")?;
    
    if !output.status.success() {
        anyhow::bail!("Docker compose down failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}