use crate::cache::CharacterCacheItem;
use crate::commands::autocompletion::{autocomplete_character_name, autocomplete_pokemon_type};
use crate::commands::characters::{build_character_list, change_character_stat, ActionType};
use crate::commands::{parse_variadic_args, Context, Error};
use crate::enums::PokemonTypeWithoutShadow;
use crate::errors::CommandInvocationError;

/// Reward players with a Terastallization charge.
#[allow(clippy::too_many_arguments)]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn reward_terastallization(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_pokemon_type"]
    #[description = "Which Type?"]
    tera_type: PokemonTypeWithoutShadow,
    #[description = "Whom?"]
    #[autocomplete = "autocomplete_character_name"]
    character1: String,
    #[autocomplete = "autocomplete_character_name"] character2: Option<String>,
    #[autocomplete = "autocomplete_character_name"] character3: Option<String>,
    #[autocomplete = "autocomplete_character_name"] character4: Option<String>,
    #[autocomplete = "autocomplete_character_name"] character5: Option<String>,
    #[autocomplete = "autocomplete_character_name"] character6: Option<String>,
    #[autocomplete = "autocomplete_character_name"] character7: Option<String>,
    #[autocomplete = "autocomplete_character_name"] character8: Option<String>,
    #[autocomplete = "autocomplete_character_name"] character9: Option<String>,
) -> Result<(), Error> {
    // TODO: Button to undo the transaction which lasts for a minute or so.
    let args = parse_variadic_args(
        character1, character2, character3, character4, character5, character6, character7,
        character8, character9,
    );

    match handle_unlock(&ctx, tera_type, &args).await {
        Ok(characters) => {
            ctx.say(format!(
                "Unlocked a {} Terastallization Charge for {}!",
                tera_type,
                build_character_list(&characters)
            ))
            .await?;
            Ok(())
        }
        Err(err) => Err(Box::new(err)),
    }
}

async fn handle_unlock<'a>(
    ctx: &Context<'a>,
    tera_type: PokemonTypeWithoutShadow,
    names: &Vec<String>,
) -> Result<Vec<CharacterCacheItem>, CommandInvocationError> {
    let tera_count_column = match tera_type {
        PokemonTypeWithoutShadow::Normal => "tera_count_normal",
        PokemonTypeWithoutShadow::Fighting => "tera_count_fighting",
        PokemonTypeWithoutShadow::Flying => "tera_count_flying",
        PokemonTypeWithoutShadow::Poison => "tera_count_poison",
        PokemonTypeWithoutShadow::Ground => "tera_count_ground",
        PokemonTypeWithoutShadow::Rock => "tera_count_rock",
        PokemonTypeWithoutShadow::Bug => "tera_count_bug",
        PokemonTypeWithoutShadow::Ghost => "tera_count_ghost",
        PokemonTypeWithoutShadow::Steel => "tera_count_steel",
        PokemonTypeWithoutShadow::Fire => "tera_count_fire",
        PokemonTypeWithoutShadow::Water => "tera_count_water",
        PokemonTypeWithoutShadow::Grass => "tera_count_grass",
        PokemonTypeWithoutShadow::Electric => "tera_count_electric",
        PokemonTypeWithoutShadow::Psychic => "tera_count_psychic",
        PokemonTypeWithoutShadow::Ice => "tera_count_ice",
        PokemonTypeWithoutShadow::Dragon => "tera_count_dragon",
        PokemonTypeWithoutShadow::Dark => "tera_count_dark",
        PokemonTypeWithoutShadow::Fairy => "tera_count_fairy",
    };

    match change_character_stat(
        ctx,
        tera_count_column,
        names,
        1,
        ActionType::TerastallizationUnlock,
    )
    .await
    {
        Ok(characters) => Ok(characters),
        Err(err) => Err(CommandInvocationError::new(&err)),
    }
}
