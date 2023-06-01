use std::collections::{HashMap};
use serde::Deserialize;
use crate::load_csv;

/// version_groups.csv
#[derive(Debug, Deserialize)]
pub struct VersionGroups {
    id: u8,
    identifier: String,
    generation_id: u8,
    order: u8
}

/// pokemon_moves.csv
/// Contains info on what moves a pokemon can learn and how.
#[derive(Debug, Deserialize)]
pub struct PokemonMoves {
    pokemon_id: u16,
    version_group_id: u8,
    move_id: u16,
    pokemon_move_method_id: u8,
    level: u8,
    order: Option<u8>,
}

/// pokemon_move_methods.csv
/// Maps a move is acquired
#[derive(Debug, Deserialize)]
pub struct PokemonMoveMethods {
    id: u8,
    identifier: String,
}

/// pokemon_species_names.csv
/// Contains the name for regular Pokemon
#[derive(Debug, Deserialize)]
pub struct PokemonSpeciesNames {
    pokemon_species_id: u16,
    local_language_id: u8,
    name: String,
    genus: String,
}

/// pokemon_form_names.csv
/// Contains the names for regional Pokemon and other weird forms
#[derive(Debug, Deserialize)]
pub struct PokemonFormNames {
    pokemon_form_id: u16,
    local_language_id: u8,
    form_name: String,
    pokemon_name: Option<String>
}

/// move_names.csv
/// Contains the names for moves
#[derive(Debug, Deserialize)]
pub struct MoveNames {
    move_id: u16,
    local_language_id: u8,
    name: String
}

#[derive(Debug)]
pub struct MoveEntry {
    move_name: String,
    method: String,
    generation_id: u8,
}

#[derive(Debug)]
pub struct ParsedResult {
    pokemon_name: String,
    moves: Vec<MoveEntry>,
}

pub fn parse_pokemon_api() -> HashMap<String, Vec<MoveEntry>> {
    let english_language_id:u8 = 9;
    let version_groups: Vec<VersionGroups> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/version_groups.csv");
    let pokemon_moves: Vec<PokemonMoves> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_moves.csv");
    let pokemon_move_methods: Vec<PokemonMoveMethods> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_move_methods.csv");
    let pokemon_species_names: Vec<PokemonSpeciesNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_species_names.csv");
    let move_names: Vec<MoveNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/move_names.csv");
    let pokemon_form_names: Vec<PokemonFormNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_form_names.csv");

    let mut pokemon_id_to_name: HashMap<u16, String> = HashMap::default();
    for x in pokemon_species_names {
        if x.local_language_id != english_language_id {
            continue;
        }

        pokemon_id_to_name.insert(x.pokemon_species_id, x.name);
    }
    for x in pokemon_form_names {
        if x.local_language_id != english_language_id {
            continue;
        }

        if let Some(name) = x.pokemon_name {
            pokemon_id_to_name.insert(x.pokemon_form_id, name);
        }
    }

    let mut move_id_to_name: HashMap<u16, String> = HashMap::default();
    for x in move_names {
        if x.local_language_id != english_language_id {
            continue;
        }

        move_id_to_name.insert(x.move_id, x.name);
    }

    let mut method_id_to_name: HashMap<u8, String> = HashMap::default();
    for x in pokemon_move_methods {
        method_id_to_name.insert(x.id, x.identifier);
    }

    let mut version_group_id_to_generation_id: HashMap<u8, u8> = HashMap::default();
    for x in version_groups {
        version_group_id_to_generation_id.insert(x.id, x.generation_id);
    }

    let mut result: HashMap<String, Vec<MoveEntry>> = HashMap::default();
    for pokemon_move in pokemon_moves {
        if let Some(pokemon_name) = pokemon_id_to_name.get(&pokemon_move.pokemon_id) {
            if !result.contains_key(pokemon_name) {
                result.insert(pokemon_name.clone(), Vec::default());
            }

            let move_name_option = move_id_to_name.get(&pokemon_move.move_id);
            if move_name_option.is_none() {
                log::warn!("Unable to find a move name for {:?}", pokemon_move);
                continue;
            }
            let move_name = move_name_option.unwrap().clone();

            let move_entry = result.get_mut(pokemon_name).unwrap();
            if move_entry.iter().any(|x| x.move_name == move_name) {
                continue;
            }


            move_entry.push(MoveEntry {
                move_name,
                method: method_id_to_name.get(&pokemon_move.pokemon_move_method_id).expect("All learning method names should be set").clone(),
                generation_id: version_group_id_to_generation_id.get(&pokemon_move.version_group_id).expect("All generation ids should be set").clone(),
            })
        } else {
            log::warn!("unable to assign a pokemon to {:?}", pokemon_move);
        }
    }

    result
}
