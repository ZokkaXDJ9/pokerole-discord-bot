use std::cmp::Ordering;
use std::sync::Arc;
use futures::StreamExt;
use crate::data::Data;
use crate::MovePokemonType;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn autocomplete(partial: &str, commands: &Arc<Vec<String>>) -> Vec<String> {
    if partial.len() < 2 {
        return Vec::default();
    }

    let lower_case = &partial.to_lowercase();

    let mut result: Vec<String> = commands.iter()
        .filter(move |x| x.to_lowercase().contains(lower_case))
        .map(|x| x.clone())
        .collect();

    result.sort_by(|a, b| {
        if a.to_lowercase().starts_with(lower_case) {
            return Ordering::Less;
        }
        if b.to_lowercase().starts_with(lower_case) {
            return Ordering::Greater;
        }

        Ordering::Equal
    });

    return result;
}

async fn autocomplete_move<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> Vec<String> {
    autocomplete(partial, &_ctx.data().move_names)
}

async fn autocomplete_ability<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> Vec<String> {
    autocomplete(partial, &_ctx.data().ability_names)
}

async fn autocomplete_pokemon<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> Vec<String> {
    autocomplete(partial, &_ctx.data().pokemon_names)
}

/// Display a move
#[poise::command(slash_command, rename = "move")]
pub async fn poke_move(
    ctx: Context<'_>,
    #[description = "Which move?"]
    #[rename = "move"]
    #[autocomplete = "autocomplete_move"]
    poke_move_name: String,
) -> Result<(), Error> {
    if let Some(poke_move) = ctx.data().moves.get(&poke_move_name) {
        let mut result : String = std::format!("__**{}**__\n", &poke_move.name);
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
        result.push_str(&poke_move.effect);

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Move not found. Oh no!").await?;
    Ok(())
}

/// Display an Ability
#[poise::command(slash_command)]
pub async fn ability(
    ctx: Context<'_>,
    #[description = "Which ability?"]
    #[rename = "ability"]
    #[autocomplete = "autocomplete_ability"]
    ability_name: String,
) -> Result<(), Error> {
    if let Some(ability) = ctx.data().abilities.get(&ability_name) {
        let mut result : String = std::format!("**{}**: {}\n*{}*", &ability.name, &ability.effect, ability.description);
        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Ability not found. Oh no!").await?;
    Ok(())
}


fn print_stat(result: &mut String, attribute: &str, min: u8, max: u8) {
    result.push_str(&std::format!("**{}**: ", attribute));

    for _ in 0..min {
        result.push_str("⬤");
    }
    for _ in 0..max-min {
        result.push_str("⭘");
    }

    result.push_str(&std::format!(" `{}/{}`\n", min, max));
}

/// Display an Ability
#[poise::command(slash_command)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon_name: String,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&pokemon_name) {
        let mut result = std::format!("{} __{}__\n", pokemon.id, pokemon.name);
        if let Some(type1) = pokemon.type1 {
            result.push_str("**Type**: ");
            result.push_str(std::format!("{:?}", type1).as_str());
            if let Some(type2) = pokemon.type2 {
                result.push_str(std::format!(" / {:?}", type2).as_str())
            }
            result.push_str("\n");
        }

        result.push_str(&std::format!("**Base HP**: {}\n", pokemon.base_hp));

        print_stat(&mut result, "Strength", pokemon.strength, pokemon.max_strength);
        print_stat(&mut result, "Dexterity", pokemon.dexterity, pokemon.max_dexterity);
        print_stat(&mut result, "Vitality", pokemon.vitality, pokemon.max_vitality);
        print_stat(&mut result, "Special", pokemon.special, pokemon.max_special);
        print_stat(&mut result, "Insight", pokemon.insight, pokemon.max_insight);

        if let Some(ability1) = &pokemon.ability1 {
            result.push_str("**Ability**: ");
            result.push_str(&std::format!("{}", ability1));
            if let Some(ability2) = &pokemon.ability2 {
                result.push_str(&std::format!(" / {}", ability2))
            }

            if let Some(hidden) = &pokemon.ability_hidden {
                result.push_str(&std::format!(" ({})", hidden))
            }
            result.push_str("\n");
        }

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Pokemon not found. Oh no!").await?;
    Ok(())
}
