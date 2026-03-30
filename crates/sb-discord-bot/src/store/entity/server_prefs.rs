use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "server_prefs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    // The default langauge code for this server: e.g. "en", "zh-CN"
    pub language_code: String,
    // The default translation key for this server: e.g. "RVR", "RVC"
    pub translation_key: String,
    // The server ID in Discord.
    #[sea_orm(unique)]
    pub server_id: String,
}

impl ActiveModelBehavior for ActiveModel {}
