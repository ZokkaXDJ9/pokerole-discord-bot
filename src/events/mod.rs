use log::info;
use poise::{Event};
use crate::Error;
use crate::game_data::GameData;

type FrameworkContext<'a> = poise::FrameworkContext<'a, GameData, Error>;

pub async fn handle_events<'a>(
    serenity_ctx: &'a serenity::client::Context,
    event: &'a Event<'a>,
    framework_ctx: FrameworkContext<'a>
) -> Result<(), Error> {
    info!("handle_events got an event: {} [{:?}]", event.name(), event);
    match event {
        Event::InteractionCreate {interaction} => {},
        _ => {}
    }

    Ok(())
}
