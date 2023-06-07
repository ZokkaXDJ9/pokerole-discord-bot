use crate::data::parser::custom_data::custom_item::CustomItem;
use crate::data::parser::custom_data::custom_move::CustomMove;
use crate::data::parser::custom_data::custom_pokemon::CustomPokemon;
use crate::data::parser::custom_data::custom_status_effect::CustomStatusEffect;
use crate::data::parser::helpers;

pub struct CustomDataParseResult {
    pub pokemon: Vec<CustomPokemon>,
    pub moves: Vec<CustomMove>,
    pub items: Vec<CustomItem>,
    pub status_effects: Vec<CustomStatusEffect>,
}

pub fn parse(custom_data_path: &str) -> CustomDataParseResult {
    CustomDataParseResult {
        pokemon: helpers::parse_directory(custom_data_path.to_owned() + "Pokedex"),
        moves: helpers::parse_directory(custom_data_path.to_owned() + "Moves"),
        items: helpers::parse_directory(custom_data_path.to_owned() + "Items"),
        status_effects: helpers::parse_directory(custom_data_path.to_owned() + "StatusEffects"),
    }
}
