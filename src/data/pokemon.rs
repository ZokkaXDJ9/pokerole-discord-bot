use std::str::FromStr;
use serde::Deserialize;
use crate::enums::{MysteryDungeonRank, PokemonType};
use crate::pokerole_data::raw_pokemon::{RawPokemonMoveLearnedByLevelUp, RawPokerolePokemon};
use crate::pokerole_discord_py_csv_parser::PokeRoleRank;

#[derive(Debug)]
pub struct Pokemon {
    pub number: u16,
    // pub dex_id: String, // TODO: Do we need this? Might be better to parse it into a variant enum
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

impl Pokemon {
    pub(in crate::data) fn new(raw: RawPokerolePokemon) -> Pokemon {
        let moves = Moves {by_pokerole_rank: raw.moves.iter().map(|x| PokemonMoveLearnedByRank::new(x)).collect() };

        Pokemon {
            number: raw.number,
            name: raw.name,
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
    pub by_pokerole_rank: Vec<PokemonMoveLearnedByRank>
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
