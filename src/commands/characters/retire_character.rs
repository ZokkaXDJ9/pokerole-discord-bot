use crate::commands::autocompletion::autocomplete_character_name;
use crate::commands::characters::{log_action, ActionType};
use crate::commands::{find_character, Context, Error};
use crate::helpers;

/// Removes a character from this guilds roster.
#[allow(clippy::too_many_arguments)]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn retire_character(
    ctx: Context<'_>,
    #[description = "Which character?"]
    #[autocomplete = "autocomplete_character_name"]
    character: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command is guild_only").get();
    let character = find_character(ctx.data(), guild_id, &character).await?;

    match sqlx::query!(
        "UPDATE character SET is_retired = true WHERE id = ?",
        character.id
    )
    .execute(&ctx.data().database)
    .await
    {
        Ok(_) => {
            let _ = ctx
                .reply(&format!("{} has been retired.", character.name))
                .await;

            let _ = log_action(
                &ActionType::CharacterStatReset,
                &ctx,
                &format!("{} has been retired.", character.name),
            )
            .await;

            ctx.data().cache.reset(&ctx.data().database).await;
        }
        Err(e) => {
            let _ = ctx
                .reply(&format!(
                    "Something went wrong when trying to retire {}:\n```{:?}```\n{}, please look into this.",
                    character.name,
                    e,
                    helpers::ADMIN_PING_STRING
                ))
                .await;
        }
    }

    Ok(())
}
