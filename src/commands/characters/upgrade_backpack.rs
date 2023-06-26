use crate::commands::autocompletion::autocomplete_owned_character_name;
use crate::commands::characters::{
    parse_user_input_to_character, ActionType, DEFAULT_BACKPACK_SLOTS,
};
use crate::commands::{characters, send_error, Context, Error};
use crate::emoji;
use serenity::collector::ReactionAction;
use serenity::model::channel::ReactionType;
use std::time::Duration;

const BASE_PRICE: i64 = 500;
const MONEY_PER_LEVEL: i64 = 500;

/// Upgrade your backpack! Requires a confirmation, so no worries about accidentally using this.
#[allow(clippy::too_many_arguments)]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn upgrade_backpack(
    ctx: Context<'_>,
    #[description = "Which character?"]
    #[autocomplete = "autocomplete_owned_character_name"]
    character: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command is guild_only").0;
    let character_option = parse_user_input_to_character(&ctx, guild_id, &character).await;
    if character_option.is_none() {
        return send_error(
            &ctx,
            &format!("Unable to find a character named {}", character),
        )
        .await;
    }
    let character = character_option.unwrap();
    let giver_record = sqlx::query!(
        "SELECT money, backpack_upgrade_count FROM character WHERE id = ?",
        character.id
    )
    .fetch_one(&ctx.data().database)
    .await;

    if let Ok(giver_record) = giver_record {
        let required_money = BASE_PRICE + MONEY_PER_LEVEL * giver_record.backpack_upgrade_count;

        let target_slots = DEFAULT_BACKPACK_SLOTS + giver_record.backpack_upgrade_count + 1;

        if giver_record.money < required_money {
            return send_error(
                &ctx,
                format!(
                    "**Unable to upgrade {}'s backpack.**\n*Upgrading to {} slots would require {} {}.*",
                    character.name,
                    target_slots,
                    required_money,
                    emoji::POKE_COIN
                )
                .as_str(),
            )
            .await;
        }

        let result = ctx.say(format!("**Upgrading {}'s backpack to {} slots will require {} {}.**\n*React with {} if you wish to proceed. Any other reaction will cancel this, and any reactions made by other users will be ignored.*",
                                     character.name,
                                     target_slots,
                                     required_money,
                                     emoji::POKE_COIN,
                                     emoji::UNICODE_CHECKMARK_BUTTON,
        ).as_str()).await?;
        let message = result.message().await?;

        message
            .react(
                ctx,
                ReactionType::Unicode(emoji::UNICODE_CHECKMARK_BUTTON.to_string()),
            )
            .await?;
        message
            .react(
                ctx,
                ReactionType::Unicode(emoji::UNICODE_CROSS_MARK_BUTTON.to_string()),
            )
            .await?;

        let interaction = message
            .await_reaction(ctx)
            .author_id(ctx.author().id)
            .timeout(Duration::from_secs(69))
            .await;

        if let Some(interaction) = interaction {
            if let ReactionAction::Added(reaction) = interaction.as_ref() {
                if reaction.emoji.unicode_eq(emoji::UNICODE_CHECKMARK_BUTTON) {
                    let updated_money = giver_record.money - required_money;
                    let updated_backpack_upgrade_count = giver_record.backpack_upgrade_count + 1;

                    let query_result = sqlx::query!(
                        "UPDATE character SET money = ?, backpack_upgrade_count = ? WHERE id = ? AND money = ? and backpack_upgrade_count = ?",
                        updated_money,
                        updated_backpack_upgrade_count,
                        character.id,
                        giver_record.money,
                        giver_record.backpack_upgrade_count,
                    )
                    .execute(&ctx.data().database)
                    .await;

                    if query_result.is_ok() && query_result.unwrap().rows_affected() == 1 {
                        characters::log_action(
                            &ActionType::Payment,
                            &ctx,
                            format!(
                                "Removed {} {} from {}",
                                required_money,
                                emoji::POKE_COIN,
                                character.name,
                            )
                            .as_str(),
                        )
                        .await?;
                        characters::log_action(
                            &ActionType::BackpackUpgrade,
                            &ctx,
                            format!("Increased {}'s backpack size by 1", character.name).as_str(),
                        )
                        .await?;

                        result
                            .edit(ctx, |f| {
                                f.content(
                                    message.content.to_owned() + "\n\n**Upgrade successful!**",
                                )
                            })
                            .await?;

                        characters::update_character_post(&ctx, character.id).await?;
                        return Ok(());
                    }
                } else {
                    result
                        .edit(ctx, |f| {
                            f.content(message.content.to_owned() + "\n\n**Request was cancelled.**")
                        })
                        .await?;

                    return Ok(());
                }
            }

            result
                .edit(ctx, |f| f.content(message.content.to_owned() + "\n\n**Something went wrong.**\n*This should only happen if you're actively trying to game the system... and if that's the case, thanks for trying, but... please stop? xD*"))
                .await?;
            return Ok(());
        }

        result
            .edit(ctx, |f| {
                f.content(
                    message.content.to_owned()
                        + "\n\n**Request timed out. Use the command again if needed.**",
                )
            })
            .await?;
    }

    Ok(())
}
