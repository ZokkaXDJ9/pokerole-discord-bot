use serenity::all::{Command, CommandInteraction, InteractionResponseType};
use serenity::prelude::*;
use chrono::{Duration, NaiveDate, Utc};

use crate::events::send_error_to_log_channel;

// Seasonal constants
const SEASONS: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"]; // Seasons in rotation

// Function to register the /season command
pub async fn register_season_command(ctx: &Context) {
    Command::create_global_application_command(&ctx.http, |command| {
        command.name("season").description("Display the current season")
    })
    .await
    .expect("Failed to create global application command");
}

// Function to handle the /season command interaction
pub async fn handle_season_command(ctx: &Context, interaction: &CommandInteraction) {
    if interaction.data.name == "season" {
        let current_season = get_current_season();

        // Respond to the interaction in the same channel
        if let Err(error) = interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content(format!(
                            "🌱 [System] The current season is: **{}**!",
                            current_season
                        ))
                    })
            })
            .await
        {
            send_error_to_log_channel(ctx, error.to_string()).await;
        }
    }
}

// Function to calculate the current season by advancing one season each week
fn get_current_season() -> &'static str {
    // Fixed starting point (epoch)
    let epoch = NaiveDate::from_ymd_opt(2021, 1, 4)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap(); // Monday, Jan 4, 2021

    let now = Utc::now().naive_utc();

    // Calculate the number of weeks since the epoch
    let duration_since_epoch = now.signed_duration_since(epoch);
    let weeks_since_epoch = duration_since_epoch.num_weeks();

    // Determine the season index
    let season_index = (weeks_since_epoch as usize) % SEASONS.len();

    SEASONS[season_index]
}
