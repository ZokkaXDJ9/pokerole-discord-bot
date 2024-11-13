use crate::commands::{Context, Error};
use crate::errors::ParseError;
use crate::helpers;
use poise::CreateReply;
use rand::Rng;
use serenity::all::CreateActionRow;
use std::str::FromStr;

const CRIT: u8 = 6;
const FAIL_THRESHOLD: u8 = 3;

/// Roll dice using a "1d6+4" style text query.
#[poise::command(slash_command)]
pub async fn r(
    ctx: Context<'_>,
    #[description = "1d6+5 will roll 1d6 and add 5."] query: String,
) -> Result<(), Error> {
    execute_query(&ctx, &query).await
}

/// Roll dice by entering die amount, sides and flat addition manually.
#[poise::command(slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "How many dice?"]
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
    execute_parsed(&ctx, ParsedRollQuery::new(dice, sides, flat_addition)).await
}

pub struct ParsedRollQuery {
    amount: u8,
    sides: u8,
    flat_addition: u8,
}

impl ParsedRollQuery {
    pub fn new(dice: Option<u8>, sides: Option<u8>, flat_addition: Option<u8>) -> Self {
        ParsedRollQuery {
            amount: dice.unwrap_or(1).clamp(1, 100),
            sides: sides.unwrap_or(6).clamp(2, 100),
            flat_addition: flat_addition.unwrap_or(0),
        }
    }

    fn as_button_callback_query_string(&self) -> String {
        format!(
            "roll-dice_{}d{}+{}",
            self.amount, self.sides, self.flat_addition
        )
    }

    pub fn execute(&mut self, super_luck: bool) -> String {
        let mut rng = rand::thread_rng();
        let mut results = Vec::new();
        let mut successes: u32 = 0;
        let mut six_count: u32 = 0;

        let mut dice_to_roll = self.amount;
        let mut applied_curses: Vec<String> = Vec::new(); // Track applied curses

        // Define possible curses, but skip if user has Super Luck
        if !super_luck {
            let mut available_curses: Vec<fn(
                &mut Self,
                &mut Vec<String>,
                &mut Vec<u8>,
                &mut u32,
                &mut rand::rngs::ThreadRng,
            )> = vec![
                Self::apply_reduced_dice_count,
                Self::apply_increased_success_threshold,
                Self::apply_skewed_rolls_to_low,
                Self::apply_single_high_roll_limit,
                Self::apply_ghost_subtraction,
            ];

            // Randomly apply curses
            let curse_count = rng.gen_range(1..=available_curses.len());
            for _ in 0..curse_count {
                if let Some(curse) = available_curses.pop() {
                    curse(
                        self,
                        &mut applied_curses,
                        &mut results,
                        &mut successes,
                        &mut rng,
                    );
                }
            }
        }

        // Roll the dice if not modified by a curse
        if results.is_empty() {
            for _ in 0..dice_to_roll {
                let value = rng.gen_range(1..=self.sides);
                if value == CRIT {
                    six_count += 1;
                }
                results.push(value);
            }
        }

        // Count successes after curses are applied
        let success_threshold = if applied_curses.contains(&"Increased Success Threshold".to_string())
        {
            5 // Success requires 5 or 6
        } else {
            4 // Default success threshold
        };
        successes = results.iter().filter(|&&x| x >= success_threshold).count() as u32;

        // If user has Super Luck and successes == 0, set successes to 1
        if super_luck && successes == 0 {
            successes = 1;

            // Modify one of the dice to reflect the success
            for x in results.iter_mut() {
                if *x < success_threshold {
                    *x = success_threshold;
                    break;
                }
            }
        }

        // Format roll results
        let result_list = results
            .iter()
            .map(|x| {
                if *x == CRIT {
                    format!("**__{}__**", x) // Highlight critical successes
                } else if *x >= success_threshold {
                    format!("**{}**", x) // Highlight regular successes
                } else {
                    x.to_string() // Plain for non-successes
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        // Prepare output
        let mut text = format!("{}d{}", self.amount, self.sides);
        text.push_str(&format!(" — {}", result_list));

        if self.sides == CRIT {
            let crit_string = if six_count >= 3 { " **(CRIT)**" } else { "" };

            text.push_str(&format!(
                "\n**{}** Success{}{}",
                successes,
                if successes == 1 { "." } else { "es." },
                crit_string
            ));
        } else {
            text.push_str(&format!(
                "\n**{} Success{}**",
                successes,
                if successes == 1 { "." } else { "es." }
            ));
        }

        // Do not display curses if user has Super Luck
        if !applied_curses.is_empty() && !super_luck {
            text.push_str("\n**Applied Curses:** ");
            text.push_str(&applied_curses.join(", "));
        }

        text
    }

    fn apply_reduced_dice_count(
        _self: &mut Self,
        applied_curses: &mut Vec<String>,
        _results: &mut Vec<u8>,
        _successes: &mut u32,
        _rng: &mut rand::rngs::ThreadRng,
    ) {
        if _self.amount > 1 {
            _self.amount -= 1;
            applied_curses.push("Reduced Dice Count".to_string());
        }
    }

    fn apply_increased_success_threshold(
        _self: &mut Self,
        applied_curses: &mut Vec<String>,
        _results: &mut Vec<u8>,
        _successes: &mut u32,
        _rng: &mut rand::rngs::ThreadRng,
    ) {
        applied_curses.push("Increased Success Threshold".to_string());
    }

    fn apply_skewed_rolls_to_low(
        _self: &mut Self,
        applied_curses: &mut Vec<String>,
        results: &mut Vec<u8>,
        _successes: &mut u32,
        rng: &mut rand::rngs::ThreadRng,
    ) {
        results.clear();
        for _ in 0.._self.amount {
            results.push(rng.gen_range(1..=3)); // Force rolls to 1-3
        }
        applied_curses.push("Skewed Rolls to Low".to_string());
    }

    fn apply_single_high_roll_limit(
        _self: &mut Self,
        applied_curses: &mut Vec<String>,
        results: &mut Vec<u8>,
        _successes: &mut u32,
        _rng: &mut rand::rngs::ThreadRng,
    ) {
        if results.iter().filter(|&&x| x == 6).count() > 1 {
            let mut six_found = false;
            *results = results
                .iter()
                .map(|&x| {
                    if x == 6 {
                        if six_found {
                            1 // Replace extra 6s with 1
                        } else {
                            six_found = true;
                            6
                        }
                    } else {
                        x
                    }
                })
                .collect();
            applied_curses.push("Single High Roll Limit".to_string());
        }
    }

    fn apply_ghost_subtraction(
        _self: &mut Self,
        applied_curses: &mut Vec<String>,
        _results: &mut Vec<u8>,
        successes: &mut u32,
        rng: &mut rand::rngs::ThreadRng,
    ) {
        if *successes > 0 {
            let penalty = rng.gen_range(1..=*successes); // Subtract up to the total successes
            *successes = successes.saturating_sub(penalty);
            applied_curses.push(format!("Ghost Subtraction (-{} Successes)", penalty));
        }
    }
}

pub fn parse_query(query: &str) -> Result<ParsedRollQuery, Error> {
    let flat_addition: Option<u8>;

    let mut remaining_query = query.to_string();
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
        let amount = match u8::from_str(&remaining_query) {
            Ok(value) => Some(value),
            Err(_) => return Err(Box::new(ParseError::new("Unable to parse query."))),
        };

        return Ok(ParsedRollQuery::new(amount, Some(6), flat_addition));
    }

    let amount = match u8::from_str(split[0]) {
        Ok(value) => Some(value),
        Err(_) => return Err(Box::new(ParseError::new("Unable to parse query."))),
    };

    let sides = match u8::from_str(split[1]) {
        Ok(value) => Some(value),
        Err(_) => return Err(Box::new(ParseError::new("Unable to parse query."))),
    };

    Ok(ParsedRollQuery::new(amount, sides, flat_addition))
}

pub async fn execute_query<'a>(ctx: &Context<'a>, query: &str) -> Result<(), Error> {
    let parsed_query = match parse_query(query) {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    execute_parsed(ctx, parsed_query).await
}

async fn execute_parsed<'a>(ctx: &Context<'a>, mut query: ParsedRollQuery) -> Result<(), Error> {
    ctx.defer().await?;

    // Check if the user has Super Luck
    let author_id = ctx.author().id;
    let super_luck = author_id == 307627785818603523;

    let result = query.execute(super_luck);
    let query_string = query.as_button_callback_query_string();
    ctx.send(
        CreateReply::default()
            .content(result)
            .components(vec![CreateActionRow::Buttons(vec![
                helpers::create_button("Roll again!", query_string.as_str(), false),
            ])]),
    )
    .await?;
    Ok(())
}
