use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_status_effect;

/// Display status effects
#[poise::command(slash_command)]
pub async fn status(
    ctx: Context<'_>,
    #[description = "Which status effect?"]
    #[rename = "name"]
    #[autocomplete = "autocomplete_status_effect"]
    status_name: String,
) -> Result<(), Error> {
    if let Some(status_effect) = ctx.data().status_effects.get(&status_name.to_lowercase()) {
        let result : String = std::format!("### {}\n*{}*\n- {}\n- {}\n- {}",
                                               &status_effect.name, &status_effect.description, &status_effect.resist, &status_effect.effect, &status_effect.duration);
        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Weather not found. Oh no!").await?;
    Ok(())
}
