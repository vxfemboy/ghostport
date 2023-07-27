use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

// Entire config
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    // Payloads to send
    pub payloads: Vec<String>,
}

// Read config file
pub fn read(path: &str) -> Result<Config> {
    // Read path
    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not parse config file {}", path))?;
    // Parse JSON
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}
