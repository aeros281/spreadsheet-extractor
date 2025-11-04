use std::io;

use crate::{config::Config, format::Format, formatter::json::convert_to_json, sheet_utils};

use super::Command;
use anyhow::Result;
use clap::Args;

/// Fetch google spreadsheet using range
#[derive(Args)]
pub struct FetchSheet {
    // The id of the sheet
    sheet_id: String,
    // The range of the spreadsheet
    range: String,
    // Output format
    #[arg(short, long, default_value = "json")]
    format: Format,
}

impl Command for FetchSheet {
    fn run(&self, config: &Config) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;

        let result = rt.block_on(async {
            sheet_utils::get_sheet_data(&config, &self.sheet_id, &self.range).await
        });

        result.map_err(anyhow::Error::from).map(|(_, res)| {
            if let Some(val) = res.values {
                match self.format {
                   Format::Json => {
                        let parsed_data = convert_to_json(&val);
                       println!("{}", serde_json::to_string_pretty(&parsed_data).unwrap());
                   } 

                   Format::CSV => {
                       let mut wtr = csv::Writer::from_writer(io::stdout());

                       for row in &val {
                           let value_rows = row.into_iter().map(|v| v.as_str().unwrap());
                           wtr.write_record(value_rows).unwrap();
                       }

                       wtr.flush().unwrap();
                   }
                   }
            }
            ()
        })
    }
}
