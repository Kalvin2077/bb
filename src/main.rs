mod cli;
mod config;
mod handlers;
mod history;
mod models;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let history_path = utils::get_xdg_config_dir().join("history.json");
    let mut history = history::load_history(&history_path)?;

    match cli.command {
        Some(cli::Commands::Run) | None => {
            handlers::handle_run(&mut history, &history_path)?;
        }
        Some(cli::Commands::Cheat { query }) => {
            handlers::handle_cheat(&query)?;
        }
        Some(cli::Commands::Add { id, name, command }) => {
            handlers::handle_add(id, name, command)?;
        }
    }

    Ok(())
}
