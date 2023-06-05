mod commands;
mod data;
mod logger;
mod csv_utils;
mod enums;

use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    logger::init_logging();
    let commands = vec![commands::roll::roll(),
                        commands::r#move::poke_move(),
                        commands::ability::ability(),
                        commands::item::item(),
                        commands::stats::stats(),
                        commands::status::status(),
                        commands::rule::rule(),
                        commands::pokelearns::pokelearns(),
                        commands::nature::nature(),
                        commands::timestamp::timestamp(),
                        commands::weather::weather(),
                        commands::about::about()];
    let data = data::game_data::initialize_data();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        });

    framework.run().await.unwrap();
}
