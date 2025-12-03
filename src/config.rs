use anyhow::{Context, Result};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;
use std::os::unix::fs::PermissionsExt;

/// Create temporary directory and write all config files
pub fn setup_temp_dir(_container_name: &str) -> Result<PathBuf> {
    let temp_dir = TempDir::new().context("Failed to create temp directory")?;
    let temp_path = temp_dir.path().to_path_buf();
    
    // Write docker-compose.yaml
    let compose_content = include_str!("../configs/docker-compose.yaml");
    fs::write(temp_path.join("docker-compose.yaml"), compose_content)
        .context("Failed to write docker-compose.yaml")?;
    
    // Write prometheus.yaml
    let prometheus_content = include_str!("../configs/prometheus.yaml");
    fs::write(temp_path.join("prometheus.yaml"), prometheus_content)
        .context("Failed to write prometheus.yaml")?;
    
    // Create grafana directory structure
    fs::create_dir_all(temp_path.join("grafana/provisioning/datasources"))
        .context("Failed to create grafana datasources directory")?;
    fs::create_dir_all(temp_path.join("grafana/provisioning/dashboards"))
        .context("Failed to create grafana dashboards directory")?;
    fs::create_dir_all(temp_path.join("grafana/dashboards"))
        .context("Failed to create grafana dashboards directory")?;
    
    // Write grafana datasource config
    let datasource_content = include_str!("../configs/grafana/provisioning/datasources/prometheus.yaml");
    fs::write(
        temp_path.join("grafana/provisioning/datasources/prometheus.yaml"),
        datasource_content
    ).context("Failed to write grafana datasource config")?;
    
    // Write grafana dashboard provisioning config
    let dashboard_config = include_str!("../configs/grafana/provisioning/dashboards/dashboards.yaml");
    fs::write(
        temp_path.join("grafana/provisioning/dashboards/dashboards.yaml"),
        dashboard_config
    ).context("Failed to write grafana dashboard config")?;
    
    // Write grafana dashboard JSON
    let dashboard_json = include_str!("../configs/grafana/dashboards/condash.json");
    fs::write(
        temp_path.join("grafana/dashboards/condash.json"),
        dashboard_json
    ).context("Failed to write grafana dashboard JSON")?;
    
    // Make all directories and files readable by everyone (for Grafana container)
    set_permissions_recursive(&temp_path)?;
    
    // Prevent temp_dir from being dropped (which would delete it)
    std::mem::forget(temp_dir);
    
    Ok(temp_path)
}

/// Recursively set permissions to be readable by all users
fn set_permissions_recursive(path: &PathBuf) -> Result<()> {
    // Set directory permissions to 0755 (rwxr-xr-x)
    if path.is_dir() {
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;
        
        // Recursively process directory contents
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            set_permissions_recursive(&entry.path())?;
        }
    } else {
        // Set file permissions to 0644 (rw-r--r--)
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o644);
        fs::set_permissions(path, perms)?;
    }
    
    Ok(())
}