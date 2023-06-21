use tokio::sync::Mutex;
use sqlx::{Pool, Row, Sqlite};
use log::error;

#[derive(Debug, Clone)]
pub struct CharacterCacheItem {
    pub name: String,
    pub user_id: u64,
    pub guild_id: u64,
    autocomplete_name: String,
}

impl CharacterCacheItem {
    pub fn new(name: String, user_id: u64, guild_id: u64, user_nickname: String) -> Self {
        CharacterCacheItem {
            autocomplete_name: CharacterCacheItem::build_autocomplete_name(&name, &user_nickname),
            user_id,
            guild_id,
            name,
        }
    }

    pub fn get_autocomplete_name(&self) -> &String {
        &self.autocomplete_name
    }

    fn build_autocomplete_name(name: &str, nickname: &str) -> String {
        format!("{} (@{})", name, nickname)
    }
}

pub struct Cache {
    character_cache: Mutex<Vec<CharacterCacheItem>>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            character_cache: Mutex::new(Vec::new()),
        }
    }

    pub async fn get_characters(&self) -> Vec<CharacterCacheItem> {
        self.character_cache.lock().await.clone()
    }

    pub async fn update_character_names(&self, db: &Pool<Sqlite>) {
        let entries = sqlx::query!(
"SELECT character.name as character_name, character.user_id, character.guild_id, user_in_guild.name as user_name
FROM character
LEFT JOIN user_in_guild ON
    user_in_guild.user_id = character.user_id AND
    user_in_guild.guild_id = character.guild_id
")
            .fetch_all(db).await;

        match entries {
            Ok(entries) => {
                let mut cache = self.character_cache.lock().await;
                cache.clear();
                for x in entries {
                    cache.push(CharacterCacheItem::new(
                        x.character_name,
                        x.user_id as u64,
                        x.guild_id as u64,
                        x.user_name.unwrap_or(String::new())
                    ))
                }
            }
            Err(e) => {
                error!("Was unable to update character names! {}", e);
            }
        }
    }
}
