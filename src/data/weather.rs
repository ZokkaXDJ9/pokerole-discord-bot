use crate::data::pokerole_discord_py_data::pokerole_discord_py_csv_parser::RawPokeWeather;

#[derive(Debug)]
pub struct Weather {
    pub name: String,
    pub description: String,
    pub effect: String,
}

impl Weather {
    pub(in crate::data) fn new(raw: RawPokeWeather) -> Self {
        Weather {
            name: raw.name,
            description: raw.description,
            effect: raw.effect
        }
    }
}
