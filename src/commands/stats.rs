use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::data::pokemon::Stat;

fn print_stat(result: &mut String, attribute: &str, stat: &Stat) {
    result.push_str(&std::format!("**{}**: ", attribute));

    for _ in 0..stat.min {
        result.push_str("⬤");
    }
    for _ in 0..stat.max-stat.min {
        result.push_str("⭘");
    }

    result.push_str(&std::format!(" `{}/{}`\n", stat.min, stat.max));
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
        let mut result = std::format!("### {} [#{}]\n", pokemon.name, pokemon.number);
        result.push_str(&std::format!("{}m / {}ft   |   {}kg / {}lbs\n",
                                      pokemon.height.meters,
                                      pokemon.height.feet,
                                      pokemon.weight.kilograms,
                                      pokemon.weight.pounds));
        result.push_str("**Type**: ");
        result.push_str(std::format!("{:?}", pokemon.type1).as_str());
        if let Some(type2) = pokemon.type2 {
            result.push_str(std::format!(" / {:?}", type2).as_str())
        }
        result.push_str("\n");

        result.push_str(&std::format!("**Base HP**: {}\n", pokemon.base_hp));

        print_stat(&mut result, "Strength", &pokemon.strength);
        print_stat(&mut result, "Dexterity", &pokemon.dexterity);
        print_stat(&mut result, "Vitality", &pokemon.vitality);
        print_stat(&mut result, "Special", &pokemon.special);
        print_stat(&mut result, "Insight", &pokemon.insight);

        result.push_str("**Ability**: ");
        result.push_str(&std::format!("{}", pokemon.ability1));
        if let Some(ability2) = &pokemon.ability2 {
            result.push_str(&std::format!(" / {}", ability2))
        }

        if let Some(hidden) = &pokemon.hidden_ability {
            result.push_str(&std::format!(" ({})", hidden))
        }

        if let Some(event) = &pokemon.event_abilities {
            result.push_str(&std::format!(" ({})", event))
        }
        result.push_str("\n");

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Pokemon not found. Oh no!").await?;
    Ok(())
}
