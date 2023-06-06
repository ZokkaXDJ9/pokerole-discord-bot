use serde::Deserialize;
use crate::enums::{MoveCategory, MoveType};

#[derive(Debug, Deserialize)]
pub struct CustomMove {
    pub name: String,
    pub r#type: MoveType,
    pub power: u8,
    pub damage: String,
    pub accuracy1: String,
    pub accuracy2: String,
    pub target: String,
    pub effect: String,
    pub description: String,
    pub category: MoveCategory,
}
