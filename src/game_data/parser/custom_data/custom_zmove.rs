// game_data/parser/custom_data/custom_zmove.rs

use serde::Deserialize;
use crate::enums::{MoveType, MoveCategory};

#[derive(Debug, Deserialize)]
pub struct CustomZMove {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub r#type: MoveType,
    #[serde(rename = "Power")]
    pub power: u16,
    #[serde(rename = "Damage1")]
    pub damage1: String,
    #[serde(rename = "Damage2")]
    pub damage2: String,
    #[serde(rename = "Accuracy1")]
    pub accuracy1: String,
    #[serde(rename = "Accuracy2")]
    pub accuracy2: String,
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(rename = "Effect")]
    pub effect: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "Category")]
    pub category: MoveCategory,
    // Add any additional fields if necessary
}
