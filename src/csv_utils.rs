use serde::de::DeserializeOwned;
use std::path::Path;

pub fn load_csv<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Vec<T> {
    let mut results = Vec::new();

    let reader = csv::ReaderBuilder::new().has_headers(true).from_path(path);

    for record in reader.expect("").records().flatten() {
        let value: T = record.deserialize(None).expect("Unable to parse csv row");
        results.push(value);
    }

    results
}

pub fn load_csv_with_custom_headers<P: AsRef<Path>, T: DeserializeOwned>(
    path: P,
    headers: Vec<&str>,
) -> Vec<T> {
    let mut result = Vec::new();
    let reader = csv::ReaderBuilder::new().has_headers(false).from_path(path);

    let header_records = csv::StringRecord::from(headers);
    for record in reader.expect("").records().flatten() {
        let item: T = record
            .deserialize(Some(&header_records))
            .expect("Csv should be parsable!");
        result.push(item);
    }

    result
}
