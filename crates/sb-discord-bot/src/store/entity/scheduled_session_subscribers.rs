use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "scheduled_session_subscribers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub session_id: i32,
    pub user_id: String,
}

impl ActiveModelBehavior for ActiveModel {}
