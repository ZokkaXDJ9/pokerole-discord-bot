use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::data::pokemon::{ApiIssueType, LearnablePokemonMoves, PokemonMoveLearnedByRank};
use crate::enums::MysteryDungeonRank;

fn filter_moves<F>(result: &mut String, title: &str, learns: &Vec<PokemonMoveLearnedByRank>, filter: F)
    where F: Fn(&PokemonMoveLearnedByRank) -> bool {
    let moves = learns.iter()
        .filter(|x| filter(x))
        .map(|x| x.name.clone())
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

fn append_all_learnable_moves(learns: &LearnablePokemonMoves, mut result: &mut String) {
    append_moves(&mut result, "\n**TM Moves**\n", learns.by_machine.iter().map(|x| x.clone()).collect());
    append_moves(&mut result, "\n**Egg Moves**\n", learns.by_egg.iter().map(|x| x.clone()).collect());
    append_moves(&mut result, "\n**Tutor**\n", learns.by_tutor.iter().map(|x| x.clone()).collect());
    append_moves(&mut result, "\n**Learned in Game through level up, but not here**\n", learns.by_level_up.iter()
        .filter(|x| learns.by_pokerole_rank.iter().any(|learn| &learn.name == x.clone()))
        .map(|x| x.clone())
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

        let learns = &pokemon.moves.by_pokerole_rank;
        let mut result = std::format!("### {} [#{}]\n", pokemon.name, pokemon.number);

        filter_moves(&mut result, "**Bronze**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Bronze);
        filter_moves(&mut result, "**Silver**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Silver);
        filter_moves(&mut result, "**Gold**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Gold);
        filter_moves(&mut result, "**Platinum**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Platinum);
        filter_moves(&mut result, "**Diamond**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Diamond);

        if show_all_moves.unwrap_or(false) {
            if let Some(issue) = pokemon.api_issue {
                if issue == ApiIssueType::FoundNothing {
                    result.push_str(&std::format!("\nUnable to match any species to this particular pokemon when searching for TM Moves."));
                } else {
                    result.push_str(&std::format!("\n**Struggling to match an exact species to this particular pokemon when searching for TM Moves. Take the values here with a grain of salt!**\n"));
                    append_all_learnable_moves(&pokemon.moves, &mut result);
                }
            } else {
                append_all_learnable_moves(&pokemon.moves, &mut result);
            }
        }

        ctx.say(result).await?;
        return Ok(());
    }

    ctx.say("Pokemon not found. Oh no!").await?;
    Ok(())
}
