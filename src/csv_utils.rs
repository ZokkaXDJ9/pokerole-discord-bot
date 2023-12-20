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
