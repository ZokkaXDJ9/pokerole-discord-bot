use serenity::all::ChannelId;
use crate::commands::{Context, Error};
use crate::game_data::PokemonApiId;
use crate::emoji;
use crate::helpers::channel_id_link;

#[poise::command(slash_command, guild_only)]
pub async fn list_characters(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().expect("Command is guild_only").clone();
    let guild_id = guild.id.get() as i64;

    // Query for all non-retired characters in the guild, sorted by name.
    let characters = sqlx::query_as!(
        CharacterInfo,
        "SELECT name, species_api_id, stat_channel_id FROM character WHERE guild_id = ? AND is_retired = false ORDER BY name",
        guild_id
    )
    .fetch_all(&ctx.data().database)
    .await?;

    let data = ctx.data();
    let mut reply = String::new();
    for character in characters {
        let channel_id = ChannelId::new(character.stat_channel_id as u64);
        let link = channel_id_link(channel_id);

        let api_id = PokemonApiId(character.species_api_id as u16);
        let pokemon = data
            .game
            .pokemon_by_api_id
            .get(&api_id)
            .expect("Database values should always be valid!");

        // Retrieve the emoji asynchronously; use a fallback if empty.
        let emoji_str = emoji::get_any_pokemon_emoji_with_space(&data.database, pokemon).await;
        let emoji_str = if emoji_str.trim().is_empty() {
            "❓ ".to_string()
        } else {
            emoji_str
        };

        reply.push_str(&format!("{} {} – {}\n", emoji_str, character.name, link));
    }

    // Split the message at line breaks, ensuring each chunk is at most 2000 characters.
    for message in split_message_at_linebreaks(&reply) {
        ctx.reply(message).await?;
    }
    Ok(())
}

struct CharacterInfo {
    name: String,
    species_api_id: i64,
    stat_channel_id: i64,
}

/// Splits a message into chunks at line break boundaries,
/// ensuring that no chunk exceeds the max Discord limit (2000 characters).
fn split_message_at_linebreaks(message: &str) -> Vec<String> {
    let max_length = 2000;
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for line in message.lines() {
        // Add back the newline that was removed by .lines()
        let line_with_newline = format!("{}\n", line);

        // If the line itself is too long, split it further.
        if line_with_newline.len() > max_length {
            // Flush the current chunk if non-empty.
            if !current_chunk.is_empty() {
                chunks.push(current_chunk);
                current_chunk = String::new();
            }
            // Split the long line safely and add its parts.
            let parts = split_long_line(&line_with_newline, max_length);
            chunks.extend(parts);
        } else {
            // If adding this line would exceed max_length, push current_chunk first.
            if current_chunk.len() + line_with_newline.len() > max_length {
                chunks.push(current_chunk);
                current_chunk = String::new();
            }
            current_chunk.push_str(&line_with_newline);
        }
    }
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }
    chunks
}

/// Fallback function to split a single long line into chunks
/// without breaking multi-byte characters.
fn split_long_line(line: &str, max_length: usize) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();

    for c in line.chars() {
        let char_len = c.len_utf8();
        if current.len() + char_len > max_length {
            parts.push(current);
            current = String::new();
        }
        current.push(c);
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}
