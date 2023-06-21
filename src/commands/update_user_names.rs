use serenity::model::id::UserId;
use serenity::model::id::GuildId;
use crate::commands::{Context, Error, send_ephemeral_reply, send_error};

/// Initialize user data. Ideally, we use this command once and then yeet it out of existence.
#[poise::command(slash_command, owners_only)]
pub async fn initialize_user_data(
    ctx: Context<'_>,
) -> Result<(), Error> {
    send_ephemeral_reply(&ctx, "Updating user data...").await?;

    let records = sqlx::query!("SELECT DISTINCT user_id, guild_id FROM character")
        .fetch_all(&ctx.data().database)
        .await;

    if let Ok(records) = records {
        for record in records {
            let result = sqlx::query!("INSERT INTO user (id) VALUES (?)",
                record.user_id
            ).execute(&ctx.data().database).await;
            if result.is_ok() {
                ctx.say(format!("Set up user row for <@{}>", record.user_id)).await?;
            } else {
                ctx.say(format!("Failed to set up user row for <@{}>, assuming it exists.", record.user_id)).await?;
            }

            let user = UserId::from(record.user_id as u64)
                .to_user(ctx).await;
            if let Ok(user) = user {
                let nickname =  user.nick_in(ctx, GuildId::from(record.guild_id as u64)).await
                    .unwrap_or(user.name.clone());
                let result = sqlx::query!("INSERT INTO user_in_guild (user_id, guild_id, name) VALUES (?, ?, ?)",
                    record.user_id,
                    record.guild_id,
                    nickname
                ).execute(&ctx.data().database).await;

                if result.is_ok() {
                    ctx.say(format!("Set up user_in_guild row for <@{}>", record.user_id)).await?;
                } else {
                    ctx.say(format!("Failed to set up user_in_guild row for <@{}>???", record.user_id)).await?;
                }
            } else {
                ctx.say(format!("Unable to get user for <@{}>", record.user_id)).await?;
            }
        }
    } else {
        send_error(&ctx, "Unable to query character database!").await?;
    }

    Ok(())
}
