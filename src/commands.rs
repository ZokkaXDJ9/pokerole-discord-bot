use std::cmp::Ordering;
use std::sync::Arc;
use futures::StreamExt;
use rand::Rng;
use crate::data::Data;
use crate::{MovePokemonType, PokeLearn, PokeLearnEntry, PokeRoleRank};
use crate::pokemon_api_parser::PokemonLearnableMoves;

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

async fn autocomplete_item<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> Vec<String> { autocomplete(partial, &_ctx.data().item_names) }

async fn autocomplete_weather<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> Vec<String> { autocomplete(partial, &_ctx.data().weather_names) }

async fn autocomplete_status_effect<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> Vec<String> { autocomplete(partial, &_ctx.data().status_effects_names) }

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
    if let Some(ability) = ctx.data().abilities.get(&ability_name.to_lowercase()) {
        let mut result : String = std::format!("**{}**: {}\n*{}*", &ability.name, &ability.effect, ability.description);
        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Ability not found. Oh no!").await?;
    Ok(())
}

/// Display the Weather
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Which weather?"]
    #[rename = "name"]
    #[autocomplete = "autocomplete_weather"]
    weather_name: String,
) -> Result<(), Error> {
    if let Some(weather) = ctx.data().weather.get(&weather_name.to_lowercase()) {
        let mut result : String = std::format!("**{}**:\n*{}*\n{}", &weather.name, &weather.description, &weather.effect);
        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Weather not found. Oh no!").await?;
    Ok(())
}

/// Display status effects
#[poise::command(slash_command)]
pub async fn status(
    ctx: Context<'_>,
    #[description = "Which status effect?"]
    #[rename = "name"]
    #[autocomplete = "autocomplete_status_effect"]
    status_name: String,
) -> Result<(), Error> {
    if let Some(status_effect) = ctx.data().status_effects.get(&status_name.to_lowercase()) {
        let mut result : String = std::format!("**__{}__**\n*{}*\n- {}\n- {}\n- {}",
                                               &status_effect.name, &status_effect.description, &status_effect.resist, &status_effect.effect, &status_effect.duration);
        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Weather not found. Oh no!").await?;
    Ok(())
}

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
        let mut result: String = std::format!("**__{}__**\n", &item.name);

        if let Some(price) = &item.suggested_price {
            if (price != "Not for Sale") {
                result.push_str(&format!("**Price**: {}\n", price));
            }
        }

        if let Some(price) = &item.pmd_price {
            result.push_str(&format!("**Price in PMD**: {}\n", price));
        }

        result.push_str(&item.description);

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Item not found. Oh no!").await?;
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

/// Display Pokemon stats
#[poise::command(slash_command)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon_name: String,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&pokemon_name.to_lowercase()) {
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

fn filter_moves<F>(result: &mut String, title: &str, learns: &PokeLearn, filter: F)
    where F: Fn(&PokeLearnEntry) -> bool {
    let moves = learns.moves.iter()
        .filter(|x| filter(x))
        .map(|x| x.poke_move.clone())
        .collect::<Vec<String>>();

    append_moves(result, title, moves);
}

fn append_moves(result: &mut String, title: &str, moves: Vec<String>) {
    let text = moves.join("  |  ");

    if text.is_empty() {
        return;
    }

    result.push_str(title);
    result.push_str(&text);
    result.push('\n');
}

fn append_all_learnable_moves(learns: &PokeLearn, mut result: &mut String, all_learnable_moves: &PokemonLearnableMoves) {
    append_moves(&mut result, "\n**TM Moves**\n", all_learnable_moves.machine.iter().map(|x| x.move_name.clone()).collect());
    append_moves(&mut result, "\n**Egg Moves**\n", all_learnable_moves.egg.iter().map(|x| x.move_name.clone()).collect());
    append_moves(&mut result, "\n**Tutor**\n", all_learnable_moves.tutor.iter().map(|x| x.move_name.clone()).collect());
    append_moves(&mut result, "\n**Learned in Game through level up, but not here**\n", all_learnable_moves.egg.iter()
        .filter(|x| learns.moves.iter().any(|learn| learn.poke_move == x.move_name))
        .map(|x| x.move_name.clone())
        .collect());
}

/// Display Pokemon moves
#[poise::command(slash_command)]
pub async fn pokelearns(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon_name: String,
    //#[description = "Includes TM, Tutor and Egg moves."]
    //#[rename = "showAll"]
    show_all_moves: Option<bool>,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&pokemon_name.to_lowercase()) {
        let lowercase = pokemon_name.to_lowercase();

        let learns = ctx.data().pokemon_learns.iter().find(|x| x.pokemon_name.to_lowercase().contains(&lowercase)).unwrap();
        let mut result = std::format!("{} __{}__\n", pokemon.id, pokemon.name);

        filter_moves(&mut result, "**Bronze**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Starter || x.rank == PokeRoleRank::Beginner);
        filter_moves(&mut result, "**Silver**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Amateur);
        filter_moves(&mut result, "**Gold**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Ace);
        filter_moves(&mut result, "**Platinum**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Pro);
        filter_moves(&mut result, "**Diamond**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Master || x.rank == PokeRoleRank::Champion);

        if show_all_moves.unwrap_or(false) {
            if let Some(all_learnable_moves) = ctx.data().all_learnable_moves.get(&pokemon.name) {
                append_all_learnable_moves(learns, &mut result, all_learnable_moves);
            } else {
                let mut options: Vec<String> = ctx.data().all_learnable_moves.keys()
                    .filter(|x| x.contains(&pokemon.name))
                    .map(|x| x.clone())
                    .collect();

                if options.is_empty() {
                    result.push_str("\n**(Unable to find learnable game moves. Maybe something's not linked up properly, lemme know if this happens.)**\n");
                } else {
                    let option = options.pop().unwrap();
                    let all_learnable_moves = ctx.data().all_learnable_moves.get(&option).unwrap();
                    result.push_str(&std::format!("\nStruggling to find TM Moves. Quickfix found the following:\n- {} (used here)\n", all_learnable_moves.pokemon_name));
                    for x in options {
                        result.push_str(&std::format!("- {}\n", x));
                    }

                    append_all_learnable_moves(learns, &mut result, all_learnable_moves);
                }
            }
        }

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Pokemon not found. Oh no!").await?;
    Ok(())
}

/// Roll them dice!
#[poise::command(slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "How many dies?"]
    dice: Option<u8>,
    #[description = "How many sides?"]
    sides: Option<u8>,
    #[description = "Add a flat value to the result"]
    flat_addition: Option<u8>,
) -> Result<(), Error> {

    let dice_amount = dice.unwrap_or(1).clamp(0, 100);
    let sides_amount = sides.unwrap_or(6).clamp(0, 100);
    let flat_addition_amount = flat_addition.unwrap_or(0);


    let mut results = Vec::new();
    let mut total: u32 = flat_addition_amount as u32;
    let mut six_count: u32 = 0;
    let mut successes: u32 = 0;
    { // TODO: this is ugly :>
        let mut rng = rand::thread_rng();
        for _ in 0..dice_amount {
            let value = rng.gen_range(1..sides_amount + 1);
            total += value as u32;
            if (value > 3) {
                successes += 1;
                if (value == 6) {
                    six_count += 1;
                }
            }
            results.push(value);
        }
    }

    let six:u8 = 6;
    let three:u8 = 3;
    let result_list = results.iter()
        .map(|x| {
            if sides_amount == six {
                if (x == &six) {
                    return format!("**__{}__**", x);
                } else if (x > &three) {
                    return format!("**{}**", x);
                }
            }

            return x.to_string();
        })
        .collect::<Vec<String>>()
        .join(", ");

    let mut text = format!("{}d{}", dice_amount, sides_amount);

    if (flat_addition_amount > 0) {
        text.push_str(&format!("+{} — {}+{} = {}", flat_addition_amount, result_list, flat_addition_amount, total));
    } else {
        text.push_str(&format!(" — {}", result_list));
        let success_string:&str;
        if (successes == 0) {
            success_string = "Successes...";
        } else if (successes >= 6) {
            success_string = "Successes!!";
        } else if (successes >= 3) {
            success_string = "Successes!";
        } else if (successes == 1) {
            success_string = "Success.";
        } else {
            success_string = "Successes.";
        }

        let crit_string:&str;
        if six_count >= 3 {
            crit_string = " **(CRIT)**"
        } else {
            crit_string = ""
        }

        if sides_amount == six {
            text.push_str(&format!("\n**{}** {}{}", successes, success_string, crit_string));
        }
    }

    ctx.say(text).await?;
    Ok(())
}


/// Blah blah blah
pub async fn about(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Movepools courtesy by pokeapi (https://github.com/PokeAPI/pokeapi).").await?;
    Ok(())
}
