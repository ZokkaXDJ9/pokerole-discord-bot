use crate::data::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub(crate) mod r#move;
pub(crate) mod weather;
pub(crate) mod status;
pub(crate) mod rule;
pub(crate) mod item;
pub(crate) mod stats;
pub(crate) mod pokelearns;
pub(crate) mod roll;
pub(crate) mod ability;

mod autocompletion;
mod about;
