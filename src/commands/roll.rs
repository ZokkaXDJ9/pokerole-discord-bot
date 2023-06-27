use crate::commands::{Context, Error};
use crate::parse_error::ParseError;
use rand::Rng;
use std::str::FromStr;

const CRIT: u8 = 6;
const FAIL_THRESHOLD: u8 = 3;

/// Roll dice using a "1d6+4" style text query.
#[poise::command(slash_command)]
pub async fn r(
    ctx: Context<'_>,
    #[description = "1d6+5 will roll 1d6 and add 5."] query: String,
) -> Result<(), Error> {
    execute_query(&ctx, query).await
}

/// Roll dice by entering die amount, sides and flat addition manually.
#[poise::command(slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "How many dies?"]
    #[min = 1_u8]
    #[max = 100_u8]
    dice: Option<u8>,
    #[description = "How many sides?"]
    #[min = 2_u8]
    #[max = 100_u8]
    sides: Option<u8>,
    #[description = "Add a flat value to the result"]
    #[min = 0_u8]
    #[max = 100_u8]
    flat_addition: Option<u8>,
) -> Result<(), Error> {
    execute_parsed(&ctx, dice, sides, flat_addition).await
}

pub async fn execute_query<'a>(ctx: &Context<'a>, query: String) -> Result<(), Error> {
    let flat_addition: Option<u8>;

    let mut remaining_query = query;
    if remaining_query.contains('+') {
        let split: Vec<&str> = remaining_query.split('+').collect();
        if remaining_query.starts_with('+') {
            if split.len() > 1 {
                return Err(Box::new(ParseError::new("Unable to parse query.")));
            }

            match u8::from_str(split[0]) {
                Ok(value) => flat_addition = Some(value),
                Err(_) => return Err(Box::new(ParseError::new("Unable to parse query."))),
            }

            remaining_query = String::from("");
        } else {
            if split.len() != 2 {
                return Err(Box::new(ParseError::new("Unable to parse query.")));
            }

            match u8::from_str(split[1]) {
                Ok(value) => flat_addition = Some(value),
                Err(_) => return Err(Box::new(ParseError::new("Unable to parse query."))),
            }
            remaining_query = String::from(split[0]);
        }
    } else {
        flat_addition = None;
    }

    let split: Vec<&str> = remaining_query.split('d').collect();
    if split.len() != 2 {
        return Err(Box::new(ParseError::new("Unable to parse query.")));
    }

    let amount = match u8::from_str(split[0]) {
        Ok(value) => Some(value),
        Err(_) => return Err(Box::new(ParseError::new("Unable to parse query."))),
    };

    let sides = match u8::from_str(split[1]) {
        Ok(value) => Some(value),
        Err(_) => return Err(Box::new(ParseError::new("Unable to parse query."))),
    };

    execute_parsed(ctx, amount, sides, flat_addition).await
}

async fn execute_parsed<'a>(
    ctx: &Context<'a>,
    dice: Option<u8>,
    sides: Option<u8>,
    flat_addition: Option<u8>,
) -> Result<(), Error> {
    let result = process_roll(dice, sides, flat_addition);
    ctx.say(result).await?;
    Ok(())
}

fn process_roll(amount: Option<u8>, sides: Option<u8>, flat_addition: Option<u8>) -> String {
    let dice_amount = amount.unwrap_or(1).clamp(0, 100);
    let sides_amount = sides.unwrap_or(6).clamp(0, 100);
    let flat_addition_amount = flat_addition.unwrap_or(0);

    let mut results = Vec::new();
    let mut total: u32 = flat_addition_amount as u32;
    let mut six_count: u32 = 0;
    let mut successes: u32 = 0;
    {
        // TODO: this is ugly :>
        let mut rng = rand::thread_rng();
        for _ in 0..dice_amount {
            let value = rng.gen_range(1..sides_amount + 1);
            total += value as u32;
            if value > 3 {
                successes += 1;
                if value == 6 {
                    six_count += 1;
                }
            }
            results.push(value);
        }
    }

    let result_list = results
        .iter()
        .map(|x| {
            if sides_amount == CRIT {
                if x == &CRIT {
                    return format!("**__{}__**", x);
                } else if x > &FAIL_THRESHOLD {
                    return format!("**{}**", x);
                }
            }

            x.to_string()
        })
        .collect::<Vec<String>>()
        .join(", ");

    let mut text = format!("{}d{}", dice_amount, sides_amount);

    if flat_addition_amount > 0 {
        text.push_str(&format!(
            "+{} — {}+{} = {}",
            flat_addition_amount, result_list, flat_addition_amount, total
        ));
    } else {
        text.push_str(&format!(" — {}", result_list));
        let success_string: &str;
        if successes == 0 {
            success_string = "Successes...";
        } else if successes >= 6 {
            success_string = "Successes!!";
        } else if successes >= 3 {
            success_string = "Successes!";
        } else if successes == 1 {
            success_string = "Success.";
        } else {
            success_string = "Successes.";
        }

        let crit_string = if six_count >= 3 { " **(CRIT)**" } else { "" };

        if sides_amount == CRIT {
            text.push_str(&format!(
                "\n**{}** {}{}",
                successes, success_string, crit_string
            ));
        }
    }

    text
}
