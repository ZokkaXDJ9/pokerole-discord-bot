use sqlx::{Pool, Sqlite};

use crate::data::Data;
use crate::enums::{Gender, PokemonType, RegionalVariant};
use crate::game_data::pokemon::Pokemon;
use crate::game_data::PokemonApiId;

pub const POKE_COIN: &str = "<:pokecoin:1272536303430270986>";
pub const BATTLE_POINT: &str = "<:battlepoint:1272533678714519612>";

pub const RANK_BRONZE: &str = "<:badgebronze:1272532685197152349>";
pub const RANK_SILVER: &str = "<:badgesilver:1272533590697185391>";
pub const RANK_GOLD: &str = "<:badgegold:1272532681992962068>";
pub const RANK_PLATINUM: &str = "<:badgeplatinum:1272533593750507570>";
pub const RANK_DIAMOND: &str = "<:badgediamond:1272532683431481445>";
pub const RANK_MASTER: &str = "<:badgemaster:1299338926431014913>";


pub const UNICODE_CROSS_MARK_BUTTON: &str = "❎";
pub const UNICODE_CROSS_MARK: &str = "❌";
pub const UNICODE_CHECK_MARK: &str = "✔️";

pub const TROPHY: &str = "🏆";
pub const BACKPACK: &str = "🎒";
pub const FENCING: &str = "🤺";
pub const TICKET: &str = "🎫";
pub const CROSSED_SWORDS: &str = "⚔️";
pub const PARTY_POPPER: &str = "🎉";
pub const PARTYING_FACE: &str = "🥳";

pub const DOT_EMPTY: char = '⭘';
pub const DOT_FILLED: char = '⬤';
pub const DOT_OVERCHARGED: char = '⧳';

pub async fn get_character_emoji(data: &Data, character_id: i64) -> Option<String> {
    let result = sqlx::query!(
        "SELECT guild_id, species_api_id, is_shiny, phenotype FROM character WHERE id = ?",
        character_id
    )
    .fetch_one(&data.database)
    .await;

    if let Ok(record) = result {
        let gender = Gender::from_phenotype(record.phenotype);
        let api_id = PokemonApiId(record.species_api_id as u16);
        let pokemon = data
            .game
            .pokemon_by_api_id
            .get(&api_id)
            .expect("DB species ID should always be set!");

        get_pokemon_emoji(
            &data.database,
            record.guild_id,
            pokemon,
            &gender,
            record.is_shiny,
        )
        .await
    } else {
        None
    }
}

pub async fn get_pokemon_emoji(
    database: &Pool<Sqlite>,
    guild_id: i64,
    pokemon: &Pokemon,
    gender: &Gender,
    is_shiny: bool,
) -> Option<String> {
    let api_id = pokemon.poke_api_id.0 as i64;
    let is_female = pokemon.species_data.has_gender_differences && gender == &Gender::Female;
    let is_animated = pokemon.has_animated_sprite();

    let result = sqlx::query!("SELECT discord_string FROM emoji WHERE species_api_id = ? AND guild_id = ? AND is_female = ? AND is_shiny = ? AND is_animated = ?", api_id, guild_id, is_female, is_shiny, is_animated)
        .fetch_one(database)
        .await;

    if let Ok(result) = result {
        return Some(result.discord_string);
    }

    // Try again, without guild_id. Technically we could just leech emojis off of extra servers
    let result = sqlx::query!("SELECT discord_string FROM emoji WHERE species_api_id = ? AND is_female = ? AND is_shiny = ? AND is_animated = ?", api_id, is_female, is_shiny, is_animated)
        .fetch_one(database)
        .await;

    if let Ok(result) = result {
        return Some(result.discord_string);
    }

    // Any will do! Please!~
    get_any_pokemon_emoji(database, pokemon).await
}

pub async fn get_any_pokemon_emoji(database: &Pool<Sqlite>, pokemon: &Pokemon) -> Option<String> {
    let api_id = pokemon.poke_api_id.0 as i64;

    if pokemon.has_animated_sprite() {
        let result = sqlx::query!(
            "SELECT discord_string FROM emoji WHERE species_api_id = ? AND is_animated = true",
            api_id
        )
        .fetch_one(database)
        .await;

        if let Ok(result) = result {
            return Some(result.discord_string);
        }
    }

    let result = sqlx::query!(
        "SELECT discord_string FROM emoji WHERE species_api_id = ?",
        api_id
    )
    .fetch_one(database)
    .await;

    if let Ok(result) = result {
        return Some(result.discord_string);
    }

    None
}

pub async fn get_any_pokemon_emoji_with_space(
    database: &Pool<Sqlite>,
    pokemon: &Pokemon,
) -> String {
    if let Some(emoji) = get_any_pokemon_emoji(database, pokemon).await {
        format!("{} ", emoji)
    } else {
        String::new()
    }
}

pub fn pokemon_to_emoji_name(
    pokemon: &Pokemon,
    is_female: bool,
    is_shiny: bool,
    is_animated: bool,
) -> String {
    let shiny = if is_shiny { "shiny_" } else { "" };
    let female = if is_female { "_female" } else { "" };
    let mut name = pokemon
        .name
        .to_lowercase()
        .replace([' ', '-'], "_")
        .replace(['(', ')'], "");

    let regional_prefix = if let Some(regional_variant) = pokemon.regional_variant {
        name = name
            .replace("paldean_form", "")
            .replace("hisuian_form", "")
            .replace("galarian_form", "")
            .replace("alolan_form", "");

        match regional_variant {
            RegionalVariant::Alola => "alolan_",
            RegionalVariant::Galar => "galarian",
            RegionalVariant::Hisui => "hisuian_",
            RegionalVariant::Paldea => "paldean_",
        }
    } else {
        ""
    };

    let animated = if is_animated { "_animated" } else { "" };

    format!(
        "{}{}{}{}{}",
        shiny,
        regional_prefix,
        name.trim_matches('_'),
        female,
        animated
    )
}

pub fn type_to_emoji(pokemon_type: &PokemonType) -> &str {
        println!("Emoji.rs is being used!");    
        match pokemon_type {
        PokemonType::Normal => "<:typenormal:1272535965893791824>",
        PokemonType::Fighting => "<:typefighting:1272535949569429586>",
        PokemonType::Flying => "<:typeflying:1272536305380753440>",
        PokemonType::Poison => "<:typepoison:1272536309147238440>",
        PokemonType::Ground => "<:typeground:1272535961682579496>",
        PokemonType::Rock => "<:typerock:1272535973481283596>",
        PokemonType::Bug => "<:typebug:1272535941420027924>",
        PokemonType::Ghost => "<:typeghost:1272535956733300879>",
        PokemonType::Steel => "<:typesteel:1272536310984212491>",
        PokemonType::Fire => "<:typefire:1272535951129968780>",
        PokemonType::Water => "<:typewater:1272535976794652673>",
        PokemonType::Grass => "<:typegrass:1272535959677960222>",
        PokemonType::Electric => "<:typeelectric:1272535946788606123>",
        PokemonType::Psychic => "<:typepsychic:1272535970897592330>",
        PokemonType::Ice => "<:typeice:1272536307276709898>",
        PokemonType::Dragon => "<:typedragon:1272535944804962335>",
        PokemonType::Dark => "<:typedark:1272535943060000800>",
        PokemonType::Fairy => "<:typefairy:1272535948357537894>",
        PokemonType::Virus => "<:typevirus:1305196521058205766>",
        PokemonType::Shadow => "",
    }
}
