use crate::cache::CharacterCacheItem;
use crate::commands::autocompletion::autocomplete_character_name;
use crate::commands::characters::{change_character_stat_after_validation, log_action, ActionType};
use crate::commands::{parse_user_input_to_character, send_error, Context, Error};

/// Reward players for giving a combat tutorial to new players.
#[allow(clippy::too_many_arguments)]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn reward_giving_combat_tutorial(
    ctx: Context<'_>,
    #[description = "Who gave the tutorial?"]
    #[autocomplete = "autocomplete_character_name"]
    character: String,
) -> Result<(), Error> {
    // TODO: Button to undo the transaction which lasts for a minute or so.
    match handle_giving_combat_tutorial(&ctx, character).await {
        Ok(result) => {
            ctx.say(format!(
                "Tracked that {} gave a combat tutorial. They gained {} experience points as a reward!",
                &result.organizer.name, result.experience_value,
            ))
            .await?;
        }
        Err(err) => {
            send_error(&ctx, err.as_str()).await?;
        }
    }

    Ok(())
}

struct CombatTutorialResult {
    organizer: CharacterCacheItem,
    experience_value: i64,
}

struct CombatTutorialRewardSettings {
    new_player_combat_tutorial_reward: i64,
}

async fn handle_giving_combat_tutorial<'a>(
    ctx: &Context<'a>,
    character_name: String,
) -> Result<CombatTutorialResult, String> {
    let guild_id = ctx
        .guild_id()
        .expect("Commands using this function are marked as guild_only")
        .get();

    let guild_id_i64 = guild_id as i64;

    let settings = sqlx::query_as!(
        CombatTutorialRewardSettings,
        "SELECT new_player_combat_tutorial_reward FROM guild WHERE id = ?",
        guild_id_i64
    )
    .fetch_one(&ctx.data().database)
    .await
    .expect("Every guild should have settings!");

    match parse_user_input_to_character(ctx.data(), guild_id, &character_name).await {
        Some(character) => {
            track_combat_tutorial_for_character(
                ctx,
                settings.new_player_combat_tutorial_reward,
                &character,
            )
            .await;

            let result = CombatTutorialResult {
                organizer: character,
                experience_value: settings.new_player_combat_tutorial_reward,
            };

            Ok(result)
        }
        None => Err(String::from("Unable to find a character with that name.")),
    }
}

async fn track_combat_tutorial_for_character<'a>(
    ctx: &Context<'a>,
    reward_amount: i64,
    character: &CharacterCacheItem,
) {
    let current = sqlx::query!(
        "SELECT total_new_player_combat_tutorial_count FROM character WHERE id = ?",
        character.id
    )
    .fetch_one(&ctx.data().database)
    .await
    .expect("Character with cached ID should exist.");

    let new_value = current.total_new_player_combat_tutorial_count + 1;

    let _ = sqlx::query!(
        "UPDATE character SET total_new_player_combat_tutorial_count = ? WHERE id = ?",
        new_value,
        character.id
    )
    .execute(&ctx.data().database)
    .await;

    let _ = log_action(
        &ActionType::NewPlayerCombatTutorial,
        ctx,
        &format!("{} gave a combat tutorial!", character.name),
    )
    .await;

    let _ = change_character_stat_after_validation(
        ctx,
        "experience",
        character,
        reward_amount,
        &ActionType::Reward,
    )
    .await;
}
