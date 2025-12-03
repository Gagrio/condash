use anyhow::{Context, Result};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;

/// Create temporary directory and write all config files
pub fn setup_temp_dir(container_name: &str) -> Result<PathBuf> {
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
    
    // Prevent temp_dir from being dropped (which would delete it)
    std::mem::forget(temp_dir);
    
    Ok(temp_path)
}