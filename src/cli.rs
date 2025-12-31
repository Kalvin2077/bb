use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bb", about = "Bash Buddy - CLI assistant", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Run,
    Cheat {
        query: Option<String>,
    },
    Add {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        command: String,
    },
}
