use std::error::Error;

use csv::Reader;
use indexmap::IndexMap;
use serde_json::Value;

#[allow(dead_code)]
pub fn read_csv_to_map(file_path: &str) -> Result<Vec<IndexMap<String, Value>>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let string_records: Vec<IndexMap<String, String>> =
        rdr.deserialize().collect::<Result<Vec<_>, _>>()?;

    let final_records = string_records
        .into_iter()
        .map(|record| {
            record
                .into_iter()
                .map(|(key, value)| (key, Value::String(value)))
                .collect()
        })
        .collect();

    Ok(final_records)
}
