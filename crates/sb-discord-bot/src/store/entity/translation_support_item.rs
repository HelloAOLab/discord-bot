use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "translation_support_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub translation_id: i32,
    // The translation key: e.g. "RVR", "RVC"
    pub translation_key: String,
    // The language code: e.g. "en", "zh-CN"
    pub language_code: String,
    // Whether both OT and NT are supported for this translation key and language code
    pub complete: bool,
    #[sea_orm(belongs_to, from = "translation_id", to = "id")]
    pub translation: Option<super::translations::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
