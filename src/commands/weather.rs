use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_weather;

/// Display the Weather
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Which weather?"]
    #[rename = "name"]
    #[autocomplete = "autocomplete_weather"]
    weather_name: String,
) -> Result<(), Error> {
    if let Some(weather) = ctx.data().weather.get(&weather_name.to_lowercase()) {
        let mut result : String = std::format!("### {}\n*{}*\n{}", &weather.name, &weather.description, &weather.effect);
        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Weather not found. Oh no!").await?;
    Ok(())
}
