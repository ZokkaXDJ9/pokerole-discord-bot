use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokeroleNature {
    pub name: String,
    pub nature: String,
    pub confidence: u32,
    pub keywords: String,
    pub description: String,
}
