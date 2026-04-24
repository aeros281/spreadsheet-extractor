use crate::{config::Config, format::Format, sheet_utils};

use super::Command;
use anyhow::Result;
use clap::Args;
use log::debug;

/// Push a CSV file to a google spreadsheet tab
#[derive(Args)]
pub struct PushSheet {
    // The input file
    input_file: String,
    // The id of the spreadsheet
    sheet_id: String,
    // The numeric GID of the sheet tab (visible in the URL as &gid=...)
    gid: i32,
    // Output format
    #[arg(short, long, default_value = "csv")]
    format: Format,
}

impl Command for PushSheet {
    fn run(&self, config: &Config) -> Result<()> {
        debug!(
            "push-sheet: input={} sheet_id={} gid={} format={:?}",
            self.input_file, self.sheet_id, self.gid, self.format
        );

        let rt = tokio::runtime::Runtime::new()?;

        rt.block_on(async {
            let sheet_name =
                sheet_utils::resolve_gid_to_name(config, &self.sheet_id, self.gid).await?;
            debug!("resolved gid {} → sheet name {:?}", self.gid, sheet_name);
            debug!("writing {:?} to sheet {:?}", self.input_file, sheet_name);
            sheet_utils::write_page(config, &self.sheet_id, &sheet_name, &self.input_file).await
        })?;

        Ok(())
    }
}
