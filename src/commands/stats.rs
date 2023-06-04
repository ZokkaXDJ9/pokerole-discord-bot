use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;

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
        let mut result = std::format!("### {} [{}]\n", pokemon.name, pokemon.id);
        let api_data = ctx.data().pokemon_api_data.get(&pokemon.name).expect("All pokerole mons should have api data!");
        result.push_str(&std::format!("{}m / {:.1}ft   |   {}kg / {:.1}lbs\n",
                                      api_data.height_in_meters,
                                      api_data.height_in_meters * 3.28084,
                                      api_data.weight_in_kg,
                                      api_data.weight_in_kg * 2.20462));
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
