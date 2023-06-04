use std::collections::HashMap;
use std::sync::Arc;
use crate::{pokemon_api_parser, pokerole_discord_py_csv_parser};
use crate::data::ability::Ability;
use crate::data::game_rule::GameRule;
use crate::data::item::Item;
use crate::data::nature::Nature;
use crate::data::pokemon::Pokemon;
use crate::data::pokerole_data;
use crate::data::r#move::Move;

use crate::pokerole_discord_py_csv_parser::{PokeStatus, PokeWeather};
use crate::pokemon_api_parser::{PokemonApiData};

/// Data which is stored and accessible in all command invocations
pub struct GameData {
    pub abilities: Arc<HashMap<String, Ability>>,
    pub ability_names: Arc<Vec<String>>,
    pub items: Arc<HashMap<String, Item>>,
    pub item_names: Arc<Vec<String>>,
    pub moves: Arc<HashMap<String, Move>>,
    pub move_names: Arc<Vec<String>>,
    pub natures: Arc<HashMap<String, Nature>>,
    pub nature_names: Arc<Vec<String>>,
    pub pokemon: Arc<HashMap<String, Pokemon>>,
    pub pokemon_names: Arc<Vec<String>>,
    pub status_effects: Arc<HashMap<String, PokeStatus>>,
    pub status_effects_names: Arc<Vec<String>>,
    pub weather: Arc<HashMap<String, PokeWeather>>,
    pub weather_names: Arc<Vec<String>>,
    pub pokemon_api_data: Arc<HashMap<String, PokemonApiData>>,
    pub rule_names: Arc<Vec<String>>,
    pub rules: Arc<HashMap<String, GameRule>>,
}

pub fn initialize_data() -> GameData {
    // TODO: Move these in to .env
    let pokerole_data_path = "/home/jacudibu/code/Pokerole-Data/";
    let custom_data_path = "/home/jacudibu/code/pokerole-discord-bot/custom_data/";

    let pokemon_api_data = pokemon_api_parser::parse_pokemon_api();
    let pokerole_data = pokerole_data::parser::parse(pokerole_data_path, custom_data_path);
    let pokerole_csv_data = pokerole_discord_py_csv_parser::parse("/home/jacudibu/code/pokerole-csv/");

    let mut rule_names = Vec::default();
    let mut rule_hash_map = HashMap::default();
    for x in GameRule::get_hardcoded_rules() {
        rule_names.push(x.name.clone());
        rule_hash_map.insert(x.name.to_lowercase(), x);
    }

    let mut move_names = Vec::default();
    let mut move_hash_map = HashMap::default();
    for x in pokerole_data.moves {
        move_names.push(x.name.clone());
        move_hash_map.insert(x.name.to_lowercase(), Move::new(x));
    }

    let mut nature_names = Vec::default();
    let mut nature_hash_map = HashMap::default();
    for x in pokerole_data.natures {
        nature_names.push(x.name.clone());
        nature_hash_map.insert(x.name.to_lowercase(), Nature::new(x));
    }

    let mut ability_names = Vec::default();
    let mut ability_hash_map = HashMap::default();
    for x in pokerole_data.abilities {
        ability_names.push(x.name.clone());
        ability_hash_map.insert(x.name.to_lowercase(), Ability::new(x));
    }

    let mut weather_names = Vec::default();
    let mut weather_hash_map = HashMap::default();
    for x in pokerole_csv_data.weather {
        weather_names.push(x.name.clone());
        weather_hash_map.insert(x.name.to_lowercase(), x);
    }

    let mut pokemon_names = Vec::default();
    let mut pokemon = HashMap::default();
    for x in pokerole_data.pokemon {
        if x.number == 0 {
            // Skip the egg!
            continue;
        }
        pokemon_names.push(x.name.clone());
        pokemon.insert(x.name.to_lowercase(), Pokemon::new(x, &pokemon_api_data));
    }

    let mut status_names = Vec::default();
    let mut status_hash_map = HashMap::default();
    for x in pokerole_csv_data.status_effects {
        status_names.push(x.name.clone());
        status_hash_map.insert(x.name.to_lowercase(), x);
    }

    let mut item_names = Vec::default();
    let mut item_hash_map = HashMap::default();
    for x in pokerole_data.items {
        item_names.push(x.name.clone());
        item_hash_map.insert(x.name.to_lowercase(), Item::new(x));
    }

    GameData {
        abilities: Arc::new(ability_hash_map),
        ability_names: Arc::new(ability_names),
        items: Arc::new(item_hash_map),
        item_names: Arc::new(item_names),
        moves: Arc::new(move_hash_map),
        move_names: Arc::new(move_names),
        natures: Arc::new(nature_hash_map),
        nature_names: Arc::new(nature_names),
        pokemon: Arc::new(pokemon),
        pokemon_names: Arc::new(pokemon_names),
        rules: Arc::new(rule_hash_map),
        rule_names: Arc::new(rule_names),
        status_effects: Arc::new(status_hash_map),
        status_effects_names: Arc::new(status_names),
        weather: Arc::new(weather_hash_map),
        weather_names: Arc::new(weather_names),
        pokemon_api_data: Arc::new(pokemon_api_data),
    }
}
