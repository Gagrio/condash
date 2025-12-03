use anyhow::{Context, Result};

/// Open the Grafana dashboard in the default browser
pub fn open_dashboard(url: &str) -> Result<()> {
    webbrowser::open(url).context("Failed to open browser")?;

    // Always print URL as fallback
    println!("   Dashboard URL: {}", url);

    Ok(())
}
