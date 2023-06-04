use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::de::DeserializeOwned;

use crate::pokerole_data::pokemon::PokerolePokemon;
use crate::pokerole_data::ability::PokeroleAbility;
use crate::pokerole_data::item::PokeroleItem;
use crate::pokerole_data::moves::PokeroleMove;
use crate::pokerole_data::nature::PokeroleNature;

pub struct PokeroleParseResult {
    pub abilities: Vec<PokeroleAbility>,
    pub items: Vec<PokeroleItem>,
    pub moves: Vec<PokeroleMove>,
    pub natures: Vec<PokeroleNature>,
    pub pokemon: Vec<PokerolePokemon>,
}

fn parse_file<T: DeserializeOwned>(file_path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let result: T = serde_json::from_str(&json_data)?;
    Ok(result)
}

fn parse_directory<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Vec<T> {
    let mut result = Vec::new();

    let entries = std::fs::read_dir(path).expect("Failed to read directory");
    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();

            if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "json") {
                match parse_file::<T>(file_path.to_str().expect("")) {
                    Ok(parsed) => result.push(parsed),
                    Err(err) => eprintln!("Failed to parse file {:?}: {}", file_path, err)
                }
            }
        }
    }

    result
}

pub fn parse(repo_path: &str) -> PokeroleParseResult {
    let mut items: Vec<PokeroleItem> = parse_directory(repo_path.to_owned() + "Version20/Items");
    items.extend(parse_directory(repo_path.to_owned() + "Homebrew/Items"));

    PokeroleParseResult {
        abilities: parse_directory(repo_path.to_owned() + "Version20/Abilities"),
        items,
        moves: parse_directory(repo_path.to_owned() + "Version20/Moves"),
        natures: parse_directory(repo_path.to_owned() + "Version20/Natures"),
        pokemon: parse_directory(repo_path.to_owned() + "Version20/Pokedex"),
    }
}
