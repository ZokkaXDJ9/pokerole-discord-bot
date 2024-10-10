pub(in crate::game_data) mod custom_data;
pub(in crate::game_data) mod helpers;

use crate::game_data::ability::Ability;
use crate::game_data::item::Item;
use crate::game_data::nature::Nature;
use crate::game_data::parser::custom_data::parser::CustomDataParseResult;
use crate::game_data::pokemon::{ApiIssueType, DataSource, LearnablePokemonMoves, Pokemon};
use crate::game_data::pokemon_api::pokemon_api_parser::PokemonApiData;
use crate::game_data::pokemon_api::{pokemon_api_parser, PokemonApiId};
use crate::game_data::pokerole_data;
use crate::game_data::pokerole_data::parser::PokeroleParseResult;
use crate::game_data::potion::Potion;
use crate::game_data::r#move::Move;
use crate::game_data::parser::custom_data::custom_zmove::CustomZMove;
use crate::game_data::zmove::ZMove;
use crate::game_data::rule::Rule;
use crate::game_data::status_effect::StatusEffect;
use crate::game_data::weather::Weather;
use crate::game_data::GameData;
use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn initialize_data() -> GameData {
    let pokerole_api_path = std::env::var("POKEMON_API").expect("missing POKEMON_API");
    let pokerole_data_path = std::env::var("POKEROLE_DATA").expect("missing POKEROLE_DATA");
    let custom_data_path = std::env::var("CUSTOM_DATA").expect("missing CUSTOM_DATA");

    let type_efficiency = pokemon_api_parser::parse_type_efficacy(pokerole_api_path.clone());
    let pokemon_api_data = pokemon_api_parser::parse_pokemon_api(pokerole_api_path);
    let pokerole_data = pokerole_data::parser::parse(&pokerole_data_path);
    let custom_data = custom_data::parser::parse(&custom_data_path);

    let (rule_names, rule_hash_map) = parse_rules(&custom_data);
    let (move_names, move_hash_map) = parse_moves(&pokerole_data, &custom_data);
    let (z_move_names, z_move_hash_map) = parse_z_moves(&custom_data);
    let (nature_names, nature_hash_map) = parse_natures(&pokerole_data);
    let (ability_names, ability_hash_map) = parse_abilities(&pokerole_data, &custom_data);
    let (weather_names, weather_hash_map) = parse_weather(&custom_data);
    let (pokemon_names, pokemon_hash_map, pokemon_by_api_id_hash_map) =
        parse_pokemon(&pokemon_api_data, &pokerole_data, &custom_data);
    let (status_names, status_hash_map) = parse_status_effects(&custom_data);
    let (item_names, item_hash_map) = parse_items(pokerole_data, &custom_data);
    let (potion_names, potion_hash_map) = parse_potions(&custom_data);
    GameData {
        abilities: Arc::new(ability_hash_map),
        ability_names: Arc::new(ability_names),
        potions: Arc::new(potion_hash_map),
        potion_names: Arc::new(potion_names),
        items: Arc::new(item_hash_map),
        item_names: Arc::new(item_names),
        moves: Arc::new(move_hash_map),
        move_names: Arc::new(move_names),
        z_moves: Arc::new(z_move_hash_map),
        z_move_names: Arc::new(z_move_names),
        natures: Arc::new(nature_hash_map),
        nature_names: Arc::new(nature_names),
        pokemon: Arc::new(pokemon_hash_map),
        pokemon_by_api_id: Arc::new(pokemon_by_api_id_hash_map),
        pokemon_names: Arc::new(pokemon_names),
        rules: Arc::new(rule_hash_map),
        rule_names: Arc::new(rule_names),
        status_effects: Arc::new(status_hash_map),
        status_effects_names: Arc::new(status_names),
        weather: Arc::new(weather_hash_map),
        weather_names: Arc::new(weather_names),
        type_efficiency: Arc::new(type_efficiency),
    }
}

fn parse_items(
    pokerole_data: PokeroleParseResult,
    custom_data: &CustomDataParseResult,
) -> (Vec<String>, HashMap<String, Item>) {
    let mut item_names = Vec::default();
    let mut item_hash_map = HashMap::default();
    for x in pokerole_data.items {
        item_names.push(x.name.clone());
        item_hash_map.insert(x.name.to_lowercase(), Item::from_pokerole(x));
    }

    for x in &custom_data.items {
        if item_names.contains(&x.name) {
            info!("Overriding {}", x.name);
        } else {
            item_names.push(x.name.clone());
        }

        item_hash_map.insert(x.name.to_lowercase(), Item::from_custom_data(x));
    }

    (item_names, item_hash_map)
}

fn parse_potions(custom_data: &CustomDataParseResult) -> (Vec<String>, HashMap<String, Potion>) {
    let mut potion_names = Vec::default();
    let mut potion_hash_map = HashMap::default();
    for x in &custom_data.potions {
        if potion_names.contains(&x.name) {
            info!("Overriding {}", x.name);
        } else {
            potion_names.push(x.name.clone());
        }

        potion_hash_map.insert(x.name.to_lowercase(), Potion::from_custom_data(x));
    }

    (potion_names, potion_hash_map)
}

fn parse_status_effects(
    custom_data: &CustomDataParseResult,
) -> (Vec<String>, HashMap<String, StatusEffect>) {
    let mut status_names = Vec::default();
    let mut status_hash_map = HashMap::default();
    for x in &custom_data.status_effects {
        status_names.push(x.name.clone());
        status_hash_map.insert(x.name.to_lowercase(), StatusEffect::from_custom_data(x));
    }

    (status_names, status_hash_map)
}

fn parse_pokemon(
    pokemon_api_data: &HashMap<String, PokemonApiData>,
    pokerole_data: &PokeroleParseResult,
    custom_data: &CustomDataParseResult,
) -> (
    Vec<String>,
    HashMap<String, Pokemon>,
    HashMap<PokemonApiId, Pokemon>,
) {
    let mut pokemon_names = Vec::default();
    let mut parsed_pokemon = Vec::default();
    let mut learnable_moves_by_api_id: HashMap<PokemonApiId, LearnablePokemonMoves> =
        HashMap::default();

    for x in &pokerole_data.pokemon {
        if x.number == 0 {
            // Skip the egg!
            continue;
        }
        if x.name == "Meowstic" {
            // Custom overrides for male & female variations
            continue;
        }
        if x.name.contains("Mega ") {
            // Our Mega Evolutions are fully customized.
            continue;
        }
        pokemon_names.push(x.name.clone());

        let pokemon = Pokemon::from_pokerole_data(x, pokemon_api_data);
        learnable_moves_by_api_id.insert(pokemon.poke_api_id, pokemon.moves.clone());
        parsed_pokemon.push(pokemon);
    }

    for x in &custom_data.pokemon {
        if let Some(pokemon) = Pokemon::from_custom_data(x, pokemon_api_data) {
            if pokemon_names.contains(&x.name) {
                info!("Overriding {}", x.name);
            } else {
                pokemon_names.push(x.name.clone());
            }

            learnable_moves_by_api_id.insert(pokemon.poke_api_id, pokemon.moves.clone());
            parsed_pokemon.push(pokemon);
        } else {
            warn!("Was unable to find a pokeapi implementation for {}", x.name)
        }
    }

    parsed_pokemon.sort_by(|a, b| a.poke_api_id.0.cmp(&b.poke_api_id.0));
    for x in &mut parsed_pokemon {
        if let Some(pre_evo_id) = x.evolves_from {
            let opt = learnable_moves_by_api_id.get(&pre_evo_id);
            if let Some(base_moves) = opt {
                x.add_pre_evo_moves(base_moves);
            } else {
                warn!(
                    "No moves found for {}'s pre-evo ID {}",
                    x.name, pre_evo_id.0
                )
            }
        }

        learnable_moves_by_api_id.insert(x.poke_api_id, x.moves.clone());
    }

    let mut pokemon_hash_map = HashMap::default();
    for x in &parsed_pokemon {
        pokemon_hash_map.insert(x.name.to_lowercase(), x.clone());
    }

    let mut pokemon_by_api_id_hash_map: HashMap<PokemonApiId, Pokemon> = HashMap::default();
    for x in parsed_pokemon {
        if let Some(already_added_poke) = pokemon_by_api_id_hash_map.get(&x.poke_api_id) {
            if x.data_source != DataSource::Custom
                && !x.name.starts_with("Rotom")
                && (already_added_poke.api_issue.is_none()
                    || already_added_poke.api_issue.unwrap() != ApiIssueType::IsLegendary)
            {
                error!(
                    "Duplicate IDs discovered: {}({}) <-> {}({})",
                    &already_added_poke.name,
                    &already_added_poke.poke_api_id.0,
                    &x.name,
                    &x.poke_api_id.0
                );
            }
        }
        pokemon_by_api_id_hash_map.insert(x.poke_api_id, x);
    }

    (pokemon_names, pokemon_hash_map, pokemon_by_api_id_hash_map)
}

fn parse_moves(
    pokerole_data: &PokeroleParseResult,
    custom_data: &CustomDataParseResult,
) -> (Vec<String>, HashMap<String, Move>) {
    let mut move_names = Vec::default();
    let mut move_hash_map = HashMap::default();
    for x in &pokerole_data.moves {
        move_names.push(x.name.clone());
        move_hash_map.insert(x.name.to_lowercase(), Move::from_pokerole(x));
    }

    for x in &custom_data.moves {
        if move_names.contains(&x.name) {
            info!("Overriding {}", x.name);
        } else {
            move_names.push(x.name.clone());
        }

        move_hash_map.insert(x.name.to_lowercase(), Move::from_custom(x));
    }

    (move_names, move_hash_map)
}

fn parse_z_moves(custom_data: &CustomDataParseResult) -> (Vec<String>, HashMap<String, ZMove>) {
    let mut z_move_names = Vec::new();
    let mut z_move_hash_map = HashMap::new();

    for raw_z_move in &custom_data.z_moves {
        let z_move = ZMove::from_custom(raw_z_move);
        z_move_names.push(z_move.name.clone());
        z_move_hash_map.insert(z_move.name.to_lowercase(), z_move);
    }

    (z_move_names, z_move_hash_map)
}

fn parse_weather(custom_data: &CustomDataParseResult) -> (Vec<String>, HashMap<String, Weather>) {
    let mut weather_names = Vec::default();
    let mut weather_hash_map = HashMap::default();
    for x in &custom_data.weather {
        weather_names.push(x.name.clone());
        weather_hash_map.insert(x.name.to_lowercase(), Weather::from_custom(x));
    }

    (weather_names, weather_hash_map)
}

fn parse_abilities(
    pokerole_data: &PokeroleParseResult,
    custom_data: &CustomDataParseResult,
) -> (Vec<String>, HashMap<String, Ability>) {
    let mut ability_names = Vec::default();
    let mut ability_hash_map = HashMap::default();
    for x in &pokerole_data.abilities {
        ability_names.push(x.name.clone());
        ability_hash_map.insert(x.name.to_lowercase(), Ability::from_pokerole(x));
    }

    for x in &custom_data.abilities {
        if ability_names.contains(&x.name) {
            info!("Overriding {}", x.name);
        } else {
            ability_names.push(x.name.clone());
        }

        ability_hash_map.insert(x.name.to_lowercase(), Ability::from_custom(x));
    }

    (ability_names, ability_hash_map)
}

fn parse_natures(pokerole_data: &PokeroleParseResult) -> (Vec<String>, HashMap<String, Nature>) {
    let mut nature_names = Vec::default();
    let mut nature_hash_map = HashMap::default();
    for x in &pokerole_data.natures {
        nature_names.push(x.name.clone());
        nature_hash_map.insert(x.name.to_lowercase(), Nature::from_pokerole(x));
    }
    (nature_names, nature_hash_map)
}

fn parse_rules(custom_data: &CustomDataParseResult) -> (Vec<String>, HashMap<String, Rule>) {
    let mut rule_names = Vec::default();
    let mut rule_hash_map = HashMap::default();
    for x in &custom_data.rules {
        rule_names.push(x.name.clone());
        rule_hash_map.insert(x.name.to_lowercase(), x.clone());
    }
    (rule_names, rule_hash_map)
}
