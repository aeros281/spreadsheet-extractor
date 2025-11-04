#[macro_use]
extern crate log;

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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a config file
    #[arg(short, long)]
    config: Option<String>,

    // log level
    #[arg(short, long, default_value = "warn")]
    log_level: log::LevelFilter,

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
    let mut clog = colog::default_builder();
    clog.filter(None, cli.log_level);
    clog.init();

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
