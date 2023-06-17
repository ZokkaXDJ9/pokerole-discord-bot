use serenity::model::channel::Channel;
use serenity::model::guild::Role;
use serenity::utils::MessageBuilder;
use crate::commands::{Context, Error};
use serenity::model::channel::ReactionType;

/// WIP. Create a post for role reactions.
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
#[allow(clippy::too_many_arguments)]
pub async fn create_role_reaction_post(
    ctx: Context<'_>,
    #[description = "In which channel?"]
    channel: Channel,
    #[description = "What should be the post title?"]
    text: String,
    #[description = "The emoji for the first role"]
    emoji_1: String,
    #[description = "The role for the first role"]
    role_1: Role,

    emoji_2: String,
    role_2: Role,
) -> Result<(), Error> {
    let mut reaction_message = MessageBuilder::default();
    let mut command_response = MessageBuilder::default();
    let mut reactions = Vec::default();
    reaction_message.push_bold_line(text);

    add_role(&mut reaction_message, &mut command_response, emoji_1, role_1, &mut reactions);
    add_role(&mut reaction_message, &mut command_response, emoji_2, role_2, &mut reactions);

    let message = channel.id().send_message(ctx, |f| f
        .content(reaction_message)
    ).await?;

    for x in reactions {
        message.react(ctx, ReactionType::Unicode(x)).await?;
    }

    ctx.say(format!("Message created.\nMessageId: `{}`\n{}", message.id, command_response)).await?;

    Ok(())
}

fn add_role(reaction_message: &mut MessageBuilder, command_response: &mut MessageBuilder, emoji: String, role: Role, all_emojis: &mut Vec<String>) {
    command_response.push(&emoji);
    command_response.push_mono(role.id);
    command_response.push(' ');
    command_response.push_mono_line("@".to_owned() + &role.name);

    reaction_message.push(&emoji);
    reaction_message.push(' ');
    reaction_message.push_line(role.name);

    all_emojis.push(emoji);
}
