use std::default::Default;
use std::sync::Arc;
use poise::ReplyHandle;
use serenity::builder::{CreateButton, CreateComponents};
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use crate::commands::{Context, efficiency, Error, learns};
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::data::pokemon::Pokemon;

/// Display Pokemon stats
#[poise::command(slash_command)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    name: String,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&name.to_lowercase()) {
        let mut button_states = ButtonStates {moves: false, abilities: false, type_effectiveness: false};

        let reply = ctx.send(|b| {
            b.content(pokemon.build_stats_string());
            b.components(|b| create_buttons(&button_states, b))
        }).await?;

        let message = reply.message().await?;

        let mut interaction_count: u8 = 0;
        while interaction_count < 3 {
            let interaction = message
                .await_component_interaction(ctx)
                .timeout(std::time::Duration::from_secs(3 * 60))
                .await;

            match_interaction(ctx, &mut button_states, pokemon, &reply, interaction).await?;
            interaction_count += 1;
        }

    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find a pokemon named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}

struct ButtonStates {
    pub moves: bool,
    pub abilities: bool,
    pub type_effectiveness: bool,
}

fn create_buttons<'a>(button_states: &ButtonStates, b: &'a mut CreateComponents) -> &'a mut CreateComponents {
    b.create_action_row(|b| {
        b.add_button(create_button("Abilities", button_states.abilities));
        b.add_button(create_button("Type Effectiveness", button_states.type_effectiveness));
        b.add_button(create_button("Moves", button_states.moves))
    })
}

fn create_button(label: &str, is_disabled: bool) -> CreateButton {
    let mut button = CreateButton::default();
    button.label(label);
    button.custom_id(label);
    button.style(ButtonStyle::Primary);
    button.disabled(is_disabled);
    button
}

async fn match_interaction<'a>(ctx: Context<'a>, button_states: &mut ButtonStates, pokemon: &Pokemon,  reply: &'a ReplyHandle<'a>, interaction: Option<Arc<MessageComponentInteraction>>) -> Result<(), Error> {
    match &interaction {
        Some(m) => {
            if m.data.custom_id == "Moves" {
                button_states.moves = true;
            } else if m.data.custom_id == "Abilities" {
                button_states.abilities = true;
            } else {
                button_states.type_effectiveness = true;
            }

            m.create_interaction_response(ctx, |response| {
                response.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
                    d.components(|b| create_buttons(button_states,b))
                })
            }).await?;

            if m.data.custom_id == "Moves" {
                learns::list_learns(ctx, pokemon).await?;
            } else if m.data.custom_id == "Abilities" {
                ctx.send(|b| b.content(pokemon.build_ability_string(&ctx.data().abilities))).await?;
            } else {
                ctx.send(|b| b.content(efficiency::get_type_resistances_string(pokemon, &ctx.data().type_efficiency))).await?;
            }
        },
        None => {
            reply.edit(ctx, |b| {
                b.components(|components| {
                    components
                })
            }).await?;
        }
    };

    Ok(())
}
