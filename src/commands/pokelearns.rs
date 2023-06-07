use serenity::builder::CreateButton;
use serenity::http::CacheHttp;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;
use crate::commands::{Context, Error};
use crate::commands::autocompletion::autocomplete_pokemon;
use crate::data::pokemon::{ApiIssueType, LearnablePokemonMoves, Pokemon, PokemonMoveLearnedByRank};
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
        .filter(|x| learns.by_pokerole_rank.iter().all(|learn| !(learn.name.to_lowercase() == x.to_lowercase())))
        .map(|x| x.clone())
        .collect());
}

/// Display Pokemon moves
#[poise::command(slash_command, prefix_command)]
pub async fn pokelearns(
    ctx: Context<'_>,
    #[description = "Which pokemon?"]
    #[rename = "pokemon"]
    #[autocomplete = "autocomplete_pokemon"]
    pokemon_name: String,
) -> Result<(), Error> {
    if let Some(pokemon) = ctx.data().pokemon.get(&pokemon_name.to_lowercase()) {
        let learns = &pokemon.moves.by_pokerole_rank;
        let mut result = std::format!("### {} [#{}]\n", pokemon.name, pokemon.number);

        filter_moves(&mut result, "**Bronze**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Bronze);
        filter_moves(&mut result, "**Silver**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Silver);
        filter_moves(&mut result, "**Gold**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Gold);
        filter_moves(&mut result, "**Platinum**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Platinum);
        filter_moves(&mut result, "**Diamond**\n", &learns, |x:&PokemonMoveLearnedByRank| x.rank == MysteryDungeonRank::Diamond);

        let reply = ctx.send(|b| {
            b.content(&result)
                .components(|b| {
                    b.create_action_row(|b| {
                        b.add_button(create_button(false))
                    })
                })
        }).await?;

        let message = reply.message().await?;
        let interaction = message
            .await_component_interaction(ctx)
            .timeout(std::time::Duration::from_secs(3 * 60))
            .await;

        match &interaction {
            Some(m) => {
                m.create_interaction_response(ctx, |response| {
                    response.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
                        d.components(|b| {
                             b.create_action_row( |row| {
                                 row.add_button(create_button(true))
                             })
                        })
                    })
                }).await?;

                ctx.send(|b| b.content(get_tm_moves(&pokemon))).await?;
            },
            None => {
                reply.edit(ctx, |b| {
                    b.components(|components| {
                        components.create_action_row( |row| {
                             row.add_button(create_button(true))
                        })
                    })
                }).await?;
            }
        };

        return Ok(());
    }

    ctx.say("Pokemon not found. Oh no!").await?;
    Ok(())
}

fn create_button(disabled: bool) -> CreateButton {
    let mut button = CreateButton::default();
    button.label("Show TM Moves");
    button.custom_id("Show TM Moves");
    button.style(ButtonStyle::Primary);
    button.disabled(disabled);
    button
}

fn get_tm_moves(pokemon: &Pokemon) -> String {
    let mut result = std::format!("### {} [#{}]\n", pokemon.name, pokemon.number);
    if let Some(issue) = pokemon.api_issue {
        if issue == ApiIssueType::FoundNothing {
            result.push_str(&std::format!("\nUnable to match any species to this particular pokemon when searching for TM Moves."));
        } else if issue == ApiIssueType::IsLegendary {
            result.push_str(&std::format!("\nToo lazy to be bothered to get this to work for legendary pokemon, sorry!"));
        } else {
            result.push_str(&std::format!("\n**Struggling to match an exact species to this particular pokemon when searching for TM Moves. Take the values here with a grain of salt!**\n"));
            append_all_learnable_moves(&pokemon.moves, &mut result);
        }
    } else {
        append_all_learnable_moves(&pokemon.moves, &mut result);
    }

    return result;
}

