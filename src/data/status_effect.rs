use crate::data::pokerole_discord_py_data::pokerole_discord_py_csv_parser::RawPokeStatus;

pub struct StatusEffect {
    pub name: String,
    pub description: String,
    pub resist: String,
    pub effect: String,
    pub duration: String,
}

impl StatusEffect {
    pub(in crate::data) fn new(raw: RawPokeStatus) -> Self {
        StatusEffect {
            name: raw.name,
            description: raw.description,
            resist: raw.resist,
            effect: raw.effect,
            duration: raw.duration
        }
    }
}
