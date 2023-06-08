use crate::data::pokerole_data::raw_nature::RawPokeroleNature;

#[derive(Debug)]
pub struct Nature {
    pub name: String,
    pub keywords: String,
    pub description: String,
}

impl Nature {
    pub(crate) fn build_string(&self) -> impl Into<String> + Sized {
        std::format!("### {}\n**Keywords**: {}\n*{}*",
                     &self.name,
                     &self.keywords,
                     &self.description)
    }
}

impl Nature {
    pub(in crate::data) fn new(raw: &RawPokeroleNature) -> Self {
        Nature {
            name: raw.name.clone(),
            keywords: raw.keywords.clone(),
            description: raw.description.clone(),
        }
    }
}
