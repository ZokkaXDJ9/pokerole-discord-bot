use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

/// Receive Magicarps' blessings!
#[poise::command(slash_command, rename = "move")]
async fn skill(
    ctx: Context<'_>,
    #[description = "Which move?"] #[rename = "move"] skill: String,
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
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), skill()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
