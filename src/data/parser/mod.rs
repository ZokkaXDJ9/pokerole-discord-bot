pub(in crate::data) mod custom_data;
pub(in crate::data) mod helpers;

use std::collections::HashMap;
use std::sync::Arc;
use log::{info};
use crate::data::ability::Ability;
use crate::data::rule::Rule;
use crate::data::item::Item;
use crate::data::nature::Nature;
use crate::data::parser::custom_data::parser::CustomDataParseResult;
use crate::data::pokemon::Pokemon;
use crate::data::pokemon_api::pokemon_api_parser;
use crate::data::pokemon_api::pokemon_api_parser::PokemonApiData;
use crate::data::pokerole_data;
use crate::data::pokerole_data::parser::PokeroleParseResult;
use crate::data::pokerole_discord_py_data::pokerole_discord_py_csv_parser;
use crate::data::pokerole_discord_py_data::pokerole_discord_py_csv_parser::RawPokeroleDiscordPyCsvData;
use crate::data::r#move::Move;
use crate::data::status_effect::StatusEffect;
use crate::data::weather::Weather;
use crate::game_data::GameData;

pub fn initialize_data() -> GameData {
    // TODO: Move these in to .env
    let pokerole_data_path = "/home/jacudibu/code/Pokerole-Data/";
    let csv_data_path = "/home/jacudibu/code/pokerole-csv/";
    let custom_data_path = "/home/jacudibu/code/pokerole-discord-bot/custom_data/";

    let pokemon_api_data = pokemon_api_parser::parse_pokemon_api();
    let pokerole_data = pokerole_data::parser::parse(pokerole_data_path);
    let pokerole_csv_data = pokerole_discord_py_csv_parser::parse(csv_data_path);
    let custom_data = custom_data::parser::parse(custom_data_path);

    let (rule_names, rule_hash_map) = parse_rules();
    let (move_names, move_hash_map) = parse_moves(&pokerole_data);
    let (nature_names, nature_hash_map) = parse_natures(&pokerole_data);
    let (ability_names, ability_hash_map) = parse_abilities(&pokerole_data);
    let (weather_names, weather_hash_map) = parse_weather(&pokerole_csv_data);
    let (pokemon_names, pokemon_hash_map) = parse_pokemon(&pokemon_api_data, &pokerole_data, &custom_data);
    let (status_names, status_hash_map) = parse_status_effects(pokerole_csv_data);
    let (item_names, item_hash_map) = parse_items(pokerole_data);

    GameData {
        abilities: Arc::new(ability_hash_map),
        ability_names: Arc::new(ability_names),
        items: Arc::new(item_hash_map),
        item_names: Arc::new(item_names),
        moves: Arc::new(move_hash_map),
        move_names: Arc::new(move_names),
        natures: Arc::new(nature_hash_map),
        nature_names: Arc::new(nature_names),
        pokemon: Arc::new(pokemon_hash_map),
        pokemon_names: Arc::new(pokemon_names),
        rules: Arc::new(rule_hash_map),
        rule_names: Arc::new(rule_names),
        status_effects: Arc::new(status_hash_map),
        status_effects_names: Arc::new(status_names),
        weather: Arc::new(weather_hash_map),
        weather_names: Arc::new(weather_names),
    }
}

fn parse_items(pokerole_data: PokeroleParseResult) -> (Vec<String>, HashMap<String, Item>) {
    let mut item_names = Vec::default();
    let mut item_hash_map = HashMap::default();
    for x in pokerole_data.items {
        item_names.push(x.name.clone());
        item_hash_map.insert(x.name.to_lowercase(), Item::new(x));
    }
    (item_names, item_hash_map)
}

fn parse_status_effects(pokerole_csv_data: RawPokeroleDiscordPyCsvData) -> (Vec<String>, HashMap<String, StatusEffect>) {
    let mut status_names = Vec::default();
    let mut status_hash_map = HashMap::default();
    for x in pokerole_csv_data.status_effects {
        status_names.push(x.name.clone());
        status_hash_map.insert(x.name.to_lowercase(), StatusEffect::new(x));
    }
    (status_names, status_hash_map)
}

fn parse_pokemon(pokemon_api_data: &HashMap<String, PokemonApiData>, pokerole_data: &PokeroleParseResult, custom_data: &CustomDataParseResult) -> (Vec<String>, HashMap<String, Pokemon>) {
    let mut pokemon_names = Vec::default();
    let mut pokemon = HashMap::default();
    for x in &pokerole_data.pokemon {
        if x.number == 0 {
            // Skip the egg!
            continue;
        }
        pokemon_names.push(x.name.clone());
        pokemon.insert(x.name.to_lowercase(), Pokemon::new(x, &pokemon_api_data));
    }

    for x in &custom_data.pokemon {
        if pokemon.contains_key(&x.name) {
            info!("Overriding {}", x.name)
        } else {
            pokemon_names.push(x.name.clone());
        }

        pokemon.insert(x.name.to_lowercase(), Pokemon::from_custom_data(x, &pokemon_api_data));
    }

    (pokemon_names, pokemon)
}

fn parse_weather(pokerole_csv_data: &RawPokeroleDiscordPyCsvData) -> (Vec<String>, HashMap<String, Weather>) {
    let mut weather_names = Vec::default();
    let mut weather_hash_map = HashMap::default();
    for x in &pokerole_csv_data.weather {
        weather_names.push(x.name.clone());
        weather_hash_map.insert(x.name.to_lowercase(), Weather::new(x));
    }
    (weather_names, weather_hash_map)
}

fn parse_abilities(pokerole_data: &PokeroleParseResult) -> (Vec<String>, HashMap<String, Ability>) {
    let mut ability_names = Vec::default();
    let mut ability_hash_map = HashMap::default();
    for x in &pokerole_data.abilities {
        ability_names.push(x.name.clone());
        ability_hash_map.insert(x.name.to_lowercase(), Ability::new(x));
    }
    (ability_names, ability_hash_map)
}

fn parse_natures(pokerole_data: &PokeroleParseResult) -> (Vec<String>, HashMap<String, Nature>) {
    let mut nature_names = Vec::default();
    let mut nature_hash_map = HashMap::default();
    for x in &pokerole_data.natures {
        nature_names.push(x.name.clone());
        nature_hash_map.insert(x.name.to_lowercase(), Nature::new(x));
    }
    (nature_names, nature_hash_map)
}

fn parse_moves(pokerole_data: &PokeroleParseResult) -> (Vec<String>, HashMap<String, Move>) {
    let mut move_names = Vec::default();
    let mut move_hash_map = HashMap::default();
    for x in &pokerole_data.moves {
        move_names.push(x.name.clone());
        move_hash_map.insert(x.name.to_lowercase(), Move::new(x));
    }
    (move_names, move_hash_map)
}

fn parse_rules() -> (Vec<String>, HashMap<String, Rule>) {
    let mut rule_names = Vec::default();
    let mut rule_hash_map = HashMap::default();
    for x in Rule::get_hardcoded_rules() {
        rule_names.push(x.name.clone());
        rule_hash_map.insert(x.name.to_lowercase(), x);
    }
    (rule_names, rule_hash_map)
}
