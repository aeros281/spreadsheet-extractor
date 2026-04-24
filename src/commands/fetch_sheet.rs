use crate::{
    config::Config,
    format::Format,
    formatter::{
        csv::print_csv,
        json::{convert_to_json, print_json},
    },
    sheet_utils,
};

use super::Command;
use anyhow::{Result, anyhow};
use clap::Args;
use serde_json::Value;

/// Fetch google spreadsheet using range
#[derive(Args)]
pub struct FetchSheet {
    // The id of the spreadsheet
    sheet_id: String,
    // The numeric GID of the sheet tab (visible in the URL as &gid=...)
    gid: i32,
    // The cell range, e.g. G4:Q7
    range: String,
    // Output format
    #[arg(short, long, default_value = "json")]
    format: Format,
    /// Columns (by letter, e.g. "A,C") whose empty cells inherit the previous row's value
    #[arg(long, value_delimiter = ',')]
    repeat_columns: Vec<String>,
}

fn column_letter_to_index(s: &str) -> Result<usize> {
    let val = s.trim();
    if val.is_empty() {
        return Err(anyhow!("empty column letter"));
    }
    let mut idx: usize = 0;
    for ch in val.chars() {
        let c = ch.to_ascii_uppercase();
        if !c.is_ascii_uppercase() {
            return Err(anyhow!("invalid column letter: {val}"));
        }
        idx = idx * 26 + (c as usize - 'A' as usize + 1);
    }
    Ok(idx - 1)
}

fn fill_repeated_columns(values: &mut [Vec<Value>], cols: &[usize]) {
    let mut last: std::collections::HashMap<usize, Value> = std::collections::HashMap::new();
    for row in values.iter_mut() {
        for &col in cols {
            let is_empty = match row.get(col) {
                None => true,
                Some(Value::Null) => true,
                Some(Value::String(s)) => s.is_empty(),
                _ => false,
            };
            if is_empty {
                if let Some(prev) = last.get(&col) {
                    if row.len() <= col {
                        row.resize(col + 1, Value::String(String::new()));
                    }
                    row[col] = prev.clone();
                }
            } else {
                last.insert(col, row[col].clone());
            }
        }
    }
}

impl Command for FetchSheet {
    fn run(&self, config: &Config) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;

        let repeat_cols: Vec<usize> = self
            .repeat_columns
            .iter()
            .map(|c| column_letter_to_index(c))
            .collect::<Result<_>>()?;

        let (_, res) = rt.block_on(async {
            let sheet_name =
                sheet_utils::resolve_gid_to_name(config, &self.sheet_id, self.gid).await?;
            let full_range = sheet_utils::a1_range(&sheet_name, &self.range);
            sheet_utils::get_sheet_data(config, &self.sheet_id, &full_range)
                .await
                .map_err(anyhow::Error::from)
        })?;

        if let Some(mut val) = res.values {
            if !repeat_cols.is_empty() {
                fill_repeated_columns(&mut val, &repeat_cols);
            }
            match self.format {
                Format::Json => {
                    let parsed_data = convert_to_json(&val);
                    print_json(&mut std::io::stdout(), &parsed_data).unwrap();
                }
                Format::Csv => {
                    print_csv(&val, std::io::stdout()).unwrap();
                }
                Format::Table => {
                    println!("not support table format right now")
                }
            }
        }
        Ok(())
    }
}
