mod commands;
mod data;
mod pokemon_api_parser;
mod logger;
mod pokerole_discord_py_csv_parser;
mod csv_utils;
mod pokerole_data;
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
                        commands::weather::weather()];
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
