use crate::commands::{Context, Error};
use rand::Rng;

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