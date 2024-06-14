use crate::commands::{send_ephemeral_reply, send_error, Context, Error};
use crate::errors::ValidationError;

/// Don't touch this. Registers this server as an emoji source in case official servers are at capacity.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn setup_emoji_guild(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command is guild_only").get() as i64;

    // Ensure this server isn't set up as a regular guild
    if sqlx::query!("SELECT * FROM guild WHERE id = ?", guild_id,)
        .fetch_optional(&ctx.data().database)
        .await
        .unwrap()
        .is_some()
    {
        return Err(Box::new(ValidationError::new("This server has already been set up as a regular guild. You probably don't want to use this command here.")));
    }

    let record = sqlx::query!(
        "INSERT INTO emoji_guild (id, emoji_count) VALUES (?, ?)",
        guild_id,
        0,
    )
    .execute(&ctx.data().database)
    .await;

    if record.is_ok() {
        send_ephemeral_reply(&ctx, "Emoji Guild has been successfully initialized!").await?;
        Ok(())
    } else {
        send_error(
            &ctx,
            "Something went wrong! Has this server already been initialized?",
        )
        .await
    }
}
