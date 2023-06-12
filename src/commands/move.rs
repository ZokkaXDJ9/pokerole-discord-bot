use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;
use crate::commands::{Context, Error, metronome};
use crate::commands::autocompletion::autocomplete_move;
use crate::data::r#move::Move;

/// Display a move
#[poise::command(slash_command, rename = "move")]
pub async fn poke_move(
    ctx: Context<'_>,
    #[description = "Which move?"]
    #[rename = "move"]
    #[autocomplete = "autocomplete_move"]
    name: String,
) -> Result<(), Error> {
    if let Some(poke_move) = ctx.data().moves.get(&name.to_lowercase()) {
        if poke_move.name == "Metronome" {
            execute_metronome(ctx, poke_move).await?;
        } else {
            ctx.say(poke_move.build_string()).await?;
        }
    } else {
        ctx.send(|b| {
            b.content(std::format!("Unable to find a move named **{}**, sorry! If that wasn't a typo, maybe it isn't implemented yet?", name));
            b.ephemeral(true)
        }).await?;
    }

    Ok(())
}

async fn execute_metronome<'a>(ctx: Context<'a>, poke_move: &Move) -> Result<(), Error> {
    let reply = ctx.send(|b| {
        b.content(poke_move.build_string())
            .components(|b| {
                b.create_action_row(|b| {
                    b.add_button(create_metronome_button(false))
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
                            row.add_button(create_metronome_button(true))
                        })
                    })
                })
            }).await?;

            metronome::execute(ctx).await?;
        },
        None => {
            reply.edit(ctx, |b| {
                b.components(|components| {
                    components.create_action_row(|row| {
                        row
                    })
                })
            }).await?;
        }
    };

    Ok(())
}

fn create_metronome_button(disabled: bool) -> CreateButton {
    let mut button = CreateButton::default();
    button.label("Use Metronome");
    button.custom_id("Use Metronome");
    button.style(ButtonStyle::Primary);
    button.disabled(disabled);
    button
}
