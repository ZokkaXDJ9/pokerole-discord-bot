use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;

pub fn create_button(label: &str, custom_id: &str) -> CreateButton {
    let mut button = CreateButton::default();
    button.label(label);
    button.custom_id(custom_id);
    button.style(ButtonStyle::Primary);
    button.disabled(false);
    button
}

pub fn split_long_messages(message: String) -> Vec<String> {
    if message.len() < 2000 {
        return vec!(message);
    }

    // TODO: Prioritize splitting at #, ## and ### sections unless those are too early in the string (<500 or so)
    let split_index = message.split_at(2000).0.rfind("\n**");
    let split = message.split_at(split_index.unwrap_or(2000));

    // TODO: Subsequent splits in case split.1 is still too long
    vec!(split.0.to_string(), split.1.to_string())
}
