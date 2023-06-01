mod commands;
mod data;

use std::collections::HashMap;
use std::sync::{Arc};
use csv::ByteRecord;
use log::debug;
use poise::serenity_prelude as serenity;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use crate::data::Data;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PokemonType {
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
}

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
#[serde(rename_all = "UPPERCASE")]
pub enum SecondaryStat {
    Strength,
    Dexterity,
    Vitality,
    Special,
    Insight,
    Tough,
    Cool,
    Beauty,
    Clever,
    Cute,
    Brawl,
    Channel,
    Clash,
    Evasion,
    Alert,
    Athletic,
    Nature,
    Stealth,
    Allure,
    Etiquette,
    Intimidate,
    Perform,
    Will,
    #[serde(rename = "Same as the copied move")]
    Copied,
    #[serde(rename = "TOUGH/CUTE")]
    ToughOrCute,
    #[serde(rename = "MISSING BEAUTY")]
    MissingBeauty,
    #[serde(rename = "BRAWL/CHANNEL")]
    BrawlOrChannel,
    Varies,
    Empathy,
    Medicine,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum HappinessDamageModifier {
    #[serde(rename = "HAPPINESS")]
    Happiness,
    #[serde(rename = "MISSING HAPPINESS")]
    MissingHappiness
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

// currently ordered the same way as in the .csv file
#[derive(Debug, Deserialize)]
pub struct PokeMove {
    pub name: String,
    pub typing: MovePokemonType,
    pub move_type: MoveType,
    pub base_power: u8,
    pub base_stat: Option<Stat>,
    pub happiness: Option<HappinessDamageModifier>,
    pub accuracy_stat: Option<SecondaryStat>,
    pub secondary_stat: Option<SecondaryStat>,
    pub target: Target,
    pub effect: String,
    pub description: Option<String>,
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

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum GenderType {
    M,
    F,
    N
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum PokeRoleRank {
    Starter,
    Beginner,
    Amateur,
    Ace,
    Pro,
    Master,
    Champion
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
    number_and_name: String,
    moves: Vec<String>,
}

fn load_pokerole_learns(path: &str) -> Vec<RawPokeLearns> {
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);


    let mut collection = Vec::new();
    for result in reader.expect(path).byte_records() {
        let record: ByteRecord = result.expect("");
        let learns: RawPokeLearns = record.deserialize(None).expect("");
        collection.push(learns);
        //println!("{:?}", learns);
    }

    return collection
}

pub struct PokeLearn {
    pokemon_name: String,
    moves: Vec<PokeLearnEntry>,
}

pub struct PokeLearnEntry {
    rank: PokeRoleRank,
    poke_move: String,
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

fn load_pokerule_csv<T: DeserializeOwned>(path: &str) -> Vec<T> {
    let mut results = Vec::new();

    let reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path);

    for result in reader.expect(path).records() {
        if let Ok(record) = result {
            let value: T = record.deserialize(None).expect("Unable to parse csv row");
            results.push(value);
        };
    }

    return results;
}

fn load_pokerules_csv_with_custom_headers<T: DeserializeOwned>(path: &str, headers: Vec<&str>) -> Vec<T> {
    let mut result = Vec::new();
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);

    let header_records = csv::StringRecord::from(headers);
    for row in reader.expect(path).records() {
        if let Ok(record) = row {
            let item : T = record.deserialize(Some(&header_records)).expect("Csv should be parsable!");
            result.push(item);
        };
    }

    return result;
}

#[tokio::main]
async fn main() {
    let raw_weather: Vec<PokeWeather> = load_pokerules_csv_with_custom_headers("/home/jacudibu/code/pokerole-csv/weather.csv", vec![
        "name",
        "description",
        "effect"
    ]);
    let moves: Vec<PokeMove> = load_pokerules_csv_with_custom_headers("/home/jacudibu/code/pokerole-csv/pokeMoveSorted.csv", vec![
        "name",
        "typing",
        "move_type",
        "base_power",
        "base_stat",
        "happiness",
        "accuracy_stat",
        "secondary_stat",
        "target",
        "effect",
        "description",
    ]);
    let abilities : Vec<PokeAbility> = load_pokerule_csv("/home/jacudibu/code/pokerole-csv/PokeRoleAbilities.csv");
    let poke : Vec<PokeStats> = load_pokerule_csv("/home/jacudibu/code/pokerole-csv/PokeroleStats.csv");
    let raw_learns = load_pokerole_learns("/home/jacudibu/code/pokerole-csv/PokeLearnMovesFull.csv");
    let pokemon_learns = parse_pokerole_learns(raw_learns);

    let mut move_names = Vec::default();
    let mut move_hash_map = HashMap::default();
    for x in moves {
        move_names.push(x.name.clone());
        move_hash_map.insert(x.name.clone(), x);
    }

    let mut ability_names = Vec::default();
    let mut ability_hash_map = HashMap::default();
    for x in abilities {
        ability_names.push(x.name.clone());
        ability_hash_map.insert(x.name.clone(), x);
    }

    let mut weather_names = Vec::default();
    let mut weather_hash_map = HashMap::default();
    for x in raw_weather {
        weather_names.push(x.name.clone());
        weather_hash_map.insert(x.name.clone(), x);
    }

    let mut pokemon_names = Vec::default();
    let mut pokemon = HashMap::default();
    for x in poke {
        pokemon_names.push(x.name.clone());
        pokemon.insert(x.name.clone(), x);
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::roll(), commands::poke_move(), commands::ability(), commands::stats(), commands::pokelearns(), commands::weather()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    abilities: Arc::new(ability_hash_map),
                    ability_names: Arc::new(ability_names),
                    moves: Arc::new(move_hash_map),
                    move_names: Arc::new(move_names),
                    pokemon: Arc::new(pokemon),
                    pokemon_names: Arc::new(pokemon_names),
                    pokemon_learns: Arc::new(pokemon_learns),
                    weather: Arc::new(weather_hash_map),
                    weather_names: Arc::new(weather_names),
                })
            })
        });

    framework.run().await.unwrap();
}
