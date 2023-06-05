mod commands;
mod data;
mod logger;
mod csv_utils;
mod enums;
mod game_data;

use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    logger::init_logging();
    let data = data::parser::initialize_data();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_all_commands(),
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
