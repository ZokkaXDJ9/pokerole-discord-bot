use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;

pub fn create_button(label: &str, custom_id: &str, is_disabled: bool) -> CreateButton {
    let mut button = CreateButton::default();
    button.label(label);
    button.custom_id(custom_id);
    button.style(ButtonStyle::Primary);
    button.disabled(is_disabled);
    button
}

pub fn split_long_messages(message: String) -> Vec<String> {
    if message.len() < 2000 {
        return vec!(message);
    }

    let mut remaining = message.as_str();
    let mut result = Vec::default();
    while remaining.len() > 2000 {
        let split_index = find_best_split_pos(remaining);
        let split = remaining.split_at(split_index);

        result.push(split.0.to_string());
        remaining = split.1;
    }
    result.push(remaining.to_string());

    result
}

const MIN_SIZE: usize = 500;
fn find_best_split_pos(message: &str) -> usize {
    let split = message.split_at(2000).0;
    if let Some(index) = split.rfind("\n# ") {
        if index > MIN_SIZE {
            return index;
        }
    }
    if let Some(index) = split.rfind("\n## ") {
        if index > MIN_SIZE {
            return index;
        }
    }
    if let Some(index) = split.rfind("\n### ") {
        if index > MIN_SIZE {
            return index;
        }
    }
    if let Some(index) = split.rfind("\n**") {
        return index;
    }
    if let Some(index) = split.rfind("\n\n") {
        return index;
    }
    if let Some(index) = split.rfind('\n') {
        return index;
    }

    2000
}
