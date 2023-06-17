use serenity::client::Context;
use serenity::model::channel::{Reaction, ReactionType};
use crate::{Error};
use crate::events::FrameworkContext;

// TODO: Move this into a database.
const ALLOWED_MESSAGE_IDS: [u64; 3] = [1119646282630303864, 1119648632291983450, 1119649167799762995];

pub async fn handle_reaction_add(ctx: &Context, _framework: FrameworkContext<'_>, reaction: &Reaction) -> Result<(), Error> {
    if ALLOWED_MESSAGE_IDS.contains(&reaction.message_id.0) {
        let bla = get_emoji_name(&reaction.emoji);
        let role_id = emoji_to_role_id(bla.as_str());
        ctx.http.add_member_role(reaction.guild_id.unwrap().0, reaction.user_id.unwrap().0, role_id, Some("Clicked the button to add the role.")).await?;
    }

    Ok(())
}

pub async fn handle_reaction_remove(ctx: &Context, _framework: FrameworkContext<'_>, reaction: &Reaction) -> Result<(), Error> {
    if ALLOWED_MESSAGE_IDS.contains(&reaction.message_id.0) {
        let bla = get_emoji_name(&reaction.emoji);
        let role_id = emoji_to_role_id(bla.as_str());
        ctx.http.remove_member_role(reaction.guild_id.unwrap().0, reaction.user_id.unwrap().0, role_id, Some("Clicked the button to remove the role.")).await?;
    }

    Ok(())
}

fn emoji_to_role_id(emoji_name: &str) -> u64 {
    match emoji_name {
        "💡" => 1119636705025200249,
        "🏖️" => 1119636734049792061,
        _ => panic!("unexpected emoji name! {}", emoji_name)
    }
}

fn get_emoji_name(reaction: &ReactionType) -> String {
    match reaction {
        ReactionType::Custom{ animated: _animated, id: _id, name} => name.clone().unwrap_or(String::new()),
        ReactionType::Unicode(value) => value.clone(),
        _ => String::new()
    }
}
