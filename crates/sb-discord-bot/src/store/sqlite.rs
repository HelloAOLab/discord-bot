use std::collections::HashSet;

use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ConnectOptions, Database, DatabaseConnection,
};
use serenity::{async_trait, model::user};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::store::{
    entity::{kv, server_prefs, user_prefs},
    store::{DailyCache, ServerPref, UserPref, VotdStore},
    valid_cache::{get_valid_translations, is_valid_translation},
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

#[async_trait]
impl DailyCache for SqliteStore {
    async fn get_all_available_translations(&self) -> Option<&'static HashSet<String>> {
        get_valid_translations()
    }
}

#[async_trait]
impl UserPref for SqliteStore {
    async fn get_user_translation(&self, user_id: String) -> Option<String> {
        get_user_pref_by_uid(&self.connection, &user_id)
            .await
            .and_then(|pref| Some(pref.translation_key).filter(|s| !s.is_empty()))
    }

    async fn get_user_language(&self, user_id: String) -> Option<String> {
        get_user_pref_by_uid(&self.connection, &user_id)
            .await
            .and_then(|pref| Some(pref.language_code).filter(|s| !s.is_empty()))
    }
    async fn set_user_translation(&self, user_id: String, translation_key: &String) {
        let existing_pref = user_prefs::Entity::find_by_user_id(&user_id)
            .one(&self.connection)
            .await
            .unwrap();

        if let Some(pref) = existing_pref {
            let mut active_model: user_prefs::ActiveModel = pref.into();
            active_model.translation_key = Set(translation_key.to_string());
            active_model.update(&self.connection).await.unwrap();
        } else {
            let new_pref = user_prefs::ActiveModel {
                id: Set(Default::default()),
                user_id: Set(user_id),
                translation_key: Set(translation_key.to_string()),
                language_code: NotSet,
            };
            new_pref.insert(&self.connection).await.unwrap();
        }
    }
}

#[async_trait]
impl ServerPref for SqliteStore {
    async fn get_server_translation(&self, guild_id: String) -> Option<String> {
        get_server_pref_by_guild_id(&self.connection, &guild_id)
            .await
            .and_then(|pref| Some(pref.translation_key).filter(|s| !s.is_empty()))
    }

    async fn get_server_language(&self, guild_id: String) -> Option<String> {
        get_server_pref_by_guild_id(&self.connection, &guild_id)
            .await
            .and_then(|pref| Some(pref.language_code).filter(|s| !s.is_empty()))
    }

    async fn set_server_translation(&self, guild_id: String, translation_key: String) {
        upsert_server_pref(&self.connection, guild_id, |m| {
            m.translation_key = Set(translation_key);
        })
        .await
    }
    async fn get_daily_verse_role(&self, guild_id: String) -> Option<String> {
        Some(
            get_server_pref_by_guild_id(&self.connection, &guild_id)
                .await
                .map(|pref| pref.daily_verse_role_id)
                .unwrap(),
        )
    }
    async fn set_daily_verse_role(&self, guild_id: String, role_id: String) {
        upsert_server_pref(&self.connection, guild_id, |m| {
            m.daily_verse_role_id = Set(role_id);
        })
        .await
    }
}

async fn get_server_pref_by_guild_id(
    db: &DatabaseConnection,
    guild_id: &String,
) -> Option<server_prefs::Model> {
    server_prefs::Entity::find_by_guild_id(guild_id)
        .one(db)
        .await
        .unwrap()
}

async fn upsert_server_pref<F>(db: &DatabaseConnection, guild_id: String, apply: F)
where
    F: FnOnce(&mut server_prefs::ActiveModel),
{
    let existing = get_server_pref_by_guild_id(db, &guild_id).await;
    let is_new = existing.is_none();

    let mut active_model: server_prefs::ActiveModel = match existing {
        Some(pref) => pref.into(),
        None => server_prefs::ActiveModel {
            id: NotSet,
            guild_id: Set(guild_id),
            translation_key: NotSet,
            language_code: NotSet,
            daily_verse_role_id: NotSet,
            votd_book: NotSet,
            votd_chapter: NotSet,
            votd_verse: NotSet,
        },
    };

    apply(&mut active_model);

    if is_new {
        active_model.insert(db).await.unwrap();
    } else {
        active_model.update(db).await.unwrap();
    }
}

async fn get_user_pref_by_uid(
    db: &DatabaseConnection,
    user_id: &String,
) -> Option<user_prefs::Model> {
    user_prefs::Entity::find_by_user_id(user_id)
        .one(db)
        .await
        .unwrap()
}

#[async_trait]
impl VotdStore for SqliteStore {
    async fn get_server_votd(&self, guild_id: &str) -> Option<(String, i64, i64)> {
        let pref = get_server_pref_by_guild_id(&self.connection, &guild_id.to_string()).await?;
        let book = pref.votd_book?;
        let chapter = pref.votd_chapter?;
        let verse = pref.votd_verse?;
        Some((book, chapter, verse))
    }

    async fn set_server_votd(&self, guild_id: &str, book_3c_id: &str, chapter: i64, verse: i64) {
        upsert_server_pref(&self.connection, guild_id.to_string(), |m| {
            m.votd_book = Set(Some(book_3c_id.to_string()));
            m.votd_chapter = Set(Some(chapter));
            m.votd_verse = Set(Some(verse));
        })
        .await;
    }
}

async fn upsert_user_prefs<F>(db: &DatabaseConnection, user_id: String, apply: F)
where
    F: FnOnce(&mut user_prefs::ActiveModel),
{
    let existing = get_user_pref_by_uid(db, &user_id).await;
    let is_new = existing.is_none();

    let mut active_model: user_prefs::ActiveModel = match existing {
        Some(pref) => pref.into(),
        None => user_prefs::ActiveModel {
            id: NotSet,
            language_code: NotSet,
            translation_key: NotSet,
            user_id: Set(user_id),
        },
    };

    apply(&mut active_model);

    if is_new {
        active_model.insert(db).await.unwrap();
    } else {
        active_model.update(db).await.unwrap();
    }
}
