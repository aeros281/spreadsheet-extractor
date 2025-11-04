use crate::config::Config;

use super::Command;
use anyhow::Result;
use clap::Args;

/// Show the current configuration
#[derive(Args)]
pub struct ShowConfig {}

impl Command for ShowConfig {
    fn run(&self, config: &Config) -> Result<()> {
        println!("========== Configuration ==========");
        println!("config path: {:?}", Config::get_cfgdir().unwrap());
        println!("client_secret_path = {}", config.google.client_secret_path);
        println!("token_storage_path = {}", config.google.token_storage_path);

        Ok(())
    }
}
