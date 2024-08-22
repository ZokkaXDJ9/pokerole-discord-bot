mod cache;
mod character_stats;
mod commands;
mod csv_utils;
mod data;
mod database_helpers;
mod discord_error_codes;
mod emoji;
mod enums;
mod errors;
mod events;
mod game_data;
mod helpers;
mod logger;

use crate::data::Data;
use crate::errors::CommandInvocationError;
use poise::builtins::on_error;
use poise::{serenity_prelude as serenity, CreateReply, FrameworkError};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use std::sync::Arc;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    logger::init_logging();

    let data = Data::new(
        initialize_database().await,
        Arc::new(game_data::parser::initialize_data().await),
    )
    .await;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_all_commands(),  // Load all commands
            event_handler: |serenity_ctx, event, ctx, _| {
                Box::pin(events::handle_events(serenity_ctx, event, ctx))
            },
            on_error: |error| Box::pin(handle_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Registering commands globally...");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;  // Register commands globally
                println!("Commands registered successfully.");
                Ok(data)
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged().union(serenity::GatewayIntents::GUILD_MEMBERS);

    let mut client = serenity::ClientBuilder::new(token, intents)  // Declare `client` as mutable
        .framework(framework)
        .await
        .expect("Creating client failed!");

    client.start().await.unwrap();
}

async fn handle_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::Command { ctx, error, .. } => {
            let should_error_get_logged = match error.downcast_ref::<CommandInvocationError>() {
                None => true,
                Some(e) => e.log,
            };

            if should_error_get_logged {
                log::error!(
                    "An error occurred in command /{}: {}",
                    &ctx.command().name,
                    error
                );
            }
            if let Err(e) = ctx
                .send(
                    CreateReply::default()
                        .ephemeral(true)
                        .reply(true)
                        .content(error.to_string()),
                )
                .await
            {
                log::error!("Fatal error while sending error message: {}", e);
            }
        }
        _ => {
            if let Err(e) = on_error(error).await {
                log::error!("Fatal error while sending error message: {}", e);
            }
        }
    }
}

async fn initialize_database() -> Pool<Sqlite> {
    let database = get_db_pool().await;

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    database
}

async fn get_db_pool() -> Pool<Sqlite> {
    let url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(url.as_str())
                .expect("Unable to parse DATABASE_URL"),
        )
        .await
        .expect("Couldn't connect to database")
}
