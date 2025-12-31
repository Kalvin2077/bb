use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn get_xdg_config_dir() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::home_dir().expect("Unable to find home directory").join(".config"));
    base.join("bb")
}

pub fn execute_command(cmd: &str) -> Result<()> {
    let mut child = Command::new("sh").args(["-c", cmd]).spawn()?;
    child.wait()?;
    Ok(())
}
