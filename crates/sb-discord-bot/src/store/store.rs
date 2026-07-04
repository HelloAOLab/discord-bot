use serenity::{async_trait, prelude::TypeMapKey};
use std::{collections::HashSet, sync::Arc};

#[async_trait]
pub trait DailyCache: Send + Sync {
    async fn get_all_available_translations(&self) -> Option<&'static HashSet<String>>;
}

#[async_trait]
pub trait UserPref: Send + Sync {
    async fn get_user_translation(&self, user_id: String) -> Option<String>;
    async fn get_user_language(&self, user_id: String) -> Option<String>;
    async fn set_user_translation(&self, user_id: String, translation_key: &String);
}

#[async_trait]
pub trait ServerPref: Send + Sync {
    async fn get_server_translation(&self, guild_id: String) -> Option<String>;
    async fn get_server_language(&self, guild_id: String) -> Option<String>;
    async fn set_server_translation(&self, guild_id: String, translation_key: String);
    async fn get_daily_verse_role(&self, guild_id: String) -> Option<String>;
    async fn set_daily_verse_role(&self, guild_id: String, role_id: String);
    async fn get_seed_bible_links_enabled(&self, guild_id: String) -> bool;
    async fn set_seed_bible_links_enabled(&self, guild_id: String, enabled: bool);
    async fn get_inline_detection_enabled(&self, guild_id: String) -> bool;
    async fn set_inline_detection_enabled(&self, guild_id: String, enabled: bool);
}

#[async_trait]
pub trait VotdStore: Send + Sync {
    /// Returns the server's verse of the day as `(book_3c_id, chapter, verse)`.
    async fn get_server_votd(&self, guild_id: &str) -> Option<(String, i64, i64)>;
    async fn set_server_votd(&self, guild_id: &str, book_3c_id: &str, chapter: i64, verse: i64);
}

pub trait Store: UserPref + ServerPref + DailyCache + VotdStore {}

impl<T: UserPref + ServerPref + DailyCache + VotdStore> Store for T {}

pub struct StoreKey;
impl TypeMapKey for StoreKey {
    type Value = Arc<dyn Store>;
}
