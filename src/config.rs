use anyhow::{Context, Result};
use std::fs;

// Read signatures file
pub fn read_signatures(path: &str) -> Result<Vec<String>> {
    // Read path
    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not read signatures file {}", path))?;
    Ok(content.lines().map(String::from).collect())
}
