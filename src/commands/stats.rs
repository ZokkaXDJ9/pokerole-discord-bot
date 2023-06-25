use crate::commands::autocompletion::autocomplete_pokemon;
use crate::commands::{Context, Error};
use crate::helpers;
use serenity::builder::CreateComponents;
use std::default::Default;

async fn print_poke_stats(ctx: Context<'_>, name: String) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().game.pokemon.get(&name.to_lowercase()) {
        ctx.send(|b| {
            b.content(pokemon.build_stats_string());
            b.components(|b| {
                create_buttons(
                    b,
                    &pokemon.name.to_lowercase(),
                    pokemon.species_data.pokedex_entries.is_empty(),
                )
            })
        })
        .await?;
    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find a pokemon named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}

/// Display Pokemon stats. Same as /stats.
#[poise::command(slash_command)]
pub async fn pokemon(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    name: String,
) -> Result<(), Error> {
    print_poke_stats(ctx, name).await
}

/// Display Pokemon stats. Same as /pokemon
#[poise::command(slash_command)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    name: String,
) -> Result<(), Error> {
    print_poke_stats(ctx, name).await
}

fn create_buttons<'a>(
    b: &'a mut CreateComponents,
    name: &String,
    _are_pokedex_entries_empty: bool,
) -> &'a mut CreateComponents {
    b.create_action_row(|b| {
        b.add_button(helpers::create_button(
            "Abilities",
            format!("abilities_{}", name).as_str(),
            false,
        ));
        b.add_button(helpers::create_button(
            "Type Effectiveness",
            format!("efficiency_{}", name).as_str(),
            false,
        ));
        b.add_button(helpers::create_button(
            "Moves",
            format!("moves_{}", name).as_str(),
            false,
        ));
        // b.add_button(helpers::create_button("Pokedex Entries", format!("pokedex_{}", name).as_str(), _are_pokedex_entries_empty));
        b
    })
}
