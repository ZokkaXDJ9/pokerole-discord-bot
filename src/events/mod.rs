use log::info;
use poise::{Event};
use serenity::client::Context;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use crate::{commands, Error};
use crate::game_data::GameData;

type FrameworkContext<'a> = poise::FrameworkContext<'a, GameData, Error>;

pub async fn handle_events<'a>(
    context: &'a Context,
    event: &'a Event<'a>,
    framework: FrameworkContext<'a>
) -> Result<(), Error> {
    match event {
        Event::InteractionCreate {interaction} => handle_interaction(context, framework, interaction).await,
        _ => Ok(())
    }
}

async fn handle_interaction(context: &Context, framework: FrameworkContext<'_>, interaction: &Interaction) -> Result<(), Error> {
    match interaction {
        Interaction::MessageComponent(component) => handle_message_component_interaction(context, framework, component).await,
        _ => Ok(())
    }
}

async fn handle_message_component_interaction(context: &Context, framework: FrameworkContext<'_>, interaction: &MessageComponentInteraction) -> Result<(), Error> {
    info!("Got a message component interaction event: {:?}", interaction);

    if interaction.data.custom_id == "Use Metronome" {
        interaction.create_interaction_response(context, |f| f
            .kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|b|
                b.components(|x| x
                    .create_action_row(|row| row
                        .add_button(commands::r#move::create_metronome_button(true))))
            )
        ).await?;

        interaction.message.reply(context, commands::metronome::get_metronome_text(framework.user_data)).await?;
    }

    Ok(())
}
