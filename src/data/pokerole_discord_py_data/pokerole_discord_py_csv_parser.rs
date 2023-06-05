use std::path::Path;
use csv::ByteRecord;
use serde::Deserialize;
use crate::csv_utils;
use crate::data::enums::poke_role_rank::PokeRoleRank;
use crate::enums::{CombatOrSocialStat, HappinessDamageModifier, PokemonType};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MovePokemonType {
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
#[serde(rename_all = "UPPERCASE")]
pub enum MoveType {
    Physical,
    Special,
    #[serde(rename = "PHYSICAL/SPECIAL")]
    PhysicalOrSpecial,
    Support,
    #[serde(rename = "???")]
    Unknown,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Stat {
    Strength,
    Dexterity,
    Vitality,
    Special,
    Insight,
    #[serde(rename = "Same as the copied move")]
    Copied,
    #[serde(rename = "STRENGTH/SPECIAL")]
    StrengthOrSpecial,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Target {
    User,
    #[serde(rename = "One Ally")]
    OneAlly,
    Ally,
    Foe,
    #[serde(rename = "User and Allies")]
    UserAndAllies,
    #[serde(rename = "Random Foe")]
    RandomFoe,
    #[serde(rename = "All Foes")]
    AllFoes,
    Area,
    Battlefield,
    #[serde(rename = "Battlefield and Area")]
    BattlefieldAndArea,
    Any,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokeAbility {
    pub name: String,
    pub effect: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct PokeWeather {
    pub name: String,
    pub description: String,
    pub effect: String,
}

#[derive(Debug, Deserialize)]
pub struct PokeStatus {
    pub name: String,
    pub description: String,
    pub resist: String,
    pub effect: String,
    pub duration: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum GenderType {
    M,
    F,
    N
}

impl PokeRoleRank {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Starter" => Some(PokeRoleRank::Starter),
            "Beginner" => Some(PokeRoleRank::Beginner),
            "Amateur" => Some(PokeRoleRank::Amateur),
            "Ace" => Some(PokeRoleRank::Ace),
            "Pro" => Some(PokeRoleRank::Pro),
            "Master" => Some(PokeRoleRank::Master),
            "Champion" => Some(PokeRoleRank::Champion),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PokeStats {
    #[serde(rename = "No.")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type 1")]
    pub type1: Option<PokemonType> ,
    #[serde(rename = "Type 2")]
    pub type2: Option<PokemonType>,
    #[serde(rename = "HP")]
    pub base_hp: u8,
    #[serde(rename = "Strength")]
    pub strength: u8,
    #[serde(rename = "Max Strength")]
    pub max_strength: u8,
    #[serde(rename = "Dexterity")]
    pub dexterity: u8,
    #[serde(rename = "Max Dexterity")]
    pub max_dexterity: u8,
    #[serde(rename = "Vitality")]
    pub vitality: u8,
    #[serde(rename = "Max Vitality")]
    pub max_vitality: u8,
    #[serde(rename = "Special")]
    pub special: u8,
    #[serde(rename = "Max Special")]
    pub max_special: u8,
    #[serde(rename = "Insight")]
    pub insight: u8,
    #[serde(rename = "Max Insight")]
    pub max_insight: u8,
    #[serde(rename = "Ability 1")]
    pub ability1: Option<String>,
    #[serde(rename = "Ability 2")]
    pub ability2: Option<String>,
    #[serde(rename = "Hidden Ability")]
    pub ability_hidden: Option<String>,
    #[serde(rename = "Event Ability")]
    pub ability_event: Option<String>,
    #[serde(rename = "Unevolved?")]
    pub is_unevolved: Option<String>,
    #[serde(rename = "Has a form?")]
    pub has_form: Option<String>,
    #[serde(rename = "Recommended Rank")]
    pub rank: PokeRoleRank,
    #[serde(rename = "Gender Type")]
    pub gender_type: Option<GenderType>,
}

#[derive(Debug, Deserialize)]
struct RawPokeLearns {
    pub number_and_name: String,
    pub moves: Vec<String>,
}

fn load_pokerole_learns<P: AsRef<Path>>(path: P) -> Vec<RawPokeLearns> {
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);


    let mut collection = Vec::new();
    for result in reader.expect("").byte_records() {
        let record: ByteRecord = result.expect("");
        let learns: RawPokeLearns = record.deserialize(None).expect("");
        collection.push(learns);
        //println!("{:?}", learns);
    }

    return collection
}

pub struct PokeLearn {
    pub pokemon_name: String,
    pub moves: Vec<PokeLearnEntry>,
}

pub struct PokeLearnEntry {
    pub rank: PokeRoleRank,
    pub poke_move: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokeItem {
    pub name: String,
    pub description: String,
    #[serde(rename = "Type Bonus")]
    pub type_bonus: Option<String>,
    pub value: Option<String>,
    pub strength: Option<String>,
    pub dexterity: Option<String>,
    pub vitality: Option<String>,
    pub special: Option<String>,
    pub insight: Option<String>,
    pub defense: Option<String>,
    #[serde(rename = "Special Defense")]
    pub special_defense: Option<String>,
    pub evasion: Option<String>,
    pub accuracy: Option<String>,
    #[serde(rename = "Specific Pokemon")]
    pub specific_pokemon: Option<String>,
    #[serde(rename = "Heal Amount")]
    pub heal_amount: Option<String>,
    #[serde(rename = "Suggested Price")]
    pub suggested_price: Option<String>,
    #[serde(rename = "PMD Price")]
    pub pmd_price: Option<String>
}

pub struct RawPokeroleDiscordPyCsvData {
    pub weather: Vec<PokeWeather>,
    pub status_effects: Vec<PokeStatus>,
//    pub moves: Vec<PokeMove>,
//    pub items: Vec<PokeItem>,
//    pub abilities: Vec<PokeAbility>,
//    pub stats: Vec<PokeStats>,
//    pub learns: Vec<PokeLearn>,
}

fn parse_pokerole_learns(raw: Vec<RawPokeLearns>) -> Vec<PokeLearn> {
    let mut result = Vec::new();
    for raw_learns in raw {
        let mut learns : Vec<PokeLearnEntry> = Vec::new();

        for chunk in raw_learns.moves.chunks(2) {
            if chunk[0].is_empty() || chunk[1].is_empty() {
                continue;
            }

            learns.push(PokeLearnEntry {
                poke_move: chunk[0].clone(),
                rank: PokeRoleRank::from_str(chunk[1].as_str()).unwrap(),
            })
        }

        result.push(PokeLearn {
            pokemon_name: raw_learns.number_and_name,
            moves: learns
        })
    }

    result
}

pub fn parse(path_to_repo: &str) -> RawPokeroleDiscordPyCsvData {
//    let raw_learns = load_pokerole_learns(path_to_repo.to_owned() + "PokeLearnMovesFull.csv");

    RawPokeroleDiscordPyCsvData {
        weather: csv_utils::load_csv_with_custom_headers(path_to_repo.to_owned() + "weather.csv", vec![
            "name",
            "description",
            "effect"
        ]),
        status_effects: csv_utils::load_csv_with_custom_headers(path_to_repo.to_owned() + "status.csv", vec![
            "name",
            "description",
            "resist",
            "effect",
            "duration",
        ]),
//        moves: csv_utils::load_csv_with_custom_headers(path_to_repo.to_owned() + "pokeMoveSorted.csv", vec![
//            "name",
//            "typing",
//            "move_type",
//            "base_power",
//            "base_stat",
//            "happiness",
//            "accuracy_stat",
//            "secondary_stat",
//            "target",
//            "effect",
//            "description",
//        ]),
//        items: csv_utils::load_csv(path_to_repo.to_owned() + "PokeRoleItems.csv"),
//        abilities: csv_utils::load_csv(path_to_repo.to_owned() + "PokeRoleAbilities.csv"),
//        stats: csv_utils::load_csv(path_to_repo.to_owned() + "PokeroleStats.csv"),
//        learns: parse_pokerole_learns(raw_learns),
    }
}
