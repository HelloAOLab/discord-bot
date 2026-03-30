use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_prefs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    // The default langauge code for this user: e.g. "en", "zh-CN"
    pub language_code: String,
    // The default translation key for this user: e.g. "RVR", "RVC"
    pub translation_key: String,
    // The user ID in Discord.
    #[sea_orm(unique)]
    pub user_id: String,
}

impl ActiveModelBehavior for ActiveModel {}
