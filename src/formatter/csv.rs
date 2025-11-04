use std::io;

use serde_json::Value;

pub fn print_csv<W: io::Write>(val: &Vec<Vec<Value>>, writer: W) -> csv::Result<()> {
    let mut wtr = csv::Writer::from_writer(writer);

    for row in val {
        let value_rows = row.into_iter().map(|v| v.as_str().unwrap());
        wtr.write_record(value_rows).unwrap();
    }

    wtr.flush().unwrap();
    Ok(())
}

