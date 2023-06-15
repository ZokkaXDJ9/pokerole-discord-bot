use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;
use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::data::pokemon::Pokemon;
use crate::helpers;

/// Display Pokemon moves
#[poise::command(slash_command, prefix_command)]
pub async fn learns(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    name: String,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&name.to_lowercase()) {
        list_learns(ctx, pokemon).await?;
    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find a pokemon named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}

pub(in crate::commands) async fn list_learns<'a>(ctx: Context<'a>, pokemon: &Pokemon) -> Result<(), Error> {
    let reply = ctx.send(|b| {
        b.content(pokemon.build_move_string())
            .components(|b| {
                b.create_action_row(|b| {
                    b.add_button(helpers::create_button("Show All Learnable Moves", format!("learns-all_{}", pokemon.name.to_lowercase()).as_str()))
                })
            })
    }).await?;

    Ok(())
}

fn create_button(disabled: bool) -> CreateButton {
    let mut button = CreateButton::default();
    button.label("Show All Learnable Moves");
    button.custom_id("Show All Learnable Moves");
    button.style(ButtonStyle::Primary);
    button.disabled(disabled);
    button
}


