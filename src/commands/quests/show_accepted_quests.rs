use crate::commands::autocompletion::autocomplete_character_name;
use crate::commands::{find_character, send_error, send_ephemeral_reply, Context, Error};
use crate::helpers;

/// Autocomplete function for the `filter` parameter
async fn autocomplete_filter(
    _ctx: Context<'_>,
    _partial: &str,
) -> impl Iterator<Item = String> {
    vec![
        "Accepted".to_string(),
        "Accepted and not completed".to_string(),
    ]
    .into_iter()
}

/// Show all quests that a character has signed up for.
#[poise::command(
    slash_command,
    guild_only,
    rename = "show_character_quests"
)]
pub async fn show_accepted_quests(
    ctx: Context<'_>,
    #[description = "Which character?"]
    #[autocomplete = "autocomplete_character_name"] character_name: String,
    #[description = "Optionally filter by quest status."]
    #[autocomplete = "autocomplete_filter"]
    filter: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command is guild_only").get() as i64;

    // Find the character by name (don't strip the autocomplete suffix - it's needed for disambiguation)
    let character = find_character(ctx.data(), guild_id.try_into().unwrap(), &character_name).await?;

    // Collect quest IDs to return
    let quests: Vec<i64> = match filter.as_deref() {
        Some("Accepted") => {
            // Show quests where the character was accepted
            sqlx::query!(
                "SELECT q.channel_id AS quest_id
                 FROM quest q
                 INNER JOIN quest_signup qs ON q.channel_id = qs.quest_id
                 WHERE qs.character_id = ? AND qs.accepted = 1",
                character.id
            )
            .fetch_all(&ctx.data().database)
            .await?
            .into_iter()
            .map(|row| row.quest_id)
            .collect()
        }
        Some("Accepted and not completed") => {
            // Show quests where the character was accepted but hasn't completed the quest
            sqlx::query!(
                "SELECT q.channel_id AS quest_id
                 FROM quest q
                 INNER JOIN quest_signup qs ON q.channel_id = qs.quest_id
                 LEFT JOIN quest_completion qc ON q.channel_id = qc.quest_id AND qc.character_id = qs.character_id
                 WHERE qs.character_id = ? AND qs.accepted = 1 AND qc.quest_id IS NULL",
                character.id
            )
            .fetch_all(&ctx.data().database)
            .await?
            .into_iter()
            .map(|row| row.quest_id)
            .collect()
        }
        _ => {
            // Default: Show all quests the character signed up for (whether accepted or not)
            sqlx::query!(
                "SELECT q.channel_id AS quest_id
                 FROM quest q
                 INNER JOIN quest_signup qs ON q.channel_id = qs.quest_id
                 WHERE qs.character_id = ?",
                character.id
            )
            .fetch_all(&ctx.data().database)
            .await?
            .into_iter()
            .map(|row| row.quest_id)
            .collect()
        }
    };

    // Check if no quests were found
    if quests.is_empty() {
        send_ephemeral_reply(
            &ctx,
            &format!("Character '{}' is not part of any quests.", character.name),
        ).await?;
        return Ok(());
    }

    // Build the result message with quest links
    let mut result = format!("Accepted quests for character '{}':\n", character.name);
    for quest_id in quests {
        result.push_str(&format!("<#{}>\n", quest_id));
    }

    // Send the result message
    ctx.say(result).await?;
    Ok(())
}
