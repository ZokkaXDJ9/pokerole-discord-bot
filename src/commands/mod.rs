use poise::{Command, ReplyHandle};
use crate::Error;
use crate::data::Data;

type Context<'a> = poise::Context<'a, Data, Error>;

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

mod characters;
mod say;
mod setting_time_offset;
mod update_user_names;

pub fn get_all_commands() -> Vec<Command<Data, Error>> {
    let mut result = vec![
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
        setting_time_offset::setting_time_offset(),
        say::say(),
        update_user_names::initialize_user_data(),
        about::about()
    ];

    for x in characters::get_all_commands() {
        result.push(x);
    }

    result
}

pub async fn send_error<'a>(ctx: &Context<'a>, content: &str) -> Result<(), Error> {
    send_ephemeral_reply(ctx, content).await?;
    Ok(())
}

pub async fn send_ephemeral_reply<'a>(ctx: &Context<'a>, content: &str) -> Result<ReplyHandle<'a>, serenity::Error> {
    ctx.send(|b| b
        .content(content)
        .ephemeral(true)
    ).await
}

#[allow(clippy::too_many_arguments)]
pub fn parse_variadic_args<T>(arg1: T,
                          arg2: Option<T>,
                          arg3: Option<T>,
                          arg4: Option<T>,
                          arg5: Option<T>,
                          arg6: Option<T>,
                          arg7: Option<T>,
                          arg8: Option<T>,
                          arg9: Option<T>) -> Vec<T> {
    let mut result = vec!(arg1);
    add_if_some(&mut result, arg2);
    add_if_some(&mut result, arg3);
    add_if_some(&mut result, arg4);
    add_if_some(&mut result, arg5);
    add_if_some(&mut result, arg6);
    add_if_some(&mut result, arg7);
    add_if_some(&mut result, arg8);
    add_if_some(&mut result, arg9);

    result
}

fn add_if_some<T>(vec: &mut Vec<T>, option: Option<T>) {
    if let Some(x) = option {
        vec.push(x);
    }
}
