use crate::data::parser::custom_data::custom_move::CustomMove;
use crate::data::parser::custom_data::custom_pokemon::CustomPokemon;
use crate::data::parser::helpers;

pub struct CustomDataParseResult {
    pub pokemon: Vec<CustomPokemon>,
    pub moves: Vec<CustomMove>
}

pub fn parse(custom_data_path: &str) -> CustomDataParseResult {
    let pokemon: Vec<CustomPokemon> = helpers::parse_directory(custom_data_path.to_owned() + "Pokedex");
    let moves: Vec<CustomMove> = helpers::parse_directory(custom_data_path.to_owned() + "Moves");

    CustomDataParseResult {
        pokemon,
        moves,
    }
}
