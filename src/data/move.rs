use std::str::FromStr;
use log::error;
use crate::data::pokerole_data::raw_move::RawPokeroleMove;
use crate::enums::{CombatOrSocialStat, HappinessDamageModifier, MoveCategory, MoveType, Stat};

pub struct Move {
    pub name: String,
    pub typing: MoveType,
    pub power: u8,
    pub damage1: Option<Stat>,
    pub happiness_damage: Option<HappinessDamageModifier>,
    pub accuracy1: Option<CombatOrSocialStat>,
    pub accuracy2: Option<CombatOrSocialStat>,
    pub target: String,
    pub effect: String,
    pub description: String,
    //pub attributes - includes stuff like never_fail: bool. But that's already written in effect.
    //pub added_effects - includes stuff like stat changes. PARSEABLE stat changes! But they are already written in effect.
    pub category: MoveCategory,
}

impl Move {
    pub(in crate::data) fn new(raw: RawPokeroleMove) -> Self {
        Move {
            name: raw.name,
            typing: raw.r#type,
            power: raw.power,
            damage1: Move::parse_damage1(raw.damage1),
            happiness_damage: Move::parse_happiness_damage(raw.damage2),
            accuracy1: Move::parse_accuracy(raw.accuracy1),
            accuracy2: Move::parse_accuracy(raw.accuracy2),
            target: raw.target,
            effect: raw.effect,
            description: raw.description,
            category: raw.category
        }
    }

    fn parse_damage1(raw: String) -> Option<Stat> {
        if raw.is_empty() {
            return None;
        }

        return match Stat::from_str(&raw) {
            Ok(result) => Some(result),
            Err(_) => {
                match raw.as_str() {
                    "Strength/special" => Some(Stat::StrengthOrSpecial),
                    "Same as the copied move" => Some(Stat::Copy),
                    _ => {
                        error!("Cannot parse damage modifier: {}", &raw);
                        None
                    }
                }
            }
        }
    }

    fn parse_happiness_damage(raw: String) -> Option<HappinessDamageModifier> {
        if raw.is_empty() {
            return None;
        }

        return match raw.as_str() {
            "Happiness" => Some(HappinessDamageModifier::Happiness),
            "Missing happiness" => Some(HappinessDamageModifier::MissingHappiness),
            _ => {
                error!("Cannot parse happiness damage modifier: {}", &raw);
                None
            }
        }
    }

    fn parse_accuracy(raw: String) -> Option<CombatOrSocialStat> {
        if raw.is_empty() {
            return None;
        }

        return match CombatOrSocialStat::from_str(&raw) {
            Ok(result) => Some(result),
            Err(_) => {
                match raw.as_str() {
                    "Missing beauty" => Some(CombatOrSocialStat::MissingBeauty),
                    "BRAWL/CHANNEL" => Some(CombatOrSocialStat::BrawlOrChannel),
                    "Tough/cute" => Some(CombatOrSocialStat::ToughOrCute),
                    "Same as the copied move" => Some(CombatOrSocialStat::Copied),
                    "BRAWL" => Some(CombatOrSocialStat::Brawl),
                    "PERFORM" => Some(CombatOrSocialStat::Perform),
                    "ALLURE" => Some(CombatOrSocialStat::Allure),
                    _ => {
                        error!("Cannot parse accuracy modifier: {}", &raw);
                        None
                    }
                }
            }
        }
    }
}
