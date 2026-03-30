use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectOptions, Database, DatabaseConnection};
use serenity::async_trait;

use crate::store::{
    entity::{server_prefs, user_prefs},
    store::{DailyCache, ServerPref, UserPref},
};

pub struct SqliteStore {
    connection: DatabaseConnection,
}

impl SqliteStore {
    /**
     * See https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/
     */
    pub async fn new(db_url: &str) -> Result<Self, sea_orm::DbErr> {
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(1);
        let connection: DatabaseConnection = Database::connect(opt).await?;
        Migrator::up(&connection, None).await?;
        Ok(Self { connection })
    }
}

impl DailyCache for SqliteStore {}

#[async_trait]
impl UserPref for SqliteStore {
    async fn get_user_translation(&self, user_id: String) -> String {
        user_prefs::Entity::find_by_user_id(&user_id)
            .one(&self.connection)
            .await
            .unwrap()
            .map(|pref| pref.translation_key)
            .unwrap_or_else(|| {
                // Default translation key if user preference is not found
                "".to_string()
            })
    }
    async fn set_user_translation(&self, user_id: String, translation_key: String) {
        let existing_pref = user_prefs::Entity::find_by_user_id(&user_id)
            .one(&self.connection)
            .await
            .unwrap();

        if let Some(pref) = existing_pref {
            let mut active_model: user_prefs::ActiveModel = pref.into();
            active_model.translation_key = Set(translation_key);
            active_model.update(&self.connection).await.unwrap();
        } else {
            let new_pref = user_prefs::ActiveModel {
                id: Set(Default::default()),
                user_id: Set(user_id),
                translation_key: Set(translation_key),
                language_code: Set("".to_string()), // Default language code
            };
            new_pref.insert(&self.connection).await.unwrap();
        }
    }
}

#[async_trait]
impl ServerPref for SqliteStore {
    async fn get_server_translation(&self, guild_id: String) -> String {
        server_prefs::Entity::find_by_server_id(&guild_id)
            .one(&self.connection)
            .await
            .unwrap()
            .map(|pref| pref.translation_key)
            .unwrap_or_else(|| {
                // Default translation key if server preference is not found
                "".to_string()
            })
    }
}
