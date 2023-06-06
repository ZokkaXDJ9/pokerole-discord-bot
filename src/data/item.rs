use std::str::FromStr;
use crate::data::parser::custom_data::custom_item::CustomItem;
use crate::data::pokerole_data::raw_item::RawPokeroleItem;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub price: Option<u16>,
    pub description: String,
    pub category: String,
    pub single_use: bool,
}

impl Item {
    pub(in crate::data) fn new(raw: RawPokeroleItem) -> Self {
        Item {
            name: raw.name,
            price: Item::parse_price(raw.pmd_price, raw.trainer_price),
            description: raw.description,
            category: Item::parse_category(raw.pocket, raw.category),
            single_use: raw.one_use,
        }
    }

    pub(in crate::data) fn from_custom_data(raw: &CustomItem) -> Self {
        Item {
            name: raw.name.clone(),
            price: Item::parse_price(raw.price, None),
            description: raw.description.clone(),
            category: raw.category.clone(),
            single_use: raw.single_use,
        }
    }

    fn parse_price(pmd: Option<u16>, trainer: Option<String>) -> Option<u16> {
        if let Some(pmd_price) = pmd {
            if pmd_price == 0 {
                return None;
            }

            return pmd;
        }

        Item::parse_trainer_price(trainer)
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
