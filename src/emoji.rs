use crate::enums::{Gender, PokemonGeneration, RegionalVariant};
use crate::game_data::pokemon::Pokemon;
use sqlx::{Pool, Sqlite};

pub const POKE_COIN: &str = "<:poke_coin:1120237132200546304>";

pub const RANK_BRONZE: &str = "<:badge_bronze:1119186018793435177>";
pub const RANK_SILVER: &str = "<:badge_silver:1119185975545954314>";
pub const RANK_GOLD: &str = "<:badge_gold:1119185974149251092>";
pub const RANK_PLATINUM: &str = "<:badge_platinum:1119185972635107378>";
pub const RANK_DIAMOND: &str = "<:badge_diamond:1119185970374389760>";

pub const UNICODE_ONE: &str = "1️⃣";
pub const UNICODE_TWO: &str = "2️⃣";
pub const UNICODE_THREE: &str = "3️⃣";
pub const UNICODE_FOUR: &str = "4️⃣";
pub const UNICODE_FIVE: &str = "5️⃣";
pub const UNICODE_SIX: &str = "6️⃣";
pub const UNICODE_SEVEN: &str = "7️⃣";
pub const UNICODE_EIGHT: &str = "8️⃣";
pub const UNICODE_NINE: &str = "9️⃣";

pub const UNICODE_CROSS_MARK_BUTTON: &str = "❎";
pub const UNICODE_CROSS_MARK: &str = "❌";

pub const TROPHY: &str = "🏆";
pub const BACKPACK: &str = "🎒";
pub const FENCING: &str = "🤺";
pub const TICKET: &str = "🎫";
pub const CROSSED_SWORDS: &str = "⚔️";

pub async fn get_character_emoji(
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
        .replace(' ', "_")
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
