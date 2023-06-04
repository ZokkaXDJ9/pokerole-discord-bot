use log::error;
use crate::enums::MoveType;
use crate::commands::{Context, Error};
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
        if poke_move.description.is_empty() {
            error!("Empty description for {}", poke_move.name);
        }

        result.push_str("*");
        result.push_str(&poke_move.description);
        result.push_str("*\n");

        result.push_str("**Type**: ");
        if poke_move.typing == MoveType::Typeless {
            result.push_str("None");
        } else {
            result.push_str(std::format!("{:?}", poke_move.typing).as_str());
        }
        result.push_str(" — **");
        result.push_str(std::format!("{:?}", poke_move.category).as_str());
        result.push_str("**\n");

        result.push_str("**Target**: ");
        result.push_str(std::format!("{}", poke_move.target).as_str());
        result.push_str("\n");

        result.push_str("**Damage Dice**: ");
        if let Some(stat) = poke_move.damage1 {
            result.push_str(std::format!("{:?}", stat).as_str());
            result.push_str(" + ");
        }
        if let Some(stat) = poke_move.happiness_damage {
            result.push_str(std::format!("{:?}", stat).as_str());
            result.push_str(" + ");
        }
        result.push_str(&std::format!("{}\n", poke_move.power));

        result.push_str("**Accuracy Dice**: ");
        if let Some(stat) = poke_move.accuracy1 {
            result.push_str(std::format!("{:?}", stat).as_str());

            if let Some(_) = poke_move.accuracy2 {
                result.push_str(" + Rank");
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
