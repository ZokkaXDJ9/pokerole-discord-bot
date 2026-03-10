use crate::commands::{send_ephemeral_reply, Context, Error};

/// Show all quests in this server that haven't been marked as completed yet.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"

)]
pub async fn show_unfinished_quests(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command is guild_only").get() as i64;

    let quests: Vec<i64> = sqlx::query!(
        "SELECT channel_id AS quest_id
         FROM quest
         WHERE guild_id = ? AND completion_timestamp IS NULL",
        guild_id
    )
    .fetch_all(&ctx.data().database)
    .await?
    .into_iter()
    .map(|row| row.quest_id)
    .collect();

    if quests.is_empty() {
        send_ephemeral_reply(
            &ctx,
            "All quests have been completed! Nothing to do here.",
        )
        .await?;
        return Ok(());
    }

    let mut result = format!("**Unfinished quests ({}):**\n", quests.len());
    for quest_id in quests {
        result.push_str(&format!("<#{}>\n", quest_id));
    }

    ctx.say(result).await?;
    Ok(())
}
