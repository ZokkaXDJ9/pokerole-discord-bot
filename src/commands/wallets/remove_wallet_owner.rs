use crate::cache::{CharacterCacheItem, WalletCacheItem};
use crate::commands::autocompletion::{autocomplete_character_name, autocomplete_wallet_name};
use crate::commands::characters::{log_action, validate_user_input, ActionType};
use crate::commands::{
    ensure_guild_exists, find_character, find_wallet, send_ephemeral_reply, send_error, Context,
    Error,
};
use crate::data::Data;

/// Removes an existing character from the ownership of an existing wallet.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn remove_wallet_owner(
    ctx: Context<'_>,
    #[description = "What's the wallet's name?"]
    #[autocomplete = "autocomplete_wallet_name"]
    wallet_name: String,
    #[autocomplete = "autocomplete_character_name"]
    #[description = "Which character should be removed as owner?"]
    character_name: String,
) -> Result<(), Error> {
    if let Err(e) = validate_user_input(wallet_name.as_str()) {
        return send_error(&ctx, e).await;
    }

    let guild_id = ctx.guild_id().expect("Command is guild_only").get();
    ensure_guild_exists(&ctx, guild_id as i64).await;

    remove_wallet_owner_impl(ctx.data(), &wallet_name, &character_name, guild_id).await?;

    send_ephemeral_reply(
        &ctx,
        format!(
            "Successfully removed {} as owner for {}",
            character_name, wallet_name
        )
        .as_str(),
    )
    .await?;
    log_action(
        &ActionType::WalletChange,
        &ctx,
        &format!("Removed {} as owner for {}", character_name, wallet_name),
    )
    .await?;
    Ok(())
}

async fn remove_wallet_owner_impl(
    data: &Data,
    wallet_name: &str,
    character_name: &str,
    guild_id: u64,
) -> Result<(), Error> {
    let character = find_character(data, guild_id, character_name).await?;
    let wallet = find_wallet(data, guild_id, wallet_name).await?;
    remove_wallet_owner_from_db(wallet, character, data).await?;
    Ok(())
}

async fn remove_wallet_owner_from_db(
    wallet: WalletCacheItem,
    character: CharacterCacheItem,
    data: &Data,
) -> Result<(), String> {
    let result = sqlx::query!(
        "DELETE FROM wallet_owner WHERE wallet_id = ? AND character_id = ?",
        wallet.id,
        character.id,
    )
    .execute(&data.database)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 1 {
                Ok(())
            } else {
                Err("Failed to remove owner: No matching entry found.".to_string())
            }
        }
        Err(e) => Err(format!("**Something went wrong!**\n{}", e)),
    }
}
