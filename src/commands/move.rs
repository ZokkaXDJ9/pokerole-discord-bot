use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_move;

/// Display a move
#[poise::command(slash_command, rename = "move")]
pub async fn poke_move(
    ctx: Context<'_>,
    #[description = "Which move?"]
    #[rename = "move"]
    #[autocomplete = "autocomplete_move"]
    name: String,
) -> Result<(), Error> {
    if let Some(poke_move) = ctx.data().moves.get(&name.to_lowercase()) {
        ctx.say(poke_move.build_string()).await?;
    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find a move named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}
