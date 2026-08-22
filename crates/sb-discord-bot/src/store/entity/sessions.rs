use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub guild_id: String,
    pub channel_id: String,
    #[sea_orm(unique)]
    pub message_id: String,
    pub scheduled_time: i64,
    pub notified: bool,
    pub session_link: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}
