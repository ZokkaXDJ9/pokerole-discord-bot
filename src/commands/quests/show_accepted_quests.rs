use crate::commands::autocompletion::autocomplete_character_name;
use crate::commands::{parse_character_names, send_error, Context, Error};

/// Lists all quests a character is participating in.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn show_accepted_quests(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_character_name"] character_name: String,
) -> Result<(), Error> {
    // Parse character name to get the corresponding character data
    let guild_id = ctx.guild_id().expect("Command is guild_only!");
    let characters = parse_character_names(&ctx, guild_id.get(), &[character_name.clone()]).await?;

    // Ensure the character exists
    if characters.is_empty() {
        return send_error(&ctx, "Character not found!").await;
    }

    let character = &characters[0]; // We assume only one character is returned with the name

    // Fetch all quests the character is signed up for
    let quests = sqlx::query!(
        "SELECT q.id, q.name FROM quest q
        INNER JOIN quest_signup qs ON q.id = qs.quest_id
        WHERE qs.character_id = ?",
        character.id
    )
    .fetch_all(&ctx.data().database)
    .await?;

    // Handle the case where the character is not part of any quest
    if quests.is_empty() {
        return send_error(
            &ctx,
            format!("Character {} is not part of any quests.", character.name),
        )
        .await;
    }

    // Build the result string
    let mut result = format!("Quests for character {}:\n", character.name);
    for quest in quests {
        result.push_str(format!("- {} (ID: {})\n", quest.name, quest.id).as_str());
    }

    // Send the list to the user
    ctx.say(result).await?;
    Ok(())
}
