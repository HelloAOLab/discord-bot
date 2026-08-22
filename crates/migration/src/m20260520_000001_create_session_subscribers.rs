use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SessionSubscribers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SessionSubscribers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SessionSubscribers::SessionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SessionSubscribers::UserId)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(SessionSubscribers::Table)
                    .col(SessionSubscribers::SessionId)
                    .col(SessionSubscribers::UserId)
                    .unique()
                    .name("idx_session_subscriber_unique")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(SessionSubscribers::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum SessionSubscribers {
    Table,
    Id,
    SessionId,
    UserId,
}
