use crate::data::pokerole_data::raw_nature::RawPokeroleNature;

#[derive(Debug)]
pub struct Nature {
    pub name: String,
    pub keywords: String,
    pub description: String,
}

impl Nature {
    pub(in crate::data) fn new(raw: RawPokeroleNature) -> Self {
        Nature {
            name: raw.name,
            keywords: raw.keywords,
            description: raw.description,
        }
    }
}
