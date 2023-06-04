use serde::Deserialize;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, EnumString)]
pub enum PokemonType {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    Fighting,
    Flying, Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    Any,
    Typeless,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum MoveCategory {
    Physical,
    Special,
    #[serde(rename = "Physical/special")] /// Only used for struggle
    PhysicalOrSpecial,
    Support,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
pub enum MysteryDungeonRank {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond
}
