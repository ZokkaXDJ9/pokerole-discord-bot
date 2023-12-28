use crate::commands::characters::update_character_post;
use crate::commands::{send_ephemeral_reply, Context, Error};

/// Temporary command to make stuff look prettier everywhere.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn update_all_character_posts(ctx: Context<'_>) -> Result<(), Error> {
    let _ = send_ephemeral_reply(&ctx, "Request received. Working on it!").await;

    for i in 0..55 {
        let _ = update_character_post(&ctx, i).await;
    }

    let _ = send_ephemeral_reply(&ctx, "DONE!").await;
    Ok(())
}
