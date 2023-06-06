use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_item;

/// Display item description
#[poise::command(slash_command)]
pub async fn item(
    ctx: Context<'_>,
    #[description = "Which item?"]
    #[rename = "name"]
    #[autocomplete = "autocomplete_item"]
    name: String,
) -> Result<(), Error> {
    if let Some(item) = ctx.data().items.get(&name.to_lowercase()) {
        let mut result: String = std::format!("### {}\n", &item.name);

        if let Some(price) = &item.price {
            result.push_str(&format!("**Price**: {}\n", price));
        }

        result.push_str(&item.description);

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Item not found. Oh no!").await?;
    Ok(())
}
