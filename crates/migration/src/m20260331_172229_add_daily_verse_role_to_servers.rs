use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(ServerPrefs::DailyVerseRoleId)
                            .string()
                            .not_null()
                            .default("".to_string()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .drop_column_if_exists(ServerPrefs::DailyVerseRoleId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ServerPrefs {
    Table,
    DailyVerseRoleId,
}
