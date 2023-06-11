use std::string::ToString;
use serenity::model::channel::ReactionType;
use serenity::utils::MessageBuilder;
use crate::commands::{Context, Error};

const ONE: &str = "1️⃣";
const TWO: &str = "2️⃣";
const THREE: &str = "3️⃣";
const FOUR: &str = "4️⃣";
const FIVE: &str = "5️⃣";
const SIX: &str = "6️⃣";
const SEVEN: &str = "7️⃣";
const EIGHT: &str = "8️⃣";
const NINE: &str = "9️⃣";

fn append(builder: &mut MessageBuilder, emoji: &str, option: &String) {
    builder.push(emoji);
    builder.push("   ");
    builder.push_line(option);
}

fn append_option(builder: &mut MessageBuilder, emoji: &str, option: &Option<String>) {
    if let Some(option) = option {
        append(builder, emoji, option);
    }
}

/// Create a poll!
#[poise::command(slash_command)]
#[allow(clippy::too_many_arguments)]
pub async fn poll(
    ctx: Context<'_>,
    #[description = "You can use \n to create linebreaks."]
    question: String,
    option1: String,
    option2: String,
    option3: Option<String>,
    option4: Option<String>,
    option5: Option<String>,
    option6: Option<String>,
    option7: Option<String>,
    option8: Option<String>,
    option9: Option<String>
) -> Result<(), Error> {
    let mut builder = MessageBuilder::default();
    builder.push_bold_line(question.replace("\\n", "\n"));
    append(&mut builder, ONE, &option1);
    append(&mut builder, TWO, &option2);
    append_option(&mut builder, THREE, &option3);
    append_option(&mut builder, FOUR, &option4);
    append_option(&mut builder, FIVE, &option5);
    append_option(&mut builder, SIX, &option6);
    append_option(&mut builder, SEVEN, &option7);
    append_option(&mut builder, EIGHT, &option8);
    append_option(&mut builder, NINE, &option9);

    let result = ctx.say(builder.to_string()).await?;
    let message = result.message().await?;
    message.react(ctx, ReactionType::Unicode(ONE.to_string())).await?;
    message.react(ctx, ReactionType::Unicode(TWO.to_string())).await?;
    if option3.is_some() {
        message.react(ctx, ReactionType::Unicode(THREE.to_string())).await?;
    }
    if option4.is_some() {
        message.react(ctx, ReactionType::Unicode(FOUR.to_string())).await?;
    }
    if option5.is_some() {
        message.react(ctx, ReactionType::Unicode(FIVE.to_string())).await?;
    }
    if option6.is_some() {
        message.react(ctx, ReactionType::Unicode(SIX.to_string())).await?;
    }
    if option7.is_some() {
        message.react(ctx, ReactionType::Unicode(SEVEN.to_string())).await?;
    }
    if option8.is_some() {
        message.react(ctx, ReactionType::Unicode(EIGHT.to_string())).await?;
    }
    if option9.is_some() {
        message.react(ctx, ReactionType::Unicode(NINE.to_string())).await?;
    }

    Ok(())
}
