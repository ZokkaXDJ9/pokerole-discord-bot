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
        let mut result = std::format!("### {}\n", &rule.name);
        if let Some(flavor) = &rule.flavor {
            result.push_str(&std::format!("*{}*\n", flavor));
        }

        result.push_str(&std::format!("{}\n", &rule.text));

        if let Some(example) = &rule.example {
            result.push_str(&std::format!("**Example**: {}", example));
        }

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Rule not found. Oh no!").await?;
    Ok(())
}
