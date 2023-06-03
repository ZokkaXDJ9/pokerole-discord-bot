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

/// pokemon.csv
/// Contains base data about pokemon, such as height and weight. Pretty much just height and weight.
#[derive(Debug, Deserialize)]
pub struct Pokemon {
    id: u16,
    identifier: String,
    species_id: u16,
    /// in 10cm
    height: u16,
    /// in 100g
    weight: u16,
    base_experience: Option<u16>,
    order: Option<u16>,
    is_default: u8,
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

/// pokemon_forms.csv
/// Contains the names for regional Pokemon and other weird forms
#[derive(Debug, Deserialize)]
pub struct PokemonForm {
    id: u16,
    identifier: String,
    form_identifier: Option<String>,
    pokemon_id: u16,
    is_default: u8,
    is_battle_only: u8,
    is_mega: u8,
    form_order: u16,
    order: u16,
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
    pub move_name: String,
    pub generation_id: u8,
}

#[derive(Debug)]
pub struct PokemonLearnableMoves {
    pub level_up: Vec<MoveEntry>,
    pub machine: Vec<MoveEntry>,
    pub tutor: Vec<MoveEntry>,
    pub egg: Vec<MoveEntry>,
}

#[derive(Debug)]
pub struct PokemonApiData {
    pub pokemon_name: String,
    pub height_in_meters: f32,
    pub weight_in_kg: f32,
    pub learnable_moves: PokemonLearnableMoves,
}

impl PokemonLearnableMoves {
    fn has_move(&self, name: String) -> bool {
        self.level_up.iter().any(|x| x.move_name == name)
            || self.machine.iter().any(|x| x.move_name == name)
            || self.tutor.iter().any(|x| x.move_name == name)
            || self.egg.iter().any(|x| x.move_name == name)
    }
}

pub fn parse_pokemon_api() -> HashMap<String, PokemonApiData> {
    let english_language_id:u8 = 9;
    let version_groups: Vec<VersionGroups> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/version_groups.csv");
    let pokemon: Vec<Pokemon> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon.csv");
    let pokemon_moves: Vec<PokemonMoves> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_moves.csv");
    let pokemon_move_methods: Vec<PokemonMoveMethods> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_move_methods.csv");
    let pokemon_species_names: Vec<PokemonSpeciesNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_species_names.csv");
    let pokemon_forms: Vec<PokemonForm> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_forms.csv");
    let pokemon_form_names: Vec<PokemonFormNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_form_names.csv");
    let move_names: Vec<MoveNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/move_names.csv");

    let mut form_id_to_pokemon_id: HashMap<u16, u16> = HashMap::default();
    for x in pokemon_forms {
        form_id_to_pokemon_id.insert(x.id, x.pokemon_id);
    }

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
            if let Some(pokemon_id) = form_id_to_pokemon_id.get(&x.pokemon_form_id) {
                pokemon_id_to_name.insert(pokemon_id.clone(), name);
            }
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

    let mut result: HashMap<String, PokemonApiData> = HashMap::default();
    for x in pokemon {
        let name = pokemon_id_to_name.get(&x.id).unwrap_or(&x.identifier);

        result.insert(name.clone(),  PokemonApiData {
            pokemon_name: name.clone(),
            height_in_meters: x.height as f32 / 10.0,
            weight_in_kg: x.weight as f32 / 10.0,
            learnable_moves: PokemonLearnableMoves {
                level_up: Vec::default(),
                machine: Vec::default(),
                tutor: Vec::default(),
                egg: Vec::default(),
            },
        });
    }

    let mut missing_pokemon_ids = Vec::new();
    let mut missing_move_ids = Vec::new();
    let mut pokemon_name_to_learnable_moves: HashMap<String, PokemonLearnableMoves> = HashMap::default();
    for pokemon_move in pokemon_moves {
        if let Some(pokemon_name) = pokemon_id_to_name.get(&pokemon_move.pokemon_id) {
            let move_name_option = move_id_to_name.get(&pokemon_move.move_id);
            if move_name_option.is_none() {
                if !missing_move_ids.contains(&pokemon_move.move_id) {
                    missing_move_ids.push(pokemon_move.move_id);
                }
                continue;
            }
            let move_name = move_name_option.unwrap().clone();

            let mut pokemon_entry = &mut result.get_mut(pokemon_name).unwrap().learnable_moves;
            if pokemon_entry.has_move(move_name.clone()) {
                continue;
            }

            let new_move_entry = MoveEntry{
                move_name,
                generation_id: version_group_id_to_generation_id.get(&pokemon_move.version_group_id).expect("All generation ids should be set").clone(),
            };

            let learn_method = method_id_to_name.get(&pokemon_move.pokemon_move_method_id).unwrap().clone();
            match learn_method.as_str() {
                "level-up" => pokemon_entry.level_up.push(new_move_entry),
                "egg" => pokemon_entry.egg.push(new_move_entry),
                "tutor" => pokemon_entry.tutor.push(new_move_entry),
                "machine" => pokemon_entry.machine.push(new_move_entry),
                _ => {}
            }
        } else {
            if !missing_pokemon_ids.contains(&pokemon_move.pokemon_id) {
                missing_pokemon_ids.push(pokemon_move.pokemon_id);
            }
        }
    }

    for x in missing_pokemon_ids {
        log::warn!("Missing pokemon data for pokemon_id {}", x)
        // 10232 - 10248 Amigento

    }
    for x in missing_move_ids {
        log::warn!("Missing move data for move_id {}", x)
    }


    result
}
