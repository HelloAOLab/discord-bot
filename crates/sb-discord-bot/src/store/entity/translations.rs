use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "translations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(has_many)]
    pub translations: HasMany<super::translation_support_item::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
