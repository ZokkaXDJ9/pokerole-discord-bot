use crate::commands::{Context, Error};
use crate::emoji;
use crate::game_data::PokemonApiId;

/// View some fancy server stats.
#[poise::command(slash_command)]
pub async fn server_stats(ctx: Context<'_>) -> Result<(), Error> {
    let defer = ctx.defer();
    let records = sqlx::query!(
        "SELECT species_api_id, COUNT(*) as count FROM character GROUP BY species_api_id ORDER BY species_api_id ASC"
    )
    .fetch_all(&ctx.data().database)
    .await
    .unwrap();

    let mut result = String::new();
    result.push_str("### Played Species Overview\n");
    for record in records {
        let species_api_id = PokemonApiId(record.species_api_id as u16);
        let pokemon = ctx
            .data()
            .game
            .pokemon_by_api_id
            .get(&species_api_id)
            .unwrap();

        result.push_str(&format!(
            "- {}{}: {}\n",
            emoji::get_any_pokemon_emoji_with_space(&ctx.data().database, pokemon).await,
            pokemon.name,
            record.count
        ));
    }

    result.push_str("\n*(Got any other ideas for what should be displayed here? Lemme know and I might add it!)*");

    let _ = defer.await;
    let _ = ctx.reply(result).await;
    Ok(())
}
