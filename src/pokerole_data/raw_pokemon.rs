use serde::Deserialize;
use crate::data::pokemon::{Height, Weight};
use crate::pokerole_discord_py_csv_parser::PokeRoleRank;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawPokerolePokemon {
    pub number: u32,
    #[serde(rename = "DexID")]
    pub dex_id: String,
    pub name: String,
    pub type1: String,
    pub type2: String,
    #[serde(rename = "BaseHP")]
    pub base_hp: u8,
    pub strength: u8,
    pub max_strength: u8,
    pub dexterity: u8,
    pub max_dexterity: u8,
    pub vitality: u8,
    pub max_vitality: u8,
    pub special: u8,
    pub max_special: u8,
    pub insight: u8,
    pub max_insight: u8,
    pub ability1: String,
    pub ability2: String,
    pub hidden_ability: String,
    pub event_abilities: String,
    pub recommended_rank: String,
    pub gender_type: String,
    pub legendary: bool,
    pub dex_category: String,
    pub height: Height,
    pub weight: Weight,
    pub dex_description: String,
    pub evolutions: Vec<Evolution>,
    pub image: String,
    pub moves: Vec<RawPokemonMoveLearnedByLevelUp>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Evolution {
    from: Option<String>,
    to: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawPokemonMoveLearnedByLevelUp {
    learned: PokeRoleRank,
    name: String,
}
