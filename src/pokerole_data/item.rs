use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokeroleItem {
    pub name: String,
    pub pmd_price: Option<u16>,
    pub pocket: String,
    pub description: String,
    pub category: String,
    pub one_use: bool,
}
