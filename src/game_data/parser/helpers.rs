use log::{error, info};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Data Objects which should be ignored
const REJECTED_DATA_FILE_NAMES: [&str; 7] = [
    "Any Move.json",
    "Potion.json",
    "Super Potion.json",
    "Hyper Potion.json",
    "Max Potion.json",
    "Full Heal.json",
    "Full Restore.json",
];

fn parse_file<T: DeserializeOwned>(file_path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let result: T = serde_json::from_str(&json_data)?;
    Ok(result)
}

pub fn parse_directory<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Vec<T> {
    let mut result = Vec::new();

    println!("Attempting to read directory: {:?}", path.as_ref());
    let entries = std::fs::read_dir(path).expect("Failed to read directory");
    for entry in entries.flatten() {
        if REJECTED_DATA_FILE_NAMES
            .iter()
            .any(|x| entry.path().ends_with(x))
        {
            info!("Skipping {:?}", entry.path());
            continue;
        }

        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "json") {
            match parse_file::<T>(file_path.to_str().expect("")) {
                Ok(parsed) => result.push(parsed),
                Err(err) => error!("Failed to parse file {:?}: {}", file_path, err),
            }
        }
    }

    result
}
