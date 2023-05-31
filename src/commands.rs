use std::cmp::Ordering;
use futures::StreamExt;
use crate::data::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn autocomplete_move<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> Vec<String> {
    let names = &_ctx.data().move_names;

    let lower_case = &partial.to_lowercase();

    let mut result: Vec<String> = names.iter()
        .filter(move |x| x.to_lowercase().contains(lower_case))
        .map(|x| x.clone())
        .collect();

    result.sort_by(|a, b| {
        if a.to_lowercase().starts_with(lower_case) {
            return Ordering::Less;
        }
        if b.to_lowercase().starts_with(lower_case) {
            return Ordering::Greater;
        }

        Ordering::Equal
    });

    return result;
}

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

/// Receive Magicarps' blessings!
#[poise::command(slash_command, rename = "move")]
pub async fn poke_move(
    ctx: Context<'_>,
    #[description = "Which move?"]
    #[rename = "move"]
    #[autocomplete = "autocomplete_move"]
    poke_move_name: String,
) -> Result<(), Error> {
    if let Some(poke_move) = ctx.data().moves.get(&poke_move_name) {
        ctx.say(std::format!("{:?}", poke_move)).await?;
        return Ok(());
    }

    ctx.say("Move not found. Oh no!").await?;
    Ok(())
}
