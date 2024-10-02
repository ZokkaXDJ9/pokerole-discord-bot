use crate::commands::Context;
use crate::Error;
use chrono::{NaiveDate, Utc};

/// Display the current season.
#[poise::command(slash_command)]
pub async fn season(ctx: Context<'_>) -> Result<(), Error> {
    let current_season = get_current_season();

    ctx.say(format!(
        "🌱 [System] The current season is: **{}**!",
        current_season
    ))
    .await?;

    Ok(())
}

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

    // Seasonal rotation
    const SEASONS: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];
    let season_index = (weeks_since_epoch as usize) % SEASONS.len();

    SEASONS[season_index]
}
