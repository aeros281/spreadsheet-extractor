use std::io;

use comfy_table::{ContentArrangement, Table};
use serde_json::Value;

pub fn print_table<W: io::Write>(val: &[Vec<Value>], mut writer: W) -> io::Result<()> {
    if val.is_empty() {
        return Ok(());
    }

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let mut rows = val.iter();

    if let Some(header) = rows.next() {
        table.set_header(header.iter().map(|v| v.as_str().unwrap_or("")));
    }

    for row in rows {
        table.add_row(row.iter().map(|v| v.as_str().unwrap_or("")));
    }

    writeln!(writer, "{table}")
}
