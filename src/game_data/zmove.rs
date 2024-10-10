// game_data/zmove.rs

use crate::enums::{MoveType, MoveCategory, Stat, CombatOrSocialStat};
use crate::game_data::parser::custom_data::custom_zmove::CustomZMove;
use std::str::FromStr;

pub struct ZMove {
    pub name: String,
    pub typing: MoveType,
    pub power: u16,
    pub damage1: Option<Stat>,
    pub damage2: Option<String>,
    pub accuracy1: Option<CombatOrSocialStat>,
    pub accuracy2: Option<CombatOrSocialStat>,
    pub target: String,
    pub effect: Option<String>,
    pub description: Option<String>,
    pub category: MoveCategory,
}

impl ZMove {
    pub fn from_custom(raw: &CustomZMove) -> Self {
        ZMove {
            name: raw.name.clone(),
            typing: raw.r#type.clone(),
            power: raw.power,
            damage1: parse_damage1(&raw.damage1),
            damage2: if raw.damage2.is_empty() { None } else { Some(raw.damage2.clone()) },
            accuracy1: parse_accuracy(&raw.accuracy1),
            accuracy2: parse_accuracy(&raw.accuracy2),
            target: raw.target.clone(),
            effect: if raw.effect.is_empty() { None } else { Some(raw.effect.clone()) },
            description: if raw.description.is_empty() { None } else { Some(raw.description.clone()) },
            category: raw.category.clone(),
        }
    }

    pub fn build_string(&self) -> String {
        // Implement this method to format the Z-Move data for display
        let mut result = format!("### {}\n", self.name);
        if let Some(description) = &self.description {
            result.push_str(&format!("*{}*\n", description));
        }
        result.push_str(&format!("**Type**: {} — **{}**\n", self.typing, self.category));
        result.push_str(&format!("**Target**: {}\n", self.target));

        if self.damage1.is_some() || self.damage2.is_some() || self.power > 0 {
            result.push_str("**Damage Dice**: ");
            if let Some(stat) = &self.damage1 {
                result.push_str(&format!("{} + ", stat));
            }
            if let Some(damage2) = &self.damage2 {
                result.push_str(&format!("{} + ", damage2));
            }
            result.push_str(&format!("{}\n", self.power));
        }

        result.push_str("**Accuracy Dice**: ");
        if let Some(acc) = &self.accuracy1 {
            result.push_str(&acc.to_string());
            if self.accuracy2.is_some() {
                result.push_str(" + Rank");
            }
        }
        result.push('\n');

        if let Some(effect) = &self.effect {
            result.push_str(&format!("**Effect**: {}", effect));
        }

        result
    }
}

// Parsing helper functions
fn parse_damage1(raw: &str) -> Option<Stat> {
    if raw.is_empty() {
        None
    } else {
        Stat::from_str(raw).ok()
    }
}

fn parse_accuracy(raw: &str) -> Option<CombatOrSocialStat> {
    if raw.is_empty() {
        None
    } else {
        CombatOrSocialStat::from_str(raw).ok()
    }
}
