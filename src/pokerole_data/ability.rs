use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokeroleAbility {
    pub name: String,
    pub description: String,
    pub effect: String,
}
