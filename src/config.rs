use crate::models::Config;
use crate::utils::get_xdg_config_dir;
use anyhow::Result;
use std::env;
use std::fs;

pub fn load_merged_config() -> Result<Config> {
    let mut all_tools = Vec::new();
    let mut all_cheats = Vec::new();
    let mut found_any = false;

    let paths = vec![
        get_xdg_config_dir().join("config.toml"),
        env::current_dir()?.join("config.toml"),
    ];

    for path in paths {
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let cfg: Config = toml::from_str(&content)?;
            all_tools.extend(cfg.tools);
            all_cheats.extend(cfg.cheats);
            found_any = true;
        }
    }

    if !found_any {
        anyhow::bail!("Unabled to find any config file in ~/.config/bb or current directory");
    }

    Ok(Config {
        tools: all_tools,
        cheats: all_cheats,
    })
}
