use serenity::model::id::UserId;
use serenity::model::id::GuildId;
use crate::commands::{Context, Error, send_ephemeral_reply, send_error};

/// Force an update to our local user name cache.
#[poise::command(slash_command, owners_only, global_cooldown = 6000)]
pub async fn update_user_names(
    ctx: Context<'_>,
) -> Result<(), Error> {
    send_ephemeral_reply(&ctx, "## Updating user data\nStep 1/2: Fetching all user names from discord...").await?;

    let records = sqlx::query!("SELECT user_id, guild_id FROM user_in_guild")
        .fetch_all(&ctx.data().database)
        .await;

    if let Ok(records) = records {
        for record in records {
            let user = UserId::from(record.user_id as u64)
                .to_user(ctx).await;
            if let Ok(user) = user {
                // TODO: Once serenity supports the new display names, default to those instead.
                let nickname =  user.nick_in(ctx, GuildId::from(record.guild_id as u64)).await
                    .unwrap_or(user.name.clone());
                let result = sqlx::query!("UPDATE user_in_guild SET name = ? WHERE user_id = ? AND guild_id = ?",
                    nickname,
                    record.user_id,
                    record.guild_id,
                ).execute(&ctx.data().database).await;

                if result.is_err() {
                    send_ephemeral_reply(&ctx, &format!("Failed to set up user_in_guild row for <@{}> ???", record.user_id)).await?;
                }
            } else {
                send_ephemeral_reply(&ctx, &format!("Unable to get user for <@{}>", record.user_id)).await?;
            }
        }

        send_ephemeral_reply(&ctx, "**Step 2/2: Rebuilding character name cache...**").await?;
        ctx.data().cache.update_character_names(&ctx.data().database).await;
        send_ephemeral_reply(&ctx, "**Done!**").await?;
    } else {
        send_error(&ctx, "Unable to query character database!").await?;
    }

    Ok(())
}
