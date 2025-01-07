use crate::commands::{Context, Error};

/// Blah blah blah
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    // TODO: Might be funny to add an actual version string here once we use proper tags and git actions and such :>
    ctx.say("\
```Omniscient Xatu Guide```
Made with love, cookies and <:ferrishappy:1272535933505503232>!
**You may use this bot on other discord servers as you see fit. Character data is server specific, so there's no need to worry about breaking anything!**
For help and support, check out the bot development & support discord server: <https://discord.gg/NMy7HgdV8H>

## Semi-Useful Links
- Source Code: [GitHub Repository](<https://github.com/ZokkaXDJ9/pokerole-discord-bot>)
- Custom Bot Data: [GitHub Repository](<https://github.com/ZokkaXDJ9/pokerole-custom-data>)
### External Data Sources
- **Original Pokerole System**: <https://www.pokeroleproject.com>
- Pokerole specific data is taken from **Pokerole-Data** and modified by us [[Link](<https://github.com/Pokerole-Software-Development/Pokerole-Data>)]
- Raw pokemon data taken from **pokeAPI** [[Link](<https://github.com/PokeAPI/pokeapi>) – [License](<https://github.com/PokeAPI/pokeapi/blob/master/LICENSE.md>)]
Thanks to all of these projects for the effort they put in! <3
"
    ).await?;
    Ok(())
}
