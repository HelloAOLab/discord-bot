use serenity::{async_trait, prelude::TypeMapKey};
use std::{collections::HashSet, sync::Arc};

#[async_trait]
pub trait DailyCache: Send + Sync {
    async fn get_all_available_translations(&self) -> Option<&'static HashSet<String>>;
}

#[async_trait]
pub trait UserPref: Send + Sync {
    async fn get_user_translation(&self, user_id: String) -> String;
    async fn set_user_translation(&self, user_id: String, translation_key: &String);
}

#[async_trait]
pub trait ServerPref: Send + Sync {
    async fn get_server_translation(&self, guild_id: String) -> String;
    async fn set_server_translation(&self, guild_id: String, translation_key: String);
    async fn get_daily_verse_role(&self, guild_id: String) -> Option<String>;
    async fn set_daily_verse_role(&self, guild_id: String, role_id: String);
}

pub trait Store: UserPref + ServerPref + DailyCache {}

impl<T: UserPref + ServerPref + DailyCache> Store for T {}

pub struct StoreKey;
impl TypeMapKey for StoreKey {
    type Value = Arc<dyn Store>;
}
