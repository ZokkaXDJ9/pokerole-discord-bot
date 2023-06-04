use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
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
