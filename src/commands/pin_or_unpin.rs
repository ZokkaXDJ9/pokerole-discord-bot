use crate::commands::{Context, Error, send_ephemeral_reply};
use crate::errors::CommandInvocationError;

/// Pin or Unpin a message
#[poise::command(context_menu_command = "Pin or Unpin")]
pub async fn pin_or_unpin(
    ctx: Context<'_>,
    #[description = "Message which should be (un)pinned."]
    message: poise::serenity_prelude::Message,
) -> Result<(), Error> {
    match if message.pinned {
        message.unpin(&ctx).await
    } else {
        message.pin(&ctx).await
    } {
        Ok(_) => {
            let _ = send_ephemeral_reply(&ctx, "Done!").await;
            Ok(())
        }
        Err(e) => {
            let pin_or_unpin = if message.pinned { "unpin" } else { "pin" };
            Err(Box::new(
                CommandInvocationError::new(&format!(
                    "Failed to {} message.\n```{}```",
                    pin_or_unpin, e
                ))
                .log(),
            ))
        }
    }
}
