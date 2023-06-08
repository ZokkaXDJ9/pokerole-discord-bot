use crate::commands::{Context, Error};
use rand::seq::SliceRandom;

/// Use the most randomest of moves!
#[poise::command(slash_command)]
pub async fn metronome(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let move_name = ctx.data().move_names.choose(&mut rand::thread_rng()).expect("There should be a name.");
    if let Some(poke_move) = ctx.data().moves.get(&move_name.to_lowercase()) {
        ctx.say(poke_move.build_string()).await?;
    } else {
        ctx.say(std::format!("Error: randomness rolled {}, but there was no move with that name defined?", move_name)).await?;
    }

    Ok(())
}
