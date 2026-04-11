pub mod discord_util;
pub mod handler;
pub mod poise_impl;
pub mod store;
pub mod util;

use std::sync::Arc;

use crate::{
    handler::handler::Handler,
    poise_impl::framework::create_poise_framework,
    store::{
        sqlite::SqliteStore,
        store::{Store, StoreKey},
        valid_cache::init_valid_translations,
    },
    util::{
        client::{create_client, start_client},
        gateway::get_niche_intents,
        token::get_token,
    },
};

pub struct AppData {
    pub store: Arc<dyn Store>,
}

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://dev.db".into());
    let store: Arc<dyn Store> = Arc::new(SqliteStore::new(&db_url).await.unwrap());
    init_valid_translations().await.unwrap();
    let mut client = create_client(
        get_token(),
        get_niche_intents(),
        Handler,
        create_poise_framework(Arc::clone(&store)).await,
    )
    .await;

    {
        let mut data = client.data.write().await;
        data.insert::<StoreKey>(Arc::clone(&store));
    }
    start_client(client).await;
}
