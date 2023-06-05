use crate::commands::{Context, Error};

/// Blah blah blah
#[poise::command(slash_command)]
pub async fn about(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("\
Made with love and 🦀!

## Data Sources:
- Original Pokerole System: https://www.pokeroleproject.com/
- Weather & Status Effects taken from Pokerole-Discord.py-Base (https://github.com/XShadeSlayerXx/PokeRole-Discord.py-Base)
- All other Pokerole specific data is taken from Pokerole-Data (https://github.com/Pokerole-Software-Development/Pokerole-Data)
- Raw pokemon data taken from pokeAPI (https://github.com/PokeAPI/pokeapi).
Thanks to all of these projects for the effort they put in! <3
"
    ).await?;
    Ok(())
}
