use poise::Command;
use crate::Error;
use crate::game_data::GameData;

type Context<'a> = poise::Context<'a, GameData, Error>;

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
mod select_random;
mod poll;
mod scale;
mod emoji;
mod encounter;
mod calculate_hp_damage_modifier;
mod potion;

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
        select_random::select_random(),
        poll::poll(),
        scale::scale(),
        emoji::emoji(),
        encounter::encounter(),
        potion::potion(),
        calculate_hp_damage_modifier::calculate_hp_damage_modifier(),
        about::about()
    ]
}
