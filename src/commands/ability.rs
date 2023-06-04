use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_ability;

/// Display an Ability
#[poise::command(slash_command)]
pub async fn ability(
    ctx: Context<'_>,
    #[description = "Which ability?"]
    #[rename = "ability"]
    #[autocomplete = "autocomplete_ability"]
    ability_name: String,
) -> Result<(), Error> {
    if let Some(ability) = ctx.data().abilities.get(&ability_name.to_lowercase()) {
        let result : String = std::format!("### {}\n{}\n*{}*", &ability.name, &ability.effect, ability.description);
        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Ability not found. Oh no!").await?;
    Ok(())
}
