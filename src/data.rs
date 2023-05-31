use std::collections::HashMap;
use std::sync::Arc;

use crate::{PokeAbility, PokeMove};

pub struct Data {
    pub abilities: Arc<HashMap<String, PokeAbility>>,
    pub ability_names: Arc<Vec<String>>,
    pub moves: Arc<HashMap<String, PokeMove>>,
    pub move_names: Arc<Vec<String>>
} // User data, which is stored and accessible in all command invocations
