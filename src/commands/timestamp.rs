use crate::commands::{Context, Error};
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, TimeZone, Utc};

/// Create a timestamp.
#[poise::command(slash_command)]
pub async fn timestamp(
    ctx: Context<'_>,
    minute: Option<u8>,
    hour: Option<u8>,
    day: Option<u8>,
    month: Option<u8>,
    year: Option<u16>,
) -> Result<(), Error> {

    // Get the current date and time in UTC
    let current_datetime = Utc::now().naive_utc();

    // Get the components from the input options or use the current components as defaults
    let minute = minute.unwrap_or(current_datetime.minute() as u8);
    let hour = hour.unwrap_or(current_datetime.hour() as u8);
    let day = day.unwrap_or(current_datetime.day() as u8);
    let month = month.unwrap_or(current_datetime.month() as u8);
    let year = year.unwrap_or(current_datetime.year() as u16);

    // Create a NaiveDateTime object from the components in UTC
    let timestamp = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into()).expect("Should be valid!")
        .and_hms_opt(hour.into(), minute.into(), 0).unwrap();
    let timestamp_utc = Utc.from_local_datetime(&timestamp).unwrap().naive_utc();

    // Convert the NaiveDateTime to Unix timestamp (seconds since January 1, 1970)
    let unix_timestamp = timestamp_utc.timestamp();

    // Send the Unix timestamp as a response in the Discord channel
    ctx.say(std::format!("<t:{0}:f> | <t:{0}:R>", unix_timestamp.to_string())).await?;

    Ok(())
}
