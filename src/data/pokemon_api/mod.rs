pub(in super) mod pokemon_api_parser;
pub(in crate::data::pokemon_api) mod api_types;

#[derive(Debug, serde::Deserialize, Eq, PartialEq, Hash)]
pub struct PokemonApiId(pub u16);
