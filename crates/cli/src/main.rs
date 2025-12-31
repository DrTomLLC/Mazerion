// Mazerion CLI - Precision brewing calculators

use clap::{Parser, Subcommand};

mod commands;
use commands::DbCommand;

#[derive(Debug, Parser)]
#[command(
    name = "mazerion",
    version,
    about = "Precision brewing calculators for beer, mead, wine, and cider",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Database operations
    #[command(visible_alias = "d", subcommand)]
    Db(DbCommand),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Db(cmd) => cmd.execute(),
    }
}