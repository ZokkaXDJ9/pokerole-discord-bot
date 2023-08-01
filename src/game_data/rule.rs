use serde_with::{serde_as, NoneAsEmptyString};

#[serde_as]
#[derive(serde::Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub flavor: Option<String>,
    pub text: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub example: Option<String>,
}

impl Rule {
    pub(crate) fn build_string(&self) -> impl Into<String> + Sized {
        let mut builder = serenity::utils::MessageBuilder::default();
        builder.push(std::format!("**{}**\n", &self.name));
        if let Some(flavor) = &self.flavor {
            builder.push_italic_line(flavor);
            builder.push('\n');
        }

        builder.push(&self.text);

        if let Some(example) = &self.example {
            builder.push('\n');
            builder.quote_rest();
            builder.push(std::format!("**Example**: {}", example));
        }

        builder.build()
    }
}
