use crate::commands::{Context, Error};

/// Blah blah blah
pub async fn about(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Raw pokemon data courtesy by pokeapi (https://github.com/PokeAPI/pokeapi).").await?;
    Ok(())
}
