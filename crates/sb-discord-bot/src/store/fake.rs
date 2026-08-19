use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use serenity::async_trait;

use crate::store::store::{
    DailyCache, ScheduledSession, ServerPref, SessionStore, UserPref, VotdStore,
};

/// In-memory `Store` implementation for tests. Never touches a real database.
#[derive(Default)]
pub struct FakeStore {
    user_translations: Mutex<HashMap<String, String>>,
    user_languages: Mutex<HashMap<String, String>>,
    server_translations: Mutex<HashMap<String, String>>,
    server_languages: Mutex<HashMap<String, String>>,
    daily_verse_roles: Mutex<HashMap<String, String>>,
    votd: Mutex<HashMap<String, (String, i64, i64)>>,
    seed_bible_links_enabled: Mutex<HashMap<String, bool>>,
    inline_detection_enabled: Mutex<HashMap<String, bool>>,
    scheduled_sessions: Mutex<HashMap<i32, ScheduledSession>>,
    next_session_id: Mutex<i32>,
    session_subscribers: Mutex<HashMap<i32, Vec<String>>>,
}

impl FakeStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_user_translation(self, user_id: &str, translation: &str) -> Self {
        self.user_translations
            .lock()
            .unwrap()
            .insert(user_id.to_string(), translation.to_string());
        self
    }

    pub fn with_server_translation(self, guild_id: &str, translation: &str) -> Self {
        self.server_translations
            .lock()
            .unwrap()
            .insert(guild_id.to_string(), translation.to_string());
        self
    }

    pub fn with_user_language(self, user_id: &str, language: &str) -> Self {
        self.user_languages
            .lock()
            .unwrap()
            .insert(user_id.to_string(), language.to_string());
        self
    }

    pub fn with_server_language(self, guild_id: &str, language: &str) -> Self {
        self.server_languages
            .lock()
            .unwrap()
            .insert(guild_id.to_string(), language.to_string());
        self
    }

    pub fn with_daily_verse_role(self, guild_id: &str, role_id: &str) -> Self {
        self.daily_verse_roles
            .lock()
            .unwrap()
            .insert(guild_id.to_string(), role_id.to_string());
        self
    }

    pub fn with_votd(self, guild_id: &str, book_3c_id: &str, chapter: i64, verse: i64) -> Self {
        self.votd.lock().unwrap().insert(
            guild_id.to_string(),
            (book_3c_id.to_string(), chapter, verse),
        );
        self
    }

    pub fn with_seed_bible_links_enabled(self, guild_id: &str, enabled: bool) -> Self {
        self.seed_bible_links_enabled
            .lock()
            .unwrap()
            .insert(guild_id.to_string(), enabled);
        self
    }

    pub fn with_inline_detection_enabled(self, guild_id: &str, enabled: bool) -> Self {
        self.inline_detection_enabled
            .lock()
            .unwrap()
            .insert(guild_id.to_string(), enabled);
        self
    }

    pub fn user_translation(&self, user_id: &str) -> Option<String> {
        self.user_translations.lock().unwrap().get(user_id).cloned()
    }

    pub fn server_translation(&self, guild_id: &str) -> Option<String> {
        self.server_translations
            .lock()
            .unwrap()
            .get(guild_id)
            .cloned()
    }

    pub fn daily_verse_role(&self, guild_id: &str) -> Option<String> {
        self.daily_verse_roles
            .lock()
            .unwrap()
            .get(guild_id)
            .cloned()
    }

    pub fn votd_for(&self, guild_id: &str) -> Option<(String, i64, i64)> {
        self.votd.lock().unwrap().get(guild_id).cloned()
    }

    /// Seeds a scheduled session directly (bypassing `create_scheduled_session`),
    /// auto-assigning its id (any `id` on the passed-in `ScheduledSession` is
    /// ignored). Useful for tests that need a session to already exist (e.g.
    /// reaction-handler tests).
    pub fn with_scheduled_session(self, mut session: ScheduledSession) -> Self {
        let id = {
            let mut next_id = self.next_session_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };
        session.id = id;
        self.scheduled_sessions.lock().unwrap().insert(id, session);
        self
    }
}

#[async_trait]
impl UserPref for FakeStore {
    async fn get_user_translation(&self, user_id: String) -> Option<String> {
        self.user_translation(&user_id)
    }

    async fn get_user_language(&self, user_id: String) -> Option<String> {
        self.user_languages.lock().unwrap().get(&user_id).cloned()
    }

    async fn set_user_translation(&self, user_id: String, translation_key: &String) {
        self.user_translations
            .lock()
            .unwrap()
            .insert(user_id, translation_key.clone());
    }
}

#[async_trait]
impl ServerPref for FakeStore {
    async fn get_server_translation(&self, guild_id: String) -> Option<String> {
        self.server_translation(&guild_id)
    }

    async fn get_server_language(&self, guild_id: String) -> Option<String> {
        self.server_languages
            .lock()
            .unwrap()
            .get(&guild_id)
            .cloned()
    }

    async fn set_server_translation(&self, guild_id: String, translation_key: String) {
        self.server_translations
            .lock()
            .unwrap()
            .insert(guild_id, translation_key);
    }

    async fn get_daily_verse_role(&self, guild_id: String) -> Option<String> {
        self.daily_verse_role(&guild_id)
    }

    async fn set_daily_verse_role(&self, guild_id: String, role_id: String) {
        self.daily_verse_roles
            .lock()
            .unwrap()
            .insert(guild_id, role_id);
    }

    async fn get_seed_bible_links_enabled(&self, guild_id: String) -> bool {
        self.seed_bible_links_enabled
            .lock()
            .unwrap()
            .get(&guild_id)
            .copied()
            .unwrap_or(true)
    }

    async fn set_seed_bible_links_enabled(&self, guild_id: String, enabled: bool) {
        self.seed_bible_links_enabled
            .lock()
            .unwrap()
            .insert(guild_id, enabled);
    }

    async fn get_inline_detection_enabled(&self, guild_id: String) -> bool {
        self.inline_detection_enabled
            .lock()
            .unwrap()
            .get(&guild_id)
            .copied()
            .unwrap_or(true)
    }

    async fn set_inline_detection_enabled(&self, guild_id: String, enabled: bool) {
        self.inline_detection_enabled
            .lock()
            .unwrap()
            .insert(guild_id, enabled);
    }
}

#[async_trait]
impl DailyCache for FakeStore {
    async fn get_all_available_translations(&self) -> Option<&'static HashSet<String>> {
        None
    }
}

#[async_trait]
impl VotdStore for FakeStore {
    async fn get_server_votd(&self, guild_id: &str) -> Option<(String, i64, i64)> {
        self.votd_for(guild_id)
    }

    async fn set_server_votd(&self, guild_id: &str, book_3c_id: &str, chapter: i64, verse: i64) {
        self.votd.lock().unwrap().insert(
            guild_id.to_string(),
            (book_3c_id.to_string(), chapter, verse),
        );
    }
}

#[async_trait]
impl SessionStore for FakeStore {
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
    ) -> ScheduledSession {
        let mut next_id = self.next_session_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        let session = ScheduledSession {
            id,
            session_uuid,
            guild_id,
            channel_id,
            creator_id,
            book_id,
            chapter,
            announcement_message_id: None,
            scheduled_at,
            reminder_minutes,
            reminder_sent: false,
            status: "pending".to_string(),
        };
        self.scheduled_sessions
            .lock()
            .unwrap()
            .insert(id, session.clone());
        session
    }

    async fn set_session_announcement_message(&self, session_id: i32, message_id: String) {
        if let Some(session) = self.scheduled_sessions.lock().unwrap().get_mut(&session_id) {
            session.announcement_message_id = Some(message_id);
        }
    }

    async fn get_session_by_announcement_message(
        &self,
        message_id: &str,
    ) -> Option<ScheduledSession> {
        self.scheduled_sessions
            .lock()
            .unwrap()
            .values()
            .find(|s| s.announcement_message_id.as_deref() == Some(message_id))
            .cloned()
    }

    async fn get_session(&self, session_id: i32) -> Option<ScheduledSession> {
        self.scheduled_sessions
            .lock()
            .unwrap()
            .get(&session_id)
            .cloned()
    }

    async fn list_pending_sessions(&self) -> Vec<ScheduledSession> {
        self.scheduled_sessions
            .lock()
            .unwrap()
            .values()
            .filter(|s| s.status == "pending")
            .cloned()
            .collect()
    }

    async fn mark_reminder_sent(&self, session_id: i32) {
        if let Some(session) = self.scheduled_sessions.lock().unwrap().get_mut(&session_id) {
            session.reminder_sent = true;
        }
    }

    async fn mark_session_started(&self, session_id: i32) {
        if let Some(session) = self.scheduled_sessions.lock().unwrap().get_mut(&session_id) {
            session.status = "started".to_string();
        }
    }

    async fn add_subscriber(&self, session_id: i32, user_id: String) {
        let mut subscribers = self.session_subscribers.lock().unwrap();
        let entry = subscribers.entry(session_id).or_default();
        if !entry.contains(&user_id) {
            entry.push(user_id);
        }
    }

    async fn remove_subscriber(&self, session_id: i32, user_id: String) {
        if let Some(entry) = self
            .session_subscribers
            .lock()
            .unwrap()
            .get_mut(&session_id)
        {
            entry.retain(|u| u != &user_id);
        }
    }

    async fn get_subscribers(&self, session_id: i32) -> Vec<String> {
        self.session_subscribers
            .lock()
            .unwrap()
            .get(&session_id)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod session_store_tests {
    use super::*;

    #[tokio::test]
    async fn create_scheduled_session_round_trips() {
        let store = FakeStore::new();
        let session = store
            .create_scheduled_session(
                "uuid-1".to_string(),
                "g1".to_string(),
                "c1".to_string(),
                "creator1".to_string(),
                "JHN".to_string(),
                3,
                1000,
                Some(10),
            )
            .await;
        assert_eq!(session.status, "pending");
        assert!(!session.reminder_sent);
        assert_eq!(store.get_session(session.id).await, Some(session));
    }

    #[tokio::test]
    async fn announcement_message_lookup_round_trips() {
        let store = FakeStore::new();
        let session = store
            .create_scheduled_session(
                "uuid-2".to_string(),
                "g1".to_string(),
                "c1".to_string(),
                "creator1".to_string(),
                "JHN".to_string(),
                3,
                1000,
                None,
            )
            .await;
        store
            .set_session_announcement_message(session.id, "msg-1".to_string())
            .await;
        let found = store
            .get_session_by_announcement_message("msg-1")
            .await
            .unwrap();
        assert_eq!(found.id, session.id);
        assert_eq!(found.announcement_message_id, Some("msg-1".to_string()));
    }

    #[tokio::test]
    async fn subscribers_add_remove_and_list() {
        let store = FakeStore::new();
        let session = store
            .create_scheduled_session(
                "uuid-3".to_string(),
                "g1".to_string(),
                "c1".to_string(),
                "creator1".to_string(),
                "JHN".to_string(),
                3,
                1000,
                None,
            )
            .await;
        store.add_subscriber(session.id, "u1".to_string()).await;
        store.add_subscriber(session.id, "u2".to_string()).await;
        store.add_subscriber(session.id, "u1".to_string()).await; // duplicate, no-op
        let mut subs = store.get_subscribers(session.id).await;
        subs.sort();
        assert_eq!(subs, vec!["u1".to_string(), "u2".to_string()]);

        store.remove_subscriber(session.id, "u1".to_string()).await;
        assert_eq!(
            store.get_subscribers(session.id).await,
            vec!["u2".to_string()]
        );
    }

    #[tokio::test]
    async fn list_pending_sessions_excludes_started() {
        let store = FakeStore::new();
        let pending = store
            .create_scheduled_session(
                "uuid-4".to_string(),
                "g1".to_string(),
                "c1".to_string(),
                "creator1".to_string(),
                "JHN".to_string(),
                3,
                1000,
                None,
            )
            .await;
        let started = store
            .create_scheduled_session(
                "uuid-5".to_string(),
                "g1".to_string(),
                "c1".to_string(),
                "creator1".to_string(),
                "JHN".to_string(),
                3,
                1000,
                None,
            )
            .await;
        store.mark_session_started(started.id).await;

        let ids: Vec<i32> = store
            .list_pending_sessions()
            .await
            .into_iter()
            .map(|s| s.id)
            .collect();
        assert_eq!(ids, vec![pending.id]);
    }

    #[tokio::test]
    async fn mark_reminder_sent_updates_flag() {
        let store = FakeStore::new();
        let session = store
            .create_scheduled_session(
                "uuid-6".to_string(),
                "g1".to_string(),
                "c1".to_string(),
                "creator1".to_string(),
                "JHN".to_string(),
                3,
                1000,
                Some(5),
            )
            .await;
        store.mark_reminder_sent(session.id).await;
        assert!(store.get_session(session.id).await.unwrap().reminder_sent);
    }
}
