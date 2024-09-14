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

/// Show all quests that the character has signed up for, with optional filters.
#[poise::command(
    slash_command,
    guild_only
)]
pub async fn show_accepted_quests(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_character_name"] character_name: String,
    #[autocomplete = "autocomplete_filter"]
    #[description = "Filter by accepted quests or accepted and not completed quests"] filter: Option<String>, // 'accepted' or 'accepted_unfinished'
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command is guild_only").get() as i64;

    // Clean the character name
    let cleaned_character_name = character_name.trim().split(' ').next().unwrap_or(&character_name);

    // Find the character by name
    let character = find_character(ctx.data(), guild_id.try_into().unwrap(), cleaned_character_name).await?;

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
            &format!("Character '{}' is not part of any quests.", cleaned_character_name),
        ).await?;
        return Ok(());
    }

    // Build the result message with quest links
    let mut result = format!("Accepted quests for character '{}':\n", cleaned_character_name);
    for quest_id in quests {
        result.push_str(&format!("<#{}>\n", quest_id));
    }

    // Send the result message
    ctx.say(result).await?;
    Ok(())
}
