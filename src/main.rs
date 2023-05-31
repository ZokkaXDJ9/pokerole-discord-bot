use std::collections::HashMap;
use std::sync::{Arc};
use std::fmt::Write;
use futures::Stream;
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use futures::StreamExt;

struct Data {
    pub moves: Arc<HashMap<String, PokeMove>>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Deserialize)]
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
    pub description: String,
}

/// Receive Magicarps' blessings!
#[poise::command(slash_command, rename = "move")]
async fn poke_move(
    ctx: Context<'_>,
    #[description = "Which move?"]
    #[rename = "move"]
    #[autocomplete = "autocomplete_move"]
    poke_move_name: String,
) -> Result<(), Error> {
    if let Some(poke_move) = ctx.data().moves.get(&poke_move_name) {
        ctx.say(std::format!("{:?}", poke_move)).await?;
        return Ok(());
    }

    ctx.say("Move not found. Oh no!").await?;
    Ok(())
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

async fn autocomplete_move<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let moves = &_ctx.data().moves;
    let names : Vec<String> = moves.iter().map(|x| x.0.clone()).collect();
    let stream = futures::stream::iter(names);

    // TODO: ignore case
    stream
        .filter(move |x| futures::future::ready(x.starts_with(partial)))
}

#[tokio::main]
async fn main() {
    let mut move_hash_map = HashMap::default();
    let mut moves = load_pokerole_moves("/home/jacudibu/code/pokerole-csv/pokeMoveSorted.csv");

    for x in moves {
        move_hash_map.insert(x.name.clone(), x);
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), poke_move()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    moves: Arc::new(move_hash_map)
                })
            })
        });

    framework.run().await.unwrap();
}
