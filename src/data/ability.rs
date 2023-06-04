use serde::Deserialize;
use crate::data::pokerole_data::raw_ability::RawPokeroleAbility;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ability {
    pub name: String,
    pub description: String,
    pub effect: String,
}

impl Ability {
    pub(in crate::data) fn new(raw: RawPokeroleAbility) -> Ability {
        Ability {
            name: raw.name,
            description: raw.description,
            effect: raw.effect,
        }
    }
}
