use std::collections::HashMap;
use std::sync::Arc;

use crate::PokeMove;

pub struct Data {
    pub moves: Arc<HashMap<String, PokeMove>>,
    pub move_names: Arc<Vec<String>>
} // User data, which is stored and accessible in all command invocations
