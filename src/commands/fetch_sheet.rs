use crate::{
    config::Config,
    format::Format,
    formatter::{
        csv::print_csv,
        json::{convert_to_json, print_json},
        table::print_table,
    },
    sheet_utils,
};

use super::Command;
use anyhow::{Result, anyhow};
use clap::Args;
use log::{debug, trace, warn};
use serde_json::Value;

/// Fetch google spreadsheet using range
#[derive(Args)]
pub struct FetchSheet {
    // The id of the spreadsheet
    sheet_id: String,
    // The numeric GID of the sheet tab (visible in the URL as &gid=...)
    gid: i32,
    // The cell range, e.g. G4:Q7 (omit the header row when --headers is used)
    range: String,
    // Output format
    #[arg(short, long, default_value = "json")]
    format: Format,
    /// Columns (by letter, e.g. "A,C") whose empty cells inherit the previous row's value
    #[arg(long, value_delimiter = ',')]
    repeat_columns: Vec<String>,
    /// Column-to-header mapping, e.g. "A:Name,C:Age". Only listed columns are included;
    /// the range must not contain a header row when this is set.
    #[arg(long)]
    headers: Option<String>,
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

fn range_col_offset(range: &str) -> Result<usize> {
    let col_str: String = range.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    if col_str.is_empty() {
        return Err(anyhow!("cannot parse start column from range: {range}"));
    }
    column_letter_to_index(&col_str)
}

fn parse_headers(s: &str, col_offset: usize) -> Result<Vec<(usize, String)>> {
    let mut cols = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (col_letter, header_name) = part
            .split_once(':')
            .ok_or_else(|| anyhow!("invalid header spec '{part}', expected format COL:Name"))?;
        let abs_idx = column_letter_to_index(col_letter)?;
        let idx = abs_idx.checked_sub(col_offset).ok_or_else(|| {
            anyhow!("column '{col_letter}' is before the range start")
        })?;
        cols.push((idx, header_name.to_string()));
    }
    cols.sort_by_key(|(idx, _)| *idx);
    Ok(cols)
}

fn apply_headers(rows: &[Vec<Value>], headers: &[(usize, String)]) -> Vec<Vec<Value>> {
    let mut result = Vec::with_capacity(rows.len() + 1);
    result.push(
        headers
            .iter()
            .map(|(_, name)| Value::String(name.clone()))
            .collect(),
    );
    for row in rows {
        result.push(
            headers
                .iter()
                .map(|(idx, _)| row.get(*idx).cloned().unwrap_or(Value::String(String::new())))
                .collect(),
        );
    }
    result
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
        debug!(
            "fetch-sheet: sheet_id={} gid={} range={} format={:?}",
            self.sheet_id, self.gid, self.range, self.format
        );

        let rt = tokio::runtime::Runtime::new()?;

        let repeat_cols: Vec<usize> = self
            .repeat_columns
            .iter()
            .map(|c| column_letter_to_index(c))
            .collect::<Result<_>>()?;

        if !repeat_cols.is_empty() {
            trace!("repeat columns (0-indexed): {:?}", repeat_cols);
        }

        let (_, res) = rt.block_on(async {
            let sheet_name =
                sheet_utils::resolve_gid_to_name(config, &self.sheet_id, self.gid).await?;
            debug!("resolved gid {} → sheet name {:?}", self.gid, sheet_name);
            let full_range = sheet_utils::a1_range(&sheet_name, &self.range);
            debug!("fetching range: {full_range}");
            sheet_utils::get_sheet_data(config, &self.sheet_id, &full_range)
                .await
                .map_err(anyhow::Error::from)
        })?;

        match res.values {
            None => {
                warn!("sheet returned no values for the requested range");
            }
            Some(mut val) => {
                debug!("received {} row(s)", val.len());

                if !repeat_cols.is_empty() {
                    trace!("filling repeated columns");
                    fill_repeated_columns(&mut val, &repeat_cols);
                }

                let data = if let Some(spec) = &self.headers {
                    let col_offset = range_col_offset(&self.range)?;
                    trace!("parsing headers spec={spec:?} col_offset={col_offset}");
                    let col_headers = parse_headers(spec, col_offset)?;
                    debug!("applying {} header mapping(s)", col_headers.len());
                    apply_headers(&val, &col_headers)
                } else {
                    val
                };

                match self.format {
                    Format::Json => {
                        let parsed_data = convert_to_json(&data);
                        print_json(&mut std::io::stdout(), &parsed_data).unwrap();
                    }
                    Format::Csv => {
                        print_csv(&data, std::io::stdout()).unwrap();
                    }
                    Format::Table => {
                        print_table(&data, std::io::stdout()).unwrap();
                    }
                }
            }
        }
        Ok(())
    }
}
