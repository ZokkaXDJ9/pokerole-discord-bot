use crate::commands::{Context, Error};
use chrono::{Datelike, NaiveDate, Timelike, TimeZone, Utc};
use serenity::utils::MessageBuilder;

/// Create a timestamp based of a UTC date.
#[poise::command(slash_command)]
pub async fn timestamp(
    ctx: Context<'_>,
    #[description = "Which minute? Default now."]
    minute: Option<u8>,
    #[description = "Which hour? Default now."]
    hour: Option<u8>,
    #[description = "Which day? Default today."]
    day: Option<u8>,
    #[description = "Which month? Default this month."]
    month: Option<u8>,
    #[description = "Which year? Default this year."]
    year: Option<u16>,
) -> Result<(), Error> {
    let current_datetime = Utc::now().naive_utc();

    let minute = minute.unwrap_or(current_datetime.minute() as u8);
    let hour = hour.unwrap_or(current_datetime.hour() as u8);
    let day = day.unwrap_or(current_datetime.day() as u8);
    let month = month.unwrap_or(current_datetime.month() as u8);
    let year = year.unwrap_or(current_datetime.year() as u16);

    let timestamp = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into()).expect("ymd should be valid!")
        .and_hms_opt(hour.into(), minute.into(), 0).expect("hms should be valid!");
    let timestamp_utc = Utc.from_local_datetime(&timestamp).unwrap().naive_utc();

    let unix_timestamp = timestamp_utc.timestamp();

    let result = std::format!("<t:{0}:f> (<t:{0}:R>)", unix_timestamp.to_string());
    let mut builder = MessageBuilder::default();
    builder.push_line(&result);
    builder.push_mono_line(&result);

    ctx.say(builder.to_string()).await?;

    Ok(())
}
