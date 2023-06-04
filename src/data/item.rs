use std::str::FromStr;
use crate::data::pokerole_data::raw_item::RawPokeroleItem;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub pmd_price: Option<u16>,
    pub trainer_price: Option<u16>,
    pub description: String,
    pub category: String,
    pub one_use: bool,
}

impl Item {
    pub(in crate::data) fn new(raw: RawPokeroleItem) -> Self {
        Item {
            name: raw.name,
            pmd_price: raw.pmd_price,
            trainer_price: Item::parse_trainer_price(raw.trainer_price),
            description: raw.description,
            category: Item::parse_category(raw.pocket, raw.category),
            one_use: raw.one_use,
        }
    }

    fn parse_trainer_price(raw: Option<String>) -> Option<u16> {
        if let Some(some_raw) = raw {
            return match u16::from_str(&some_raw) {
                Ok(parsed) => Some(parsed),
                Err(_) => None,
            };
        }

        None
    }

    fn parse_category(raw_pocket: String, raw_category: String) -> String {
        if raw_category.is_empty() {
            return raw_category;
        }

        raw_pocket
    }
}
