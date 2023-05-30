use std::collections::HashMap;
use poise::serenity_prelude as serenity;

struct Data {
    pub moves: HashMap<String, PokeMove>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    Physical,
    Special,
    Support,
}

#[derive(Debug, Clone, Copy)]
pub enum Stat {
    Strength,
    Dexterity,
    Vitality,
    Special,
    Insight
}

#[derive(Debug, Clone, Copy)]
pub enum SecondaryStat {
    Strength,
    Dexterity,
    Vitality,
    Special,
    Insight
}

pub enum HappinessDamageModifier {
    Happiness,
    MissingHappiness
}

pub enum Target {
    User,
    Ally,
    Foe,
    UserAndAllies,
    RandomFoe,
    AllFoes,
    Area,
    Battlefield,
}

// currently ordered the same way as in the .csv file
pub struct PokeMove {
    pub name: String,
    pub typing: PokeType,
    pub move_type: MoveType,
    pub base_power: u8,
    pub base_stat: Option<Stat>,
    pub accuracy_modifier_flat: Option<i8>, // todo: just default to 0
    pub accuracy_stat: SecondaryStat,
    pub secondary_stat: SecondaryStat,
    pub target: Target,
    pub description: String,
}

/// Receive Magicarps' blessings!
#[poise::command(slash_command, rename = "move")]
async fn poke_move(
    ctx: Context<'_>,
    #[description = "Which move?"] #[rename = "move"] poke_move: String,
) -> Result<(), Error> {
    ctx.say("__Splash__
*The user just flops splashing some water, this has no effect at all...*
**Type**: Normal -- **Support**
**Target**: User -- **Power**: 0
**Damage Dice**: None
**Accuracy Dice**: DEXTERITY + BRAWL
**Effect**: -").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let move_hash_map = HashMap::default();
    let mut reader = csv::Reader::from_path("/home/jacudibu/code/pokerole-csv/pokeMoveSorted.csv");
    for result in reader.expect("Move path should be valid!").records() {
        if let Ok(record) = result {
            println!("{:?}", record);
        };
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
                    moves: move_hash_map
                })
            })
        });

    framework.run().await.unwrap();
}
