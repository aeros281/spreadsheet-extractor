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

        if let Some(google_config) = &config.google {
            println!("client_secret_path = {}", google_config.client_secret_path);
            println!("token_storage_path = {}", google_config.token_storage_path);
        } else {
            println!("No google configuration presented");
        }

        Ok(())
    }
}
