use poise::Command;
use crate::game_data::GameData;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, GameData, Error>;

mod r#move;
mod weather;
mod status;
mod rule;
mod item;
mod stats;
mod learns;
mod roll;
mod ability;
mod nature;
mod timestamp;
mod about;

mod autocompletion;
mod metronome;
mod efficiency;

pub fn get_all_commands() -> Vec<Command<GameData, Error>> {
    vec![
        roll::roll(),
        roll::r(),
        r#move::poke_move(),
        ability::ability(),
        item::item(),
        stats::stats(),
        status::status(),
        rule::rule(),
        learns::learns(),
        nature::nature(),
        timestamp::timestamp(),
        weather::weather(),
        metronome::metronome(),
        efficiency::efficiency(),
        about::about()
    ]
}
