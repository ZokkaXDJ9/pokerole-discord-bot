use crate::commands::{Context, Error};

/// Blah blah blah
pub async fn about(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("\
Made with love and 🦀!

Original Pokerole System: https://www.pokeroleproject.com/
Original Pokerole CSV data taken from Pokerole-Discord.py-Base (https://github.com/XShadeSlayerXx/PokeRole-Discord.py-Base)
With lots of custom edits to make the whole pokerole system fit our custom setting better.

Raw pokemon data taken from pokeAPI (https://github.com/PokeAPI/pokeapi).\
"
    ).await?;
    Ok(())
}
