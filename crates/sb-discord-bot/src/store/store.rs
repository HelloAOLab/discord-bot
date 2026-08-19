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

/// A scheduled `/launchsession` reading session, decoupled from the sea-orm entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduledSession {
    pub id: i32,
    pub session_uuid: String,
    pub guild_id: String,
    pub channel_id: String,
    pub creator_id: String,
    pub book_id: String,
    pub chapter: i64,
    pub announcement_message_id: Option<String>,
    pub scheduled_at: i64,
    pub reminder_minutes: Option<i64>,
    pub reminder_sent: bool,
    pub status: String,
}

#[async_trait]
pub trait SessionStore: Send + Sync {
    #[allow(clippy::too_many_arguments)]
    async fn create_scheduled_session(
        &self,
        session_uuid: String,
        guild_id: String,
        channel_id: String,
        creator_id: String,
        book_id: String,
        chapter: i64,
        scheduled_at: i64,
        reminder_minutes: Option<i64>,
    ) -> ScheduledSession;
    async fn set_session_announcement_message(&self, session_id: i32, message_id: String);
    async fn get_session_by_announcement_message(
        &self,
        message_id: &str,
    ) -> Option<ScheduledSession>;
    async fn get_session(&self, session_id: i32) -> Option<ScheduledSession>;
    /// Returns all sessions still awaiting their start job (`status == "pending"`).
    async fn list_pending_sessions(&self) -> Vec<ScheduledSession>;
    async fn mark_reminder_sent(&self, session_id: i32);
    async fn mark_session_started(&self, session_id: i32);
    async fn add_subscriber(&self, session_id: i32, user_id: String);
    async fn remove_subscriber(&self, session_id: i32, user_id: String);
    async fn get_subscribers(&self, session_id: i32) -> Vec<String>;
}

pub trait Store: UserPref + ServerPref + DailyCache + VotdStore + SessionStore {}

impl<T: UserPref + ServerPref + DailyCache + VotdStore + SessionStore> Store for T {}

pub struct StoreKey;
impl TypeMapKey for StoreKey {
    type Value = Arc<dyn Store>;
}
