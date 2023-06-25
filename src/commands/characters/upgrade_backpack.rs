use crate::commands::autocompletion::autocomplete_owned_character_name;
use crate::commands::characters::{parse_user_input_to_character, DEFAULT_BACKPACK_SLOTS};
use crate::commands::{send_error, Context, Error};
use crate::emoji;

const BASE_PRICE: i64 = 500;
const MONEY_PER_LEVEL: i64 = 500;

/// Upgrade your backpack! Requires a confirmation, so no worries about accidentally using this.
#[allow(clippy::too_many_arguments)]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn reward_money(
    ctx: Context<'_>,
    #[description = "Which character?"]
    #[autocomplete = "autocomplete_owned_character_name"]
    character: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command is guild_only").0;
    let giver_option = parse_user_input_to_character(&ctx, guild_id, &character).await;
    if giver_option.is_none() {
        return send_error(
            &ctx,
            &format!("Unable to find a character named {}", character),
        )
        .await;
    }
    let giver = giver_option.unwrap();
    let giver_record = sqlx::query!(
        "SELECT money, backpack_upgrade_count FROM character WHERE id = ?",
        giver.id
    )
    .fetch_one(&ctx.data().database)
    .await;

    if let Ok(giver_record) = giver_record {
        let required_money = BASE_PRICE + MONEY_PER_LEVEL * giver_record.backpack_upgrade_count;

        if giver_record.money < required_money {
            return send_error(
                &ctx,
                format!(
                    "**Unable to upgrade {}'s backpack.**\n*Upgrading to {} slots would require {} {}.*",
                    giver.name,
                    giver_record.backpack_upgrade_count + DEFAULT_BACKPACK_SLOTS,
                    required_money,
                    emoji::POKE_COIN
                )
                .as_str(),
            )
            .await;
        }

        // TODO: send actual confirmation message

        // TODO: Upgrade backpack size
    }

    Ok(())
}
