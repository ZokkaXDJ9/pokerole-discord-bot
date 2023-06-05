use std::collections::{HashMap};
use log::error;
use serde::Deserialize;
use crate::csv_utils::load_csv;
use crate::data::pokemon::{Height, Weight};
use crate::enums::PokemonType;

/// version_groups.csv
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiVersionGroups {
    id: u8,
    identifier: String,
    generation_id: u8,
    order: u8
}

/// pokemon.csv
/// Contains base data about pokemon, such as height and weight. Pretty much just height and weight.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiPokemon {
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

/// ability_names.csv
/// Contains names for all abilities.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiAbilityName {
    ability_id: u16,
    local_language_id: u8,
    name: String,
}

/// pokemon_abilities.csv
/// Contains data about each pokemon's abilities.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiPokemonAbility {
    pokemon_id: u16,
    ability_id: u16,
    is_hidden: u8,
    slot: u8,
}

/// pokemon_moves.csv
/// Contains info on what moves a pokemon can learn and how.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiPokemonMoves {
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
#[allow(dead_code)]
pub struct ApiPokemonMoveMethods {
    id: u8,
    identifier: String,
}

/// pokemon_species_names.csv
/// Contains the name for regular Pokemon
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiPokemonSpeciesNames {
    pokemon_species_id: u16,
    local_language_id: u8,
    name: String,
    genus: String,
}

/// pokemon_forms.csv
/// Contains the names for regional Pokemon and other weird forms
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiPokemonForm {
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
#[allow(dead_code)]
pub struct ApiPokemonFormNames {
    pokemon_form_id: u16,
    local_language_id: u8,
    form_name: String,
    pokemon_name: Option<String>
}

/// pokemon_types.csv
/// Contains type identifiers for pokemon
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiPokemonTypes {
    pokemon_id: u16,
    type_id: u16,
    slot: u8,
}

/// pokemon_form_types.csv
/// Contains type identifiers for pokemon
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiPokemonFormTypes {
    pokemon_form_id: u16,
    type_id: u16,
    slot: u8,
}

/// move_names.csv
/// Contains the names for moves
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiMoveNames {
    move_id: u16,
    local_language_id: u8,
    name: String
}

#[derive(Debug)]
pub struct ApiMoveEntry {
    pub move_name: String,
    pub generation_id: u8,
}

#[derive(Debug)]
pub struct ApiPokemonLearnableMoves {
    pub level_up: Vec<ApiMoveEntry>,
    pub machine: Vec<ApiMoveEntry>,
    pub tutor: Vec<ApiMoveEntry>,
    pub egg: Vec<ApiMoveEntry>,
}

#[derive(Debug)]
pub struct PokemonApiData {
    pub pokemon_name: String,
    pub height: Height,
    pub weight: Weight,
    pub type1: PokemonType,
    pub type2: Option<PokemonType>,
    pub ability1: String,
    pub ability2: Option<String>,
    pub ability_hidden: Option<String>,
    pub ability_event: Option<String>,
    pub learnable_moves: ApiPokemonLearnableMoves,
}

impl ApiPokemonLearnableMoves {
    fn has_move(&self, name: String) -> bool {
        self.level_up.iter().any(|x| x.move_name == name)
            || self.machine.iter().any(|x| x.move_name == name)
            || self.tutor.iter().any(|x| x.move_name == name)
            || self.egg.iter().any(|x| x.move_name == name)
    }
}

fn type_id_to_pokemon_type(id: u16) -> PokemonType {
    match id {
        1 => PokemonType::Normal,
        2 => PokemonType::Fighting,
        3 => PokemonType::Flying,
        4 => PokemonType::Poison,
        5 => PokemonType::Ground,
        6 => PokemonType::Rock,
        7 => PokemonType::Bug,
        8 => PokemonType::Ghost,
        9 => PokemonType::Steel,
        10 => PokemonType::Fire,
        11 => PokemonType::Water,
        12 => PokemonType::Grass,
        13 => PokemonType::Electric,
        14 => PokemonType::Psychic,
        15 => PokemonType::Ice,
        16 => PokemonType::Dragon,
        17 => PokemonType::Dark,
        18 => PokemonType::Fairy,
        10001 => PokemonType::Normal, // Unknown but ... what pokemon has unknown type?! D:
        10002 => PokemonType::Shadow,
        _ => panic!("Weird type id: {}", id)
    }
}

pub fn parse_pokemon_api() -> HashMap<String, PokemonApiData> {
    let english_language_id:u8 = 9;
    let version_groups: Vec<ApiVersionGroups> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/version_groups.csv");
    let ability_names: Vec<ApiAbilityName> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/ability_names.csv");
    let pokemon: Vec<ApiPokemon> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon.csv");
    let pokemon_abilities: Vec<ApiPokemonAbility> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_abilities.csv");
    let pokemon_types: Vec<ApiPokemonTypes> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_types.csv");
    let pokemon_moves: Vec<ApiPokemonMoves> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_moves.csv");
    let pokemon_move_methods: Vec<ApiPokemonMoveMethods> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_move_methods.csv");
    let pokemon_species_names: Vec<ApiPokemonSpeciesNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_species_names.csv");
    let pokemon_forms: Vec<ApiPokemonForm> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_forms.csv");
    let pokemon_form_types: Vec<ApiPokemonFormTypes> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_form_types.csv");
    let pokemon_form_names: Vec<ApiPokemonFormNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/pokemon_form_names.csv");
    let move_names: Vec<ApiMoveNames> = load_csv("/home/jacudibu/code/pokeapi/data/v2/csv/move_names.csv");

    let mut ability_id_to_name: HashMap<u16, String> = HashMap::default();
    for x in ability_names {
        if x.local_language_id != english_language_id {
            continue;
        }

        ability_id_to_name.insert(x.ability_id, x.name);
    }

    let mut form_id_to_pokemon_id: HashMap<u16, u16> = HashMap::default();
    for x in pokemon_forms {
        form_id_to_pokemon_id.insert(x.id, x.pokemon_id);
    }

    let mut pokemon_id_to_pokemon_ability1: HashMap<u16, String> = HashMap::default();
    let mut pokemon_id_to_pokemon_ability2: HashMap<u16, String> = HashMap::default();
    let mut pokemon_id_to_pokemon_ability_hidden: HashMap<u16, String> = HashMap::default();
    for x in pokemon_abilities {
        match x.slot {
            1 => pokemon_id_to_pokemon_ability1.insert(x.pokemon_id, ability_id_to_name.get(&x.ability_id).expect("Ability should be set!").clone()),
            2 => pokemon_id_to_pokemon_ability2.insert(x.pokemon_id, ability_id_to_name.get(&x.ability_id).expect("Ability should be set!").clone()),
            3 => pokemon_id_to_pokemon_ability_hidden.insert(x.pokemon_id, ability_id_to_name.get(&x.ability_id).expect("Ability should be set!").clone()),
            _ => None,
        };
    }

    let mut pokemon_id_to_pokemon_type1: HashMap<u16, PokemonType> = HashMap::default();
    let mut pokemon_id_to_pokemon_type2: HashMap<u16, PokemonType> = HashMap::default();
    for x in pokemon_types {
        if x.slot == 1 {
            pokemon_id_to_pokemon_type1.insert(x.pokemon_id, type_id_to_pokemon_type(x.type_id));
        } else {
            pokemon_id_to_pokemon_type2.insert(x.pokemon_id, type_id_to_pokemon_type(x.type_id));
        }
    }
    for x in pokemon_form_types {
        if let Some(pokemon_id) = form_id_to_pokemon_id.get(&x.pokemon_form_id) {
            if x.slot == 1 {
                pokemon_id_to_pokemon_type1.insert(pokemon_id.to_owned(), type_id_to_pokemon_type(x.type_id));
            } else {
                pokemon_id_to_pokemon_type2.insert(pokemon_id.to_owned(), type_id_to_pokemon_type(x.type_id));
            }
        } else {
            error!("Unable to map pokemon form id {} to a pokemon id!", x.pokemon_form_id);
        }
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
            } else {
                error!("Unable to map pokemon form id {} to a pokemon id!", x.pokemon_form_id);
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
            height: Height {meters: x.height as f32 / 10.0, feet: x.height as f32 / 10.0 * 3.28084 },
            weight: Weight {kilograms: x.weight as f32 / 10.0, pounds: x.weight as f32 / 10.0 * 2.20462 },
            type1: pokemon_id_to_pokemon_type1.get(&x.species_id).expect("").to_owned(),
            type2: pokemon_id_to_pokemon_type2.get(&x.species_id).copied(),
            ability1: pokemon_id_to_pokemon_ability1.get(&x.species_id).expect("").clone(),
            ability2: pokemon_id_to_pokemon_ability2.get(&x.species_id).cloned(),
            ability_hidden: pokemon_id_to_pokemon_ability_hidden.get(&x.species_id).cloned(),
            ability_event: None,
            learnable_moves: ApiPokemonLearnableMoves {
                level_up: Vec::default(),
                machine: Vec::default(),
                tutor: Vec::default(),
                egg: Vec::default(),
            },
        });
    }

    let mut missing_pokemon_ids = Vec::new();
    let mut missing_move_ids = Vec::new();
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

            let pokemon_entry = &mut result.get_mut(pokemon_name).unwrap().learnable_moves;
            if pokemon_entry.has_move(move_name.clone()) {
                continue;
            }

            let new_move_entry = ApiMoveEntry {
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
        // 10250 - 10271 is Amigento, skip that one for now
        if x < 10250 || x > 10271 {
            log::warn!("Missing pokemon data for pokemon_id {}", x)
        }

    }
    for x in missing_move_ids {
        log::warn!("Missing move data for move_id {}", x)
    }


    result
}
