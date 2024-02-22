use crate::commands::autocompletion::autocomplete_character_name;
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::commands::characters::{log_action, reset_character_stats, ActionType};
use crate::commands::create_emojis::create_emojis_for_pokemon;
use crate::commands::{
    find_character, pokemon_from_autocomplete_string, send_ephemeral_reply, send_error,
    update_character_post, Context, Error,
};
use crate::enums::Gender;
use crate::game_data::PokemonApiId;

/// Update character data. All arguments are optional.
#[allow(clippy::too_many_arguments)]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn edit_character(
    ctx: Context<'_>,
    #[description = "Which character?"]
    #[autocomplete = "autocomplete_character_name"]
    character: String,
    #[description = "Change their name?"] name: Option<String>,
    #[description = "Change their species?"]
    #[autocomplete = "autocomplete_pokemon"]
    species: Option<String>,
    #[description = "Change from which pokemon their stats should be taken? Useful for unevolved mons at high levels."]
    #[autocomplete = "autocomplete_pokemon"]
    species_override_for_stats: Option<String>,
) -> Result<(), Error> {
    if species.is_some() && species_override_for_stats.is_some() {
        send_error(&ctx, "\
You can't (and probably also don't really want to) edit a character's species and its override at the same time:
- Changing the species will remove any existing overrides.
- Overrides are meant for situations where you aren't able to change the species due to to other external constraints.").await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().expect("Command is guild_only").get();
    let character = find_character(ctx.data(), guild_id, &character).await?;

    let record = sqlx::query!(
        "SELECT name, species_api_id, phenotype, is_shiny, species_override_for_stats FROM character WHERE id = ?",
        character.id
    )
    .fetch_one(&ctx.data().database)
    .await?;

    let mut action_log = Vec::new();

    let mut should_stats_be_reset = false;
    let mut reset_species_override = false;
    let species = if let Some(species) = species {
        let species = pokemon_from_autocomplete_string(&ctx, &species)?;
        if species.poke_api_id.0 as i64 != record.species_api_id {
            action_log.push(format!("species to {}", species.name));
            should_stats_be_reset = record.species_override_for_stats.is_none();
        }

        let gender = Gender::from_phenotype(record.phenotype);
        create_emojis_for_pokemon(&ctx, species, &gender, record.is_shiny).await;
        if let Some(existing_species_override) = record.species_override_for_stats {
            if existing_species_override == species.poke_api_id.0 as i64 {
                reset_species_override = true;
                should_stats_be_reset = false;
            }
        }

        species
    } else {
        ctx.data()
            .game
            .pokemon_by_api_id
            .get(&PokemonApiId(record.species_api_id as u16))
            .expect("Species IDs in database should always be valid!")
    };

    let species_override_for_stats =
        if let Some(species_override_for_stats) = species_override_for_stats {
            let species = pokemon_from_autocomplete_string(&ctx, &species_override_for_stats)?;
            if species.poke_api_id.0 as i64 != record.species_api_id {
                action_log.push(format!("species stat override to {}", species.name));
                should_stats_be_reset = true;
                Some(species.poke_api_id.0 as i64)
            } else {
                action_log.push(String::from("removed species stat override"));
                None
            }
        } else if reset_species_override {
            action_log.push(String::from("removed species stat override"));
            None
        } else {
            record.species_override_for_stats
        };

    let name = if let Some(name) = name {
        action_log.push(format!("name to {}", name));
        name
    } else {
        record.name
    };

    if action_log.is_empty() {
        send_error(&ctx, "No changes requested, aborting.").await?;
        return Ok(());
    }

    sqlx::query!(
        "UPDATE character SET name = ?, species_api_id = ?, species_override_for_stats = ? WHERE id = ?",
        name,
        species.poke_api_id.0,
        species_override_for_stats,
        character.id,
    )
    .execute(&ctx.data().database)
    .await?;

    if should_stats_be_reset {
        let _ = reset_character_stats::reset_db_stats(&ctx, &character).await;
        action_log.push("and reset their stats".to_string());
    }

    update_character_post(&ctx, character.id).await;

    let action_log = action_log.join(", ");
    let _ = log_action(
        &ActionType::CharacterEdit,
        &ctx,
        &format!("Set {}'s {}.", character.name, action_log),
    )
    .await;
    let _ = send_ephemeral_reply(
        &ctx,
        &format!("Updated {}'s {}.", character.name, action_log),
    )
    .await;
    Ok(())
}
