use std::collections::HashMap;
use std::sync::Arc;

use crate::{PokeAbility, PokeLearn, PokeMove, PokeStats};

pub struct Data {
    pub abilities: Arc<HashMap<String, PokeAbility>>,
    pub ability_names: Arc<Vec<String>>,
    pub moves: Arc<HashMap<String, PokeMove>>,
    pub move_names: Arc<Vec<String>>,
    pub pokemon: Arc<HashMap<String, PokeStats>>,
    pub pokemon_names: Arc<Vec<String>>,
    pub pokemon_learns: Arc<Vec<PokeLearn>>,
} // User data, which is stored and accessible in all command invocations
