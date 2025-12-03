use anyhow::{Context, Result};
use std::thread;
use std::time::Duration;

/// Wait for Grafana to become healthy
pub fn wait_for_grafana() -> Result<()> {
    let max_attempts = 30;
    let wait_time = Duration::from_secs(2);

    for attempt in 1..=max_attempts {
        match check_grafana_health() {
            Ok(true) => return Ok(()),
            Ok(false) => {
                if attempt == max_attempts {
                    anyhow::bail!(
                        "Grafana failed to become healthy after {} attempts",
                        max_attempts
                    );
                }
                thread::sleep(wait_time);
            }
            Err(e) => {
                if attempt == max_attempts {
                    return Err(e).context("Failed to check Grafana health");
                }
                thread::sleep(wait_time);
            }
        }
    }

    Ok(())
}

/// Check if Grafana is healthy
fn check_grafana_health() -> Result<bool> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get("http://localhost:3000/api/health")
        .timeout(Duration::from_secs(5))
        .send();

    match response {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => Ok(false),
    }
}
