use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;
use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;

/// Display Pokemon moves
#[poise::command(slash_command, prefix_command)]
pub async fn pokelearns(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    name: String,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&name.to_lowercase()) {
        let reply = ctx.send(|b| {
            b.content(pokemon.build_move_string())
                .components(|b| {
                    b.create_action_row(|b| {
                        b.add_button(create_button(false))
                    })
                })
        }).await?;

        let message = reply.message().await?;
        let interaction = message
            .await_component_interaction(ctx)
            .timeout(std::time::Duration::from_secs(3 * 60))
            .await;

        match &interaction {
            Some(m) => {
                m.create_interaction_response(ctx, |response| {
                    response.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
                        d.components(|b| {
                             b.create_action_row( |row| {
                                 row.add_button(create_button(true))
                             })
                        })
                    })
                }).await?;

                ctx.send(|b| b.content(pokemon.build_tm_move_string())).await?;
            },
            None => {
                reply.edit(ctx, |b| {
                    b.components(|components| {
                        components.create_action_row( |row| {
                             row.add_button(create_button(true))
                        })
                    })
                }).await?;
            }
        };

        return Ok(());
    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find a pokemon named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}

fn create_button(disabled: bool) -> CreateButton {
    let mut button = CreateButton::default();
    button.label("Show TM Moves");
    button.custom_id("Show TM Moves");
    button.style(ButtonStyle::Primary);
    button.disabled(disabled);
    button
}


