use crate::commands::{Context, Error};
use crate::commands::characters::{ActionType, log_action};
use crate::errors::CommandInvocationError;

/// Store your GM Experience after a quest.
#[poise::command(slash_command, guild_only)]
pub async fn store_gm_experience(ctx: Context<'_>, amount: i64) -> Result<(), Error> {
    let user_id = ctx.author().id.get() as i64;
    let guild_id = ctx.guild().expect("Command is guild_only!").id.get() as i64;

    match sqlx::query!(
        "SELECT gm_experience FROM user_in_guild WHERE user_id = ? AND guild_id = ?",
        user_id,
        guild_id
    )
    .fetch_one(&ctx.data().database)
    .await
    {
        Ok(record) => {
            let new_amount = record.gm_experience + amount;
            match sqlx::query!(
                "UPDATE user_in_guild SET gm_experience = ? WHERE user_id = ? AND guild_id = ?",
                new_amount,
                user_id,
                guild_id
            )
            .execute(&ctx.data().database)
                .await
            {
                Ok(_) => {
                    let _ = ctx.say(format!("Applied {} GM Experience to {}!", amount, ctx.author())).await;
                    let _ = log_action(&ActionType::StoreGMExperience, &ctx, &format!("Stored {} GM Experience.", amount)).await;
                }
                Err(e) => {
                    return Err(Box::new(
                        CommandInvocationError::new(&format!(
                            "Something went wrong when applying GM Experience for a user with id {} in guild with id {}!\n```{:?}```",
                            user_id, guild_id, e
                        ))
                            .log(),
                    ))
                }
            }
        }
        Err(e) => {
            return Err(Box::new(
                CommandInvocationError::new(&format!(
                "Was unable to find a user with id {} in guild with id {} in database!\n```{:?}```",
                user_id, guild_id, e
            ))
                .log(),
            ))
        }
    };

    Ok(())
}
