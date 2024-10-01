use std::sync::atomic::Ordering;
use std::sync::Arc;

use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};
use serenity::all::{CreateMessage};
use serenity::model::id::ChannelId;
use serenity::prelude::Context;
use sqlx::{Pool, Sqlite};

use crate::data::Data;
use crate::events::send_error_to_log_channel;

// Added: Seasonal constants
const SEASON_CHANNEL_ID: u64 = 1290754769140580353; // Channel for season announcements
const SEASONS: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"]; // Seasons in rotation

pub async fn start_weekly_reset_thread(ctx: &Context, data: &Data) {
    let ctx = Arc::new(ctx.clone());
    if !data.is_weekly_reset_thread_running.load(Ordering::Relaxed) {
        let ctx_in_thread = Arc::clone(&ctx);
        let database = data.database.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(calculate_duration_until_next_run()).await;
                execute_weekly_reset(Arc::clone(&ctx_in_thread), database.clone()).await;
            }
        });

        data.is_weekly_reset_thread_running
            .swap(true, Ordering::Relaxed);
    }
}

fn calculate_duration_until_next_run() -> std::time::Duration {
    let now = Utc::now();
    let now_iso = now.iso_week();

    let last_monday = NaiveDate::from_isoywd_opt(now_iso.year(), now_iso.week(), Weekday::Mon)
        .expect("Week 1 Monday should always exist, even if it's within the last year.");

    let next_run = (last_monday + Duration::days(7))
        .and_hms_opt(0, 1, 0)
        .expect("Should never return None when passing 0's.");
    let seconds_until_next_run = next_run
        .signed_duration_since(now.naive_utc())
        .num_seconds()
        .unsigned_abs();

    std::time::Duration::from_secs(seconds_until_next_run)
}

async fn execute_weekly_reset(ctx: Arc<Context>, database: Pool<Sqlite>) {
    match sqlx::query!("UPDATE character SET weekly_spar_count = 0")
        .execute(&database)
        .await
    {
        Ok(_) => {
            notify_guilds(&ctx, &database).await;
            announce_season(&ctx).await; // Added: Announce the new season
            // Updating character posts is disabled until we figure out how to reopen forum threads without sending a message...
        }
        Err(error) => {
            send_error_to_log_channel(&ctx, error.to_string()).await;
        }
    }
}

async fn notify_guilds(ctx: &Arc<Context>, database: &Pool<Sqlite>) {
    match sqlx::query!("SELECT action_log_channel_id FROM guild")
        .fetch_all(database)
        .await
    {
        Ok(records) => {
            let channel_ids: Vec<i64> = records
                .iter()
                .filter_map(|x| x.action_log_channel_id)
                .collect();

            for action_log_channel_id in channel_ids {
                let channel = ChannelId::from(action_log_channel_id as u64);
                let _ = channel
                    .send_message(
                        &ctx,
                        CreateMessage::new().content("📅 [System] Performing weekly reset.\n- Weekly Spar counts have been reset."),
                    )
                    .await;
            }
        }
        Err(error) => {
            send_error_to_log_channel(ctx, error.to_string()).await;
        }
    }
}

// Added: Function to announce the season
async fn announce_season(ctx: &Arc<Context>) {
    let current_season = get_current_season();
    let season_channel = ChannelId::new(SEASON_CHANNEL_ID); // Updated line

    if let Err(error) = season_channel
        .send_message(
            &ctx,
            CreateMessage::new().content(format!(
                "🌱 [System] Welcome to the new season: **{}**!",
                current_season
            )),
        )
        .await
    {
        send_error_to_log_channel(ctx, error.to_string()).await;
    }
}

// Added: Function to calculate the current season
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
