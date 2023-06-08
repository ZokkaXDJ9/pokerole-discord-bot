use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_ability;

/// Display an Ability
#[poise::command(slash_command)]
pub async fn ability(
    ctx: Context<'_>,
    #[description = "Which ability?"]
    #[rename = "ability"]
    #[autocomplete = "autocomplete_ability"]
    name: String,
) -> Result<(), Error> {
    if let Some(ability) = ctx.data().abilities.get(&name.to_lowercase()) {
        ctx.say(ability.build_string()).await?;
    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find an ability named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}
