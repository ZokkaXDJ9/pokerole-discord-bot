use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokeroleMove {
    pub name: String,
    pub r#type: String,
    pub power: u32,
    pub damage1: String,
    pub damage2: String,
    pub accuracy1: String,
    pub accuracy2: String,
    pub target: String,
    pub effect: String,
    pub description: String,
    //pub attributes - includes stuff like never_fail: bool. But that's already written in effect.
    //pub added_effects - includes stuff like stat changes. PARSEABLE stat changes! But they are already written in effect.
    pub category: String,
}
