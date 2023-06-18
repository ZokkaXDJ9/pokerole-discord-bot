use poise::Command;
use crate::Error;
use crate::game_data::GameData;

type Context<'a> = poise::Context<'a, GameData, Error>;

mod autocompletion;

pub mod r#move;
pub mod weather;
pub mod status;
pub mod rule;
pub mod item;
pub mod stats;
pub mod learns;
pub mod roll;
pub mod ability;
pub mod nature;
pub mod timestamp;
pub mod about;
pub mod metronome;
pub mod efficiency;
pub mod select_random;
pub mod poll;
pub mod scale;
pub mod emoji;
pub mod encounter;
pub mod calculate_hp_damage_modifier;
pub mod potion;
mod create_role_reaction_post;
mod initialize_character;

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
        create_role_reaction_post::create_role_reaction_post(),
        initialize_character::initialize_character(),
        about::about()
    ]
}
