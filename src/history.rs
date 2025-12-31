use crate::models::History;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn load_history(path: &PathBuf) -> Result<History> {
    if !path.exists() {
        return Ok(History::default());
    }
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content).unwrap_or_default())
}

pub fn save_history(path: &PathBuf, history: &History) -> Result<()> {
    let content = serde_json::to_string_pretty(history)?;
    fs::write(path, content)?;
    Ok(())
}
