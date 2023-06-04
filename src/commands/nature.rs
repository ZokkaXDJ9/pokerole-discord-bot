use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_nature;

/// Display an Ability
#[poise::command(slash_command)]
pub async fn nature(
    ctx: Context<'_>,
    #[description = "Which nature?"]
    #[rename = "nature"]
    #[autocomplete = "autocomplete_nature"]
    name: String,
) -> Result<(), Error> {
    if let Some(nature) = ctx.data().natures.get(&name.to_lowercase()) {
        let result : String = std::format!("### {}\n**Keywords**: {}\n*{}*", &nature.name, &nature.keywords, nature.description);
        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Nature not found. Oh no!").await?;
    Ok(())
}
