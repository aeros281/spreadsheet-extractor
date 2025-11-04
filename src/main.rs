mod commands;
mod config;
mod format;
mod formatter;
mod sheet_utils;

use anyhow::Result;
use clap::{Parser, command};
use commands::*;
use config::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a config file
    #[arg(short, long)]
    config: Option<String>,

    #[command(subcommand)]
    commands: Commands,
}

// List the names of your sub commands here.
register_commands! {
    FetchSheet
}

fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let cli = Cli::parse();

    let cfg = Config::parse(cli.config)?;

    cli.commands.run(&cfg)?;

    Ok(())
}
