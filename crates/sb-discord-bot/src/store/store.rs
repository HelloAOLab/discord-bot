use serenity::{async_trait, prelude::TypeMapKey};
use std::{collections::HashSet, sync::Arc};

pub struct DueSession {
    pub id: i32,
    pub channel_id: String,
    pub message_id: String,
    pub scheduled_time: i64,
    pub session_link: Option<String>,
}

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn create_session(
        &self,
        guild_id: &str,
        channel_id: &str,
        message_id: &str,
        scheduled_time: i64,
        session_link: Option<&str>,
    ) -> i32;
    async fn get_session_id_by_message_id(&self, message_id: &str) -> Option<i32>;
    async fn add_subscriber(&self, session_id: i32, user_id: &str);
    async fn remove_subscriber(&self, session_id: i32, user_id: &str);
    async fn get_subscribers(&self, session_id: i32) -> Vec<String>;
    async fn get_due_sessions(&self, now: i64) -> Vec<DueSession>;
    async fn mark_session_notified(&self, session_id: i32);
}

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
}

#[async_trait]
pub trait VotdStore: Send + Sync {
    /// Returns the server's verse of the day as `(book_3c_id, chapter, verse)`.
    async fn get_server_votd(&self, guild_id: &str) -> Option<(String, i64, i64)>;
    async fn set_server_votd(&self, guild_id: &str, book_3c_id: &str, chapter: i64, verse: i64);
}

pub trait Store: UserPref + ServerPref + DailyCache + VotdStore + SessionStore {}

impl<T: UserPref + ServerPref + DailyCache + VotdStore + SessionStore> Store for T {}

pub struct StoreKey;
impl TypeMapKey for StoreKey {
    type Value = Arc<dyn Store>;
}
