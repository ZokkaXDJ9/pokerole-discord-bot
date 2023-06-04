use crate::commands::{Context, Error};
use futures::StreamExt;
use crate::{MovePokemonType};
use crate::commands::autocompletion::autocomplete_move;

/// Display a move
#[poise::command(slash_command, rename = "move")]
pub async fn poke_move(
    ctx: Context<'_>,
    #[description = "Which move?"]
    #[rename = "move"]
    #[autocomplete = "autocomplete_move"]
    poke_move_name: String,
) -> Result<(), Error> {
    if let Some(poke_move) = ctx.data().moves.get(&poke_move_name.to_lowercase()) {
        let mut result : String = std::format!("### {}\n", &poke_move.name);
        if let Some(description) = &poke_move.description {
            result.push_str("*");
            result.push_str(description);
            result.push_str("*\n");
        }

        result.push_str("**Type**: ");
        if poke_move.typing == MovePokemonType::Typeless {
            result.push_str("None");
        } else {
            result.push_str(std::format!("{:?}", poke_move.typing).as_str());
        }
        result.push_str(" — **");
        result.push_str(std::format!("{:?}", poke_move.move_type).as_str());
        result.push_str("**\n");

        result.push_str("**Target**: ");
        result.push_str(std::format!("{:?}", poke_move.target).as_str());
        result.push_str("\n");

        result.push_str("**Damage Dice**: ");
        if let Some(stat) = poke_move.base_stat {
            result.push_str(std::format!("{:?}", stat).as_str());
            result.push_str(" + ");
        }
        result.push_str(&std::format!("{}\n", poke_move.base_power));

        result.push_str("**Accuracy Dice**: ");
        if let Some(stat) = poke_move.accuracy_stat {
            result.push_str(std::format!("{:?}", stat).as_str());

            if let Some(secondary) = poke_move.secondary_stat {
                result.push_str(" + Rank");
//                result.push_str(std::format!("{:?}", secondary).as_str());
            }
        }
        result.push_str("\n");

        result.push_str("**Effect**: ");
        result.push_str(&poke_move.effect.replace("Lethal", "Inflicts Wounds"));

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Move not found. Oh no!").await?;
    Ok(())
}










/// Blah blah blah
pub async fn about(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Movepools courtesy by pokeapi (https://github.com/PokeAPI/pokeapi).").await?;
    Ok(())
}
