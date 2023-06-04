use crate::commands::{Context, Error};
use crate::pokerole_discord_py_csv_parser::{PokeLearn, PokeLearnEntry, PokeRoleRank};
use crate::pokemon_api_parser::ApiPokemonLearnableMoves;
use crate::commands::autocompletion::autocomplete_pokemon;

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

fn append_all_learnable_moves(learns: &PokeLearn, mut result: &mut String, all_learnable_moves: &ApiPokemonLearnableMoves) {
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
        let mut result = std::format!("### {} [{}]\n", pokemon.name, pokemon.number);

        filter_moves(&mut result, "**Bronze**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Starter || x.rank == PokeRoleRank::Beginner);
        filter_moves(&mut result, "**Silver**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Amateur);
        filter_moves(&mut result, "**Gold**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Ace);
        filter_moves(&mut result, "**Platinum**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Pro);
        filter_moves(&mut result, "**Diamond**\n", &learns, |x:&PokeLearnEntry| x.rank == PokeRoleRank::Master || x.rank == PokeRoleRank::Champion);

        if show_all_moves.unwrap_or(false) {
            if let Some(api_data) = ctx.data().pokemon_api_data.get(&pokemon.name) {
                append_all_learnable_moves(learns, &mut result, &api_data.learnable_moves);
            } else {
                let mut options: Vec<String> = ctx.data().pokemon_api_data.keys()
                    .filter(|x| x.contains(&pokemon.name))
                    .map(|x| x.clone())
                    .collect();

                if options.is_empty() {
                    result.push_str("\n**(Unable to find learnable game moves. Maybe something's not linked up properly, lemme know if this happens.)**\n");
                } else {
                    let option = options.pop().unwrap();
                    let api_data = ctx.data().pokemon_api_data.get(&option).unwrap();
                    result.push_str(&std::format!("\nStruggling to find TM Moves. Quickfix found the following:\n- {} (used here)\n", api_data.pokemon_name));
                    for x in options {
                        result.push_str(&std::format!("- {}\n", x));
                    }

                    append_all_learnable_moves(learns, &mut result, &api_data.learnable_moves);
                }
            }
        }

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Pokemon not found. Oh no!").await?;
    Ok(())
}
