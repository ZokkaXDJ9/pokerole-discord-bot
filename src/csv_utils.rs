use std::env;
use std::path::Path;
use serde::de::DeserializeOwned;

pub fn load_csv<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Vec<T> {
    let path_ref = path.as_ref();
    println!("Attempting to open CSV file at: {:?}", path_ref);

    let mut results = Vec::new();

    let reader = csv::ReaderBuilder::new().has_headers(true).from_path(path_ref);

    for record in reader.expect("Failed to open CSV file").records().flatten() {
        let value: T = record.deserialize(None).expect("Unable to parse CSV row");
        results.push(value);
    }

    results
}
