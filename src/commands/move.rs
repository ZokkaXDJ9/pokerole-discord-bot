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
                    b.add_button(create_button("Use Metronome", "metronome"))
                })
            })
    }).await?;

    reply.message().await?;
    Ok(())
}

pub fn create_button(label: &str, custom_id: &str) -> CreateButton {
    let mut button = CreateButton::default();
    button.label(label);
    button.custom_id(custom_id);
    button.style(ButtonStyle::Primary);
    button.disabled(false);
    button
}
