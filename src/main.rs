mod commands;
mod data;

use std::collections::HashMap;
use std::sync::{Arc};
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use crate::data::Data;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PokeType {
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
    pub typing: PokeType,
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

fn load_pokerole_abilities(path: &str) -> Vec<PokeAbility> {
    let mut abilities = Vec::new();

    let reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path);

    for result in reader.expect("Ability path should be valid!").records() {
        if let Ok(record) = result {
            let value: PokeAbility = record.deserialize(None).expect("Csv should be parsable!");
            abilities.push(value);
        };
    }

    return abilities;
}

fn load_pokerole_moves(path: &str) -> Vec<PokeMove> {
    let mut moves = Vec::new();
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);

    let headers = csv::StringRecord::from(vec![
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

    for result in reader.expect("Move path should be valid!").records() {
        if let Ok(record) = result {
            let poke_move : PokeMove = record.deserialize(Some(&headers)).expect("Csv should be parsable!");
            moves.push(poke_move);
        };
    }

    return moves;
}

#[tokio::main]
async fn main() {
    let moves = load_pokerole_moves("/home/jacudibu/code/pokerole-csv/pokeMoveSorted.csv");
    let abilities = load_pokerole_abilities("/home/jacudibu/code/pokerole-csv/PokeRoleAbilities.csv");

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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::poke_move(), commands::ability()],
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
                })
            })
        });

    framework.run().await.unwrap();
}
