use crate::commands::{Context, Error};

/// Make Xatu say something.
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn say(ctx: Context<'_>, text: String) -> Result<(), Error> {
    ctx.say(format!("```[{}]```", text)).await?;
    Ok(())
}
