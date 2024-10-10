use crate::commands::autocompletion::autocomplete_zmove;
use crate::commands::{Context, Error};
use crate::game_data::zmove::ZMove;
use poise::CreateReply;

#[poise::command(slash_command, rename = "z_move")]
pub async fn z_move(
    ctx: Context<'_>,
    #[description = "Which Z-Move?"]
    #[autocomplete = "autocomplete_zmove"]
    name: String,
) -> Result<(), Error> {
    if let Some(z_move) = ctx.data().game.z_moves.get(&name.to_lowercase()) {
        ctx.say(z_move.build_string()).await?;
    } else {
        ctx.send(
            CreateReply::default()
                .content(format!(
                    "Unable to find a Z-Move named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?",
                    name
                ))
                .ephemeral(true),
        )
        .await?;
    }

    Ok(())
}
