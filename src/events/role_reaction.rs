use crate::events::FrameworkContext;
use crate::Error;
use serenity::client::Context;
use serenity::model::channel::{Reaction, ReactionType};

// TODO: Move this into a database.
const ALLOWED_MESSAGE_IDS: [u64; 3] = [
    1119664626188156978,
    1119665389899616396,
    1119666186867716137,
];

pub async fn handle_reaction_add(
    ctx: &Context,
    _framework: FrameworkContext<'_>,
    reaction: &Reaction,
) -> Result<(), Error> {
    if ALLOWED_MESSAGE_IDS.contains(&reaction.message_id.0) {
        let bla = get_emoji_name(&reaction.emoji);
        let role_id = emoji_to_role_id(bla.as_str());
        ctx.http
            .add_member_role(
                reaction.guild_id.unwrap().0,
                reaction.user_id.unwrap().0,
                role_id,
                Some("Clicked the button to add the role."),
            )
            .await?;
    }

    Ok(())
}

pub async fn handle_reaction_remove(
    ctx: &Context,
    _framework: FrameworkContext<'_>,
    reaction: &Reaction,
) -> Result<(), Error> {
    if ALLOWED_MESSAGE_IDS.contains(&reaction.message_id.0) {
        let bla = get_emoji_name(&reaction.emoji);
        let role_id = emoji_to_role_id(bla.as_str());
        ctx.http
            .remove_member_role(
                reaction.guild_id.unwrap().0,
                reaction.user_id.unwrap().0,
                role_id,
                Some("Clicked the button to remove the role."),
            )
            .await?;
    }

    Ok(())
}

fn emoji_to_role_id(emoji_name: &str) -> u64 {
    match emoji_name {
        "❤️" => 1115475058958278707,
        "💙" => 1115475277611544596,
        "💛" => 1115475324264792074,
        "🍆" => 1115475361380188171,
        "🐱" => 1115475400215244861,
        "🔔" => 1115475494956179598,
        "⚙️" => 1115475607950721165,
        "🤖" => 1116615590308749352,
        "💬" => 1115475646668353577,
        "📣" => 1119659198968512655,
        _ => panic!("unexpected emoji name! {}", emoji_name),
    }
}

fn get_emoji_name(reaction: &ReactionType) -> String {
    match reaction {
        ReactionType::Custom {
            animated: _animated,
            id: _id,
            name,
        } => name.clone().unwrap_or(String::new()),
        ReactionType::Unicode(value) => value.clone(),
        _ => String::new(),
    }
}
