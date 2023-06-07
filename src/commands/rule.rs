use serenity::utils::MessageBuilder;
use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_rule;

/// Display rule
#[poise::command(slash_command)]
pub async fn rule(
    ctx: Context<'_>,
    #[description = "Which rule?"]
    #[rename = "name"]
    #[autocomplete = "autocomplete_rule"]
    name: String,
) -> Result<(), Error> {
    if let Some(rule) = ctx.data().rules.get(&name.to_lowercase()) {
        let mut builder = MessageBuilder::default();
        builder.push(std::format!("### {}\n", &rule.name));
        if let Some(flavor) = &rule.flavor {
            builder.push_italic_line(flavor);
        }

        builder.push(&rule.text);

        if let Some(example) = &rule.example {
            builder.quote_rest();
            builder.push(std::format!("**Example**: {}", example));
        }

        ctx.say(builder.build()).await?;
        return Ok(());
    }

    ctx.say("Rule not found. Oh no!").await?;
    Ok(())
}
