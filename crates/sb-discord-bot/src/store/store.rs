use serenity::{async_trait, prelude::TypeMapKey};
use std::sync::Arc;

#[async_trait]
pub trait DailyCache: Send + Sync {}

#[async_trait]
pub trait UserPref: Send + Sync {
    async fn get_user_translation(&self, user_id: String) -> String;
    async fn set_user_translation(&self, user_id: String, translation_key: String);
}

#[async_trait]
pub trait ServerPref: Send + Sync {
    async fn get_server_translation(&self, guild_id: String) -> String;
}

pub trait Store: UserPref + ServerPref + DailyCache {}

impl<T: UserPref + ServerPref + DailyCache> Store for T {}

pub struct StoreKey;
impl TypeMapKey for StoreKey {
    type Value = Arc<dyn Store>;
}
