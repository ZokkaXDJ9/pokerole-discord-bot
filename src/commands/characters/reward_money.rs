use crate::emoji;
use crate::commands::{Context, Error, send_error};
use crate::commands::characters::{change_character_stat, validate_user_input};
use crate::commands::autocompletion::autocomplete_character_name;

/// Reward players with cash.
#[poise::command(slash_command, guild_only, default_member_permissions = "ADMINISTRATOR")]
pub async fn reward_money(
    ctx: Context<'_>,
    amount: i16,
    #[description = "Which character?"]
    #[autocomplete = "autocomplete_character_name"]
    name: String,
) -> Result<(), Error> {
    // TODO: Option to also add the untaxed amount to guild stash.
    // TODO: Button to undo the transaction which lasts for a minute or so.
    if let Err(e) = validate_user_input(name.as_str()) {
        return send_error(&ctx, e).await;
    }

    if let Ok(result) = change_character_stat(&ctx, "money", &name, amount as i64).await {
        ctx.say(format!("{} received {} {}!", name, amount, emoji::POKE_COIN)).await?;
    }

    Ok(())
}
