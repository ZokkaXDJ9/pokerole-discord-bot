use crate::game_data::parser::custom_data::custom_ability::CustomAbility;
use crate::game_data::parser::custom_data::custom_item::CustomItem;
use crate::game_data::parser::custom_data::custom_move::CustomMove;
use crate::game_data::parser::custom_data::custom_zmove::CustomZMove;
use crate::game_data::parser::custom_data::custom_pokemon::CustomPokemon;
use crate::game_data::parser::custom_data::custom_potion::CustomPotion;
use crate::game_data::parser::custom_data::custom_status_effect::CustomStatusEffect;
use crate::game_data::parser::custom_data::custom_weather::CustomWeather;
use crate::game_data::parser::helpers;
use crate::game_data::rule::Rule;

pub struct CustomDataParseResult {
    pub abilities: Vec<CustomAbility>,
    pub pokemon: Vec<CustomPokemon>,
    pub moves: Vec<CustomMove>,
    pub z_moves: Vec<CustomZMove>, // Add this field
    pub items: Vec<CustomItem>,
    pub status_effects: Vec<CustomStatusEffect>,
    pub potions: Vec<CustomPotion>,
    pub weather: Vec<CustomWeather>,
    pub rules: Vec<Rule>,
}

pub fn parse(custom_data_path: &str) -> CustomDataParseResult {
    let base_path = std::path::Path::new(custom_data_path);
    CustomDataParseResult {
        abilities: helpers::parse_directory(custom_data_path.to_owned() + "Abilities"),
        pokemon: helpers::parse_directory(custom_data_path.to_owned() + "Pokedex"),
        moves: helpers::parse_directory(custom_data_path.to_owned() + "Moves"),
        z_moves: helpers::parse_directory(base_path.join("ZMoves")),
        items: helpers::parse_directory(custom_data_path.to_owned() + "Items"),
        status_effects: helpers::parse_directory(custom_data_path.to_owned() + "StatusEffects"),
        potions: helpers::parse_directory(custom_data_path.to_owned() + "Potions"),
        weather: helpers::parse_directory(custom_data_path.to_owned() + "Weather"),
        rules: helpers::parse_directory(custom_data_path.to_owned() + "Rules"),
    }
}
