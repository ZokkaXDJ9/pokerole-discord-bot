use log::info;
use poise::{Event, FrameworkContext};
use crate::Error;
use crate::game_data::GameData;

pub async fn handle_events(
    serenity_ctx: &serenity::client::Context,
    event: &Event<'_>,
    framework_ctx: FrameworkContext<'_, GameData, Error>,
    data: &GameData
) -> Result<(), Error> {
    info!("handle_events got an event: {} [{:?}]", event.name(), event);
    match event {
        Event::Message {new_message} => Ok(()),
        Event::InteractionCreate {interaction} => Ok(()),
        _ => {Ok(())}
    }
}
