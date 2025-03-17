use serenity::all::ChannelId;
use crate::commands::{Context, Error};
use crate::helpers;
use crate::game_data::PokemonApiId;
use crate::emoji;

#[poise::command(slash_command, guild_only)]
pub async fn list_characters(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().expect("Command is guild_only").clone();
    let guild_id = guild.id.get() as i64;

    // Query for all non-retired characters in the guild, including species_api_id for emoji lookup.
    let characters = sqlx::query_as!(
        CharacterInfo,
        "SELECT name, species_api_id, stat_channel_id FROM character WHERE guild_id = ? AND is_retired = false",
        guild_id
    )
    .fetch_all(&ctx.data().database)
    .await?;

    let data = ctx.data();
    let mut reply = String::new();
    for character in characters {
        let channel_id = ChannelId::new(character.stat_channel_id as u64);
        let link = helpers::channel_id_link(channel_id);

        let api_id = PokemonApiId(character.species_api_id as u16);
        let pokemon = data
            .game
            .pokemon_by_api_id
            .get(&api_id)
            .expect("Database values should always be valid!");

        // Await the emoji string and provide a fallback if it's empty.
        let emoji_str = emoji::get_any_pokemon_emoji_with_space(&data.database, pokemon).await;
        let emoji_str = if emoji_str.trim().is_empty() {
            "❓ ".to_string() // Fallback emoji if nothing is returned
        } else {
            emoji_str
        };

        reply.push_str(&format!("{} {} – {}\n", emoji_str, character.name, link));
    }

    ctx.reply(reply).await?;
    Ok(())
}

struct CharacterInfo {
    name: String,
    species_api_id: i64,
    stat_channel_id: i64,
}
