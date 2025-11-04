use serde_json::{Map, Value, json};


pub fn convert_to_json(data: &Vec<Vec<Value>>) -> Value {
    // Handle the edge case of empty data.
    if data.is_empty() {
        return json!([]); // Return an empty JSON array
    }

    // 1. Get the headers from the first row.
    let headers = &data[0];

    // 2. Get the data rows (everything *except* the first row).
    let rows = &data[1..];

    let mut json_array: Vec<Map<String, Value>> = vec![];

    // 3. Iterate over each data row.
    for row in rows {
        // Create a new JSON object (Map) for this row.
        let mut object_map = Map::new();

        // 4. Iterate over headers and cells in the current row *together*.
        // `zip` neatly pairs them up.
        for (header, cell) in headers.iter().zip(row.iter()) {
            // Insert into the map.
            // The `json!` macro converts the `cell` (String) into a `Value::String`.
            object_map.insert(header.as_str().unwrap().to_string(), json!(cell));
        }

        // Add the completed object to our array.
        // `.into()` converts the Map<String, Value> into a Value::Object.
        json_array.push(object_map.into());
    }

    // 5. Convert the Vec<Value> into a single Value::Array.
    json_array.into()
}
