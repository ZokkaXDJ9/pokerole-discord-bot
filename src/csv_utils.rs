use std::path::Path;
use serde::de::DeserializeOwned;

pub fn load_csv<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Vec<T> {
    let mut results = Vec::new();

    let reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path);

    for result in reader.expect("").records() {
        if let Ok(record) = result {
            let value: T = record.deserialize(None).expect("Unable to parse csv row");
            results.push(value);
        };
    }

    return results;
}

pub fn load_csv_with_custom_headers<P: AsRef<Path>, T: DeserializeOwned>(path: P, headers: Vec<&str>) -> Vec<T> {
    let mut result = Vec::new();
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);

    let header_records = csv::StringRecord::from(headers);
    for row in reader.expect("").records() {
        if let Ok(record) = row {
            let item : T = record.deserialize(Some(&header_records)).expect("Csv should be parsable!");
            result.push(item);
        };
    }

    return result;
}
