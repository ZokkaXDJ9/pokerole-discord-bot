use crate::commands::{send_ephemeral_reply, send_error, Context, Error};

/// Set an offset for the timestamp command.
#[poise::command(slash_command)]
pub async fn setting_time_offset(
    ctx: Context<'_>,
    #[description = "Hours."]
    #[min = -15_i8]
    #[max = 15_i8]
    hours: i8,
    #[description = "Minutes."]
    #[min = -45_i8]
    #[max = 45_i8]
    minutes: i8,
) -> Result<(), Error> {
    let user_id = ctx.author().id.0 as i64;
    let user = sqlx::query!(
        "SELECT setting_time_offset_hours, setting_time_offset_minutes FROM user WHERE id = ?",
        user_id
    )
    .fetch_one(&ctx.data().database)
    .await;

    match user {
        Ok(_) => {
            let result = sqlx::query!(
                "UPDATE user SET setting_time_offset_hours = ?, setting_time_offset_minutes = ? WHERE id = ?",
                hours,
                minutes,
                user_id
            ).execute(&ctx.data().database).await;

            if result.is_ok() && result.unwrap().rows_affected() == 1 {
                send_ephemeral_reply(&ctx, "Successfully set your local time!").await?;
                Ok(())
            } else {
                send_error(&ctx, "Unable to update your time offsets. Mh! Weird.").await?;
                Ok(())
            }
        }
        Err(_) => {
            let result = sqlx::query!(
                "INSERT INTO user (id, setting_time_offset_hours, setting_time_offset_minutes) VALUES (?, ?, ?) RETURNING id",
                user_id,
                hours,
                minutes,
            ).fetch_one(&ctx.data().database).await;

            if result.is_ok() {
                send_ephemeral_reply(&ctx, "Successfully set your local time!").await?;
                Ok(())
            } else {
                send_error(&ctx, "Unable to create a user entry for you. Mh! Weird.").await?;
                Ok(())
            }
        }
    }
}
