use crate::commands::autocompletion::autocomplete_pokemon;
use crate::commands::create_emojis::{does_emoji_exist_in_database, store_emoji_in_database};
use crate::commands::{pokemon_from_autocomplete_string, send_ephemeral_reply, send_error};
use crate::commands::{Context, Error};
use crate::enums::Gender;
use serenity::all::Emoji;

/// Add an existing pokemon emoji to the database.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn parse_existing_emoji(
    ctx: Context<'_>,
    emoji: Emoji,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    name: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("command is guild_only").get() as i64;
    let pokemon = pokemon_from_autocomplete_string(&ctx, &name)?;
    let gender = if emoji.name.contains("female") {
        Gender::Female
    } else {
        Gender::Male
    };
    let is_shiny = emoji.name.contains("shiny");
    let is_animated = emoji.animated;

    if does_emoji_exist_in_database(
        &ctx.data().database,
        guild_id,
        pokemon,
        &gender,
        is_shiny,
        is_animated,
    )
    .await
    {
        let _ = send_error(&ctx, "Emoji was already registered.").await;
        return Ok(());
    }

    store_emoji_in_database(
        &ctx.data().database,
        guild_id,
        &emoji,
        pokemon,
        &gender,
        is_shiny,
        is_animated,
    )
    .await;

    let _ = send_ephemeral_reply(&ctx, "Done.").await;
    Ok(())
}
