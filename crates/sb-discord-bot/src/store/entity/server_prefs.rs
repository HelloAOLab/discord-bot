use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "server_prefs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    // The id of the role which will be pinged for daily verses.
    pub daily_verse_role_id: String,
    // The default langauge code for this server: e.g. "en", "zh-CN"
    pub language_code: String,
    // The default translation key for this server: e.g. "RVR", "RVC"
    pub translation_key: String,
    // The server ID in Discord.
    #[sea_orm(unique)]
    pub guild_id: String,
    // Verse of the day — book 3-character ID (e.g. "JHN"), NULL if unset.
    pub votd_book: Option<String>,
    // Verse of the day — chapter number, NULL if unset.
    pub votd_chapter: Option<i64>,
    // Verse of the day — verse number, NULL if unset.
    pub votd_verse: Option<i64>,
}

impl ActiveModelBehavior for ActiveModel {}
