mod commands;
mod config;
mod format;
mod formatter;
mod reader;
mod sheet_utils;

use anyhow::Result;
use clap::{Parser, command};
use commands::*;
use config::Config;
use tracing::trace;
use tracing_subscriber::filter::LevelFilter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a config file
    #[arg(short, long)]
    config: Option<String>,

    // log level
    #[arg(short, long, default_value = "warn")]
    log_level: LevelFilter,

    #[command(subcommand)]
    commands: Commands,
}

// List the names of your sub commands here.
register_commands! {
    ShowConfig
    FetchSheet
    PushSheet
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(cli.log_level)
        .with_target(true)
        .init();

    trace!("Trying to init crypto default provider");
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    trace!("Finish init crypto default provider");

    trace!("Parsing configuration");
    let cfg = Config::parse(cli.config)?;
    trace!("Finish parsing configuration");

    trace!("Run subcommand");
    cli.commands.run(&cfg)?;
    trace!("Finish running subcommand");

    Ok(())
}
