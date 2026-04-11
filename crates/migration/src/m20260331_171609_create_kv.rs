use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KV::Table)
                    .if_not_exists()
                    .col(pk_auto(KV::Id))
                    .col(string(KV::Key).not_null().unique_key())
                    .col(string(KV::Value).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KV::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum KV {
    Table,
    Id,
    Key,
    Value,
}
