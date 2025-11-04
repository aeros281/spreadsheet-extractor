use crate::{config::Config, format::Format, sheet_utils::write_page};

use super::Command;
use anyhow::Result;
use clap::Args;

/// Fetch google spreadsheet using range
#[derive(Args)]
pub struct PushSheet {
    // The input file
    input_file: String,
    // The id of the sheet
    sheet_id: String,
    // Tab name
    tab_name: String,
    // Output format
    #[arg(short, long, default_value = "csv")]
    format: Format,
}

impl Command for PushSheet {
    fn run(&self, config: &Config) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;

        rt.block_on(async {
            write_page(config, &self.sheet_id, &self.tab_name, &self.input_file).await
        })?;

        Ok(())
    }
}
