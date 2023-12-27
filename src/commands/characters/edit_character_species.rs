use crate::commands::autocompletion::autocomplete_character_name;
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::commands::characters::{log_action, update_character_post, ActionType};
use crate::commands::{
    find_character, pokemon_from_autocomplete_string, send_ephemeral_reply, Context, Error,
};

/// Update what kinda pokemon someone is.
#[allow(clippy::too_many_arguments)]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn edit_character_species(
    ctx: Context<'_>,
    #[description = "Which character?"]
    #[autocomplete = "autocomplete_character_name"]
    character: String,
    #[description = "Into which pokemon?"]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command is guild_only").get();
    let character = find_character(ctx.data(), guild_id, &character).await?;
    let pokemon = pokemon_from_autocomplete_string(&ctx, &pokemon)?;

    let _ = sqlx::query!(
        "UPDATE character SET species_api_id = ? WHERE id = ?",
        pokemon.poke_api_id.0,
        character.id
    )
    .execute(&ctx.data().database)
    .await;

    update_character_post(&ctx, character.id).await;

    let _ = log_action(
        &ActionType::CharacterEdit,
        &ctx,
        &format!("Set {}'s species to {}.", character.name, pokemon.name),
    )
    .await;
    let _ = send_ephemeral_reply(
        &ctx,
        &format!("Updated {}'s species to {}.", character.name, pokemon.name),
    )
    .await;
    Ok(())
}
