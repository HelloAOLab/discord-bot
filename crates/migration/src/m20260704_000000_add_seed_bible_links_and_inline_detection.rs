use sea_orm_migration::prelude::*;

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
                        ColumnDef::new(ServerPrefs::SeedBibleLinksEnabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(ServerPrefs::InlineDetectionEnabled)
                            .boolean()
                            .not_null()
                            .default(true),
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
                    .drop_column_if_exists(ServerPrefs::SeedBibleLinksEnabled)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .drop_column_if_exists(ServerPrefs::InlineDetectionEnabled)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ServerPrefs {
    Table,
    SeedBibleLinksEnabled,
    InlineDetectionEnabled,
}
