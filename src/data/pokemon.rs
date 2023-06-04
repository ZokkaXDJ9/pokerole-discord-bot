use std::collections::HashMap;
use std::str::FromStr;
use log::{error, warn};
use serde::Deserialize;
use crate::enums::{MysteryDungeonRank, PokemonType, RegionalVariant};
use crate::pokemon_api_parser::PokemonApiData;
use crate::pokerole_data::raw_pokemon::{RawPokemonMoveLearnedByLevelUp, RawPokerolePokemon};
use crate::pokerole_discord_py_csv_parser::PokeRoleRank;

#[derive(Debug)]
pub struct Pokemon {
    pub number: u16,
    pub regional_variant: Option<RegionalVariant>,
    pub api_issue: Option<ApiIssueType>,
    pub name: String,
    pub type1: PokemonType,
    pub type2: Option<PokemonType>,
    pub base_hp: u8,
    pub strength: Stat,
    pub dexterity: Stat,
    pub vitality: Stat,
    pub special: Stat,
    pub insight: Stat,
    pub ability1: String,
    pub ability2: Option<String>,
    pub hidden_ability: Option<String>,
    pub event_abilities: Option<String>,
    pub height: Height,
    pub weight: Weight,
    pub dex_description: String,
    pub moves: Moves,
}

#[derive(Debug)]
pub enum ApiIssueType {
    FoundNothing,
    Form,
}

impl Pokemon {
    fn try_find<'a>(name: &str, api: &'a HashMap<String, PokemonApiData>)
        -> (Option<ApiIssueType>, Option<&'a PokemonApiData>) {
        if let Some(value) = api.get(name) {
            return (None, Some(value))
        }
        let options: Vec<String> = api.keys()
            .filter(|x| x.contains(&name.split(' '.to_owned()).collect::<Vec<&str>>()[0]))
            .map(|x| x.clone())
            .collect();

        if options.len() == 0 {
            error!("Found no matches for {}", name);
            return (Some(ApiIssueType::FoundNothing), None);
        }

        if options.len() == 1 {
            return (None, api.get(options.first().unwrap()));
        }

        // warn!("Found multiple matches for {}", name);
        (Some(ApiIssueType::Form), api.get(options.first().unwrap()))
    }


    fn get_api_entry<'a>(raw: &RawPokerolePokemon, api: &'a HashMap<String, PokemonApiData>, regional_variant: &Option<RegionalVariant>)
        -> (Option<ApiIssueType>, Option<&'a PokemonApiData>) {
        match regional_variant {
            None => Pokemon::try_find(&raw.name, api),
            Some(variant) => {
                // We can either replace <pokemon name>(Galarian Form) with Galarian <Pokemon name>
                // Or search for the respective form by using the <pokemon name> and form_id.
                // pokemon.csv maps pokemon-id to pokedex #, that way we could figure out how many forms a specific mon has and what they are called
                match variant {
                    RegionalVariant::Alola => Pokemon::try_find(&raw.name.split("(Alolan Form)").collect::<Vec<&str>>()[0], api),
                    RegionalVariant::Galar => Pokemon::try_find(&raw.name.split("(Galarian Form)").collect::<Vec<&str>>()[0], api),
                    RegionalVariant::Hisui => Pokemon::try_find(&raw.name.split("(Hisuian Form)").collect::<Vec<&str>>()[0], api),
                    RegionalVariant::Paldea => Pokemon::try_find(&raw.name.split("(Paldean Form)").collect::<Vec<&str>>()[0], api)
                }
            }
        }
    }

    pub(in crate::data) fn new(raw: RawPokerolePokemon, api: &HashMap<String, PokemonApiData>) -> Pokemon {
        let regional_variant= Pokemon::parse_variant(&raw.dex_id);

        let (api_issue, api_option) = Pokemon::get_api_entry(&raw, api, &regional_variant);

        let moves;
        if let Some(api_data) = api_option {
            moves = Moves {
                by_pokerole_rank: raw.moves.iter().map(|x| PokemonMoveLearnedByRank::new(x)).collect(),
                by_level_up: api_data.learnable_moves.level_up.iter().map(|x| x.move_name.to_owned()).collect(),
                by_machine: api_data.learnable_moves.machine.iter().map(|x| x.move_name.to_owned()).collect(),
                by_tutor: api_data.learnable_moves.tutor.iter().map(|x| x.move_name.to_owned()).collect(),
                by_egg: api_data.learnable_moves.egg.iter().map(|x| x.move_name.to_owned()).collect()
            };
        } else {
            moves = Moves {
                by_pokerole_rank: raw.moves.iter().map(|x| PokemonMoveLearnedByRank::new(x)).collect(),
                by_level_up: vec![],
                by_machine: vec![],
                by_tutor: vec![],
                by_egg: vec![]
            };
        }

        Pokemon {
            number: raw.number,
            name: raw.name,
            regional_variant,
            api_issue,
            type1: Pokemon::parse_type(raw.type1).unwrap(),
            type2: Pokemon::parse_type(raw.type2),
            base_hp: raw.base_hp,
            strength: Stat::new(raw.strength, raw.max_strength),
            dexterity: Stat::new(raw.dexterity, raw.max_dexterity),
            vitality: Stat::new(raw.vitality, raw.max_vitality),
            special: Stat::new(raw.special, raw.max_special),
            insight: Stat::new(raw.insight, raw.max_insight),
            ability1: raw.ability1,
            ability2: Pokemon::parse_ability(raw.ability2),
            hidden_ability: Pokemon::parse_ability(raw.hidden_ability),
            event_abilities: Pokemon::parse_ability(raw.event_abilities),
            height: raw.height,
            weight: raw.weight,
            dex_description: raw.dex_description,
            moves
        }
    }

    fn parse_variant(dex_id: &str) -> Option<RegionalVariant> {
        if dex_id.contains('A') {
            return Some(RegionalVariant::Alola);
        }
        if dex_id.contains('G') {
            return Some(RegionalVariant::Galar);
        }
        if dex_id.contains('H') {
            return Some(RegionalVariant::Hisui);
        }
        if dex_id.contains('P') {
            return Some(RegionalVariant::Paldea);
        }

        None
    }

    fn parse_type(raw: String) -> Option<PokemonType> {
        if raw.is_empty() {
            return None;
        }

        return Some(PokemonType::from_str(&raw).unwrap());
    }

    fn parse_ability(raw: String) -> Option<String> {
        if raw.is_empty() {
            return None;
        }

        return Some(raw);
    }
}

#[derive(Debug)]
pub struct Stat {
    pub min: u8,
    pub max: u8,
}

impl Stat {
    fn new(min: u8, max: u8) -> Stat {
        Stat {min, max}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Height {
    pub meters: f32,
    pub feet: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Weight {
    pub kilograms: f32,
    pub pounds: f32,
}

#[derive(Debug)]
pub struct Moves {
    pub by_pokerole_rank: Vec<PokemonMoveLearnedByRank>,
    pub by_level_up: Vec<String>,
    pub by_machine: Vec<String>,
    pub by_tutor: Vec<String>,
    pub by_egg: Vec<String>,
}

#[derive(Debug)]
pub struct PokemonMoveLearnedByRank {
    pub rank: MysteryDungeonRank,
    pub name: String
}

impl PokemonMoveLearnedByRank {
    pub(in crate::data) fn new(raw: &RawPokemonMoveLearnedByLevelUp) -> PokemonMoveLearnedByRank {
        let rank = match raw.learned {
            PokeRoleRank::Starter => MysteryDungeonRank::Bronze,
            PokeRoleRank::Beginner => MysteryDungeonRank::Bronze,
            PokeRoleRank::Amateur => MysteryDungeonRank::Silver,
            PokeRoleRank::Ace => MysteryDungeonRank::Gold,
            PokeRoleRank::Pro => MysteryDungeonRank::Platinum,
            PokeRoleRank::Master => MysteryDungeonRank::Diamond,
            PokeRoleRank::Champion => MysteryDungeonRank::Diamond,
        };

        PokemonMoveLearnedByRank {rank, name: raw.name.clone()}
    }
}
