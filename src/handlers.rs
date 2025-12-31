use crate::config::load_merged_config;
use crate::history::save_history;
use crate::models::{CheatSheet, Config, History, Tool};
use crate::utils::{execute_command, get_xdg_config_dir};

use anyhow::Result;
use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use std::fs;
use std::path::PathBuf;

pub fn handle_run(history: &mut History, history_path: &PathBuf) -> Result<()> {
    let config = load_merged_config()?;
    let mut tools = config.tools;

    tools.sort_by(|a, b| {
        let count_a = history.usage.get(&a.id).unwrap_or(&0);
        let count_b = history.usage.get(&b.id).unwrap_or(&0);
        count_b.cmp(count_a)
    });

    let display_items: Vec<String> = tools
        .iter()
        .map(|t| {
            format!(
                "{} [used {} times]",
                t.name,
                history.usage.get(&t.id).unwrap_or(&0)
            )
        })
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Input to filter, use arrow keys to navigate and Enter to select:")
        .items(&display_items)
        .default(0)
        .interact()?;

    let selected_tool = &tools[selection];

    history
        .usage
        .entry(selected_tool.id.clone())
        .and_modify(|c| *c += 1)
        .or_insert(1);
    save_history(history_path, history)?;

    println!("🚀 Running: {}", selected_tool.command);
    execute_command(&selected_tool.command)
}

pub fn handle_cheat(query: &Option<String>) -> Result<()> {
    let config = load_merged_config()?;

    let filtered: Vec<&CheatSheet> = if let Some(q) = query {
        let q_low = q.to_lowercase();
        config
            .cheats
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&q_low) || c.content.to_lowercase().contains(&q_low)
            })
            .collect()
    } else {
        config.cheats.iter().collect()
    };

    if filtered.len() == 1 {
        print_cheat(filtered[0]);
    } else if !filtered.is_empty() {
        let names: Vec<&String> = filtered.iter().map(|c| &c.name).collect();
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Input to filter, use arrow keys to navigate and Enter to select:")
            .items(&names)
            .interact()?;
        print_cheat(filtered[selection]);
    }

    Ok(())
}

fn print_cheat(cheat: &CheatSheet) {
    println!("\n✨ 【{}】", cheat.name);
    println!("{}\n", "-".repeat(cheat.name.len() + 6));
    println!("{}", cheat.content.trim());
}

pub fn handle_add(id: String, name: String, cmd: String) -> Result<()> {
    let config_dir = get_xdg_config_dir();
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("bb.toml");

    let mut config = if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        toml::from_str(&content)?
    } else {
        Config::default()
    };

    if config.tools.iter().any(|t| t.id == id) {
        anyhow::bail!(
            "ID '{}' already exists. Please choose a different ID.",
            id
        );
    }

    config.tools.push(Tool {
        id,
        name: name.clone(),
        command: cmd,
    });
    let toml_string = toml::to_string_pretty(&config)?;
    fs::write(&config_path, toml_string)?;

    println!("✅ Successfully added command '{}' to global config!", name);
    println!("File path: {:?}", config_path);
    Ok(())
}
