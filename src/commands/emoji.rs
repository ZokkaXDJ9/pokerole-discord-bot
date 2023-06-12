use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::data::pokemon::Pokemon;


const GEN5_ANIMATED: &str = "https://github.com/PokeAPI/sprites/blob/master/sprites/pokemon/versions/generation-v/black-white/animated/";
const GEN5_ANIMATED_FEMALE: &str = "https://github.com/PokeAPI/sprites/blob/master/sprites/pokemon/versions/generation-v/black-white/animated/female/";

const FRONT_MALE: &str = "https://github.com/PokeAPI/sprites/blob/master/sprites/pokemon/";
const FRONT_FEMALE: &str = "https://github.com/PokeAPI/sprites/blob/master/sprites/pokemon/female/";

fn bla(pokemon: &Pokemon) -> String {
    let mut result = String::from("");

    result.push_str(std::format!("\
## If it existed in Gen 5
Animated: <{}{}.gif>
Female Animated (unless it's the same): <{}{}.gif>
## Front Sprite:
Male: <{}{}.png>
Female (unless it's the same): <{}{}.png>
",
        GEN5_ANIMATED, pokemon.number,
        GEN5_ANIMATED_FEMALE, pokemon.number,
        FRONT_MALE, pokemon.number,
        FRONT_FEMALE, pokemon.number,
    ).as_str());

    result
}

/// Display links to fancy emojis!
#[poise::command(slash_command)]
pub async fn emoji(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    name: String,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&name.to_lowercase()) {
        ctx.say(bla(pokemon)).await?;
    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find a pokemon named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}
