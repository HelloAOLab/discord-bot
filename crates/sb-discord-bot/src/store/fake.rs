use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use serenity::async_trait;

use crate::store::store::{DailyCache, ServerPref, UserPref, VotdStore};

/// In-memory `Store` implementation for tests. Never touches a real database.
#[derive(Default)]
pub struct FakeStore {
    user_translations: Mutex<HashMap<String, String>>,
    user_languages: Mutex<HashMap<String, String>>,
    server_translations: Mutex<HashMap<String, String>>,
    server_languages: Mutex<HashMap<String, String>>,
    daily_verse_roles: Mutex<HashMap<String, String>>,
    votd: Mutex<HashMap<String, (String, i64, i64)>>,
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
