pub(in crate::game_data::pokemon_api) mod api_types;
pub(super) mod pokemon_api_parser;

#[derive(Debug, serde::Deserialize, Eq, PartialEq, Hash)]
pub struct PokemonApiId(pub u16);
