use std::collections::HashMap;
use std::sync::Arc;

use crate::{PokeAbility, PokeItem, PokeLearn, PokeMove, PokeStats, PokeStatus, PokeWeather, GameRule};
use crate::pokemon_api_parser::{PokemonApiData};

pub struct Data {
    pub abilities: Arc<HashMap<String, PokeAbility>>,
    pub ability_names: Arc<Vec<String>>,
    pub items: Arc<HashMap<String, PokeItem>>,
    pub item_names: Arc<Vec<String>>,
    pub moves: Arc<HashMap<String, PokeMove>>,
    pub move_names: Arc<Vec<String>>,
    pub pokemon: Arc<HashMap<String, PokeStats>>,
    pub pokemon_names: Arc<Vec<String>>,
    pub pokemon_learns: Arc<Vec<PokeLearn>>,
    pub status_effects: Arc<HashMap<String, PokeStatus>>,
    pub status_effects_names: Arc<Vec<String>>,
    pub weather: Arc<HashMap<String, PokeWeather>>,
    pub weather_names: Arc<Vec<String>>,
    pub pokemon_api_data: Arc<HashMap<String, PokemonApiData>>,
    pub rule_names: Arc<Vec<String>>,
    pub rules: Arc<HashMap<String, GameRule>>,
} // User data, which is stored and accessible in all command invocations
