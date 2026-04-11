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
                    .add_column_if_not_exists(ColumnDef::new(ServerPrefs::VotdBook).string().null())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .add_column_if_not_exists(ColumnDef::new(ServerPrefs::VotdChapter).big_integer().null())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .add_column_if_not_exists(ColumnDef::new(ServerPrefs::VotdVerse).big_integer().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .drop_column_if_exists(ServerPrefs::VotdBook)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .drop_column_if_exists(ServerPrefs::VotdChapter)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(ServerPrefs::Table)
                    .drop_column_if_exists(ServerPrefs::VotdVerse)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ServerPrefs {
    Table,
    VotdBook,
    VotdChapter,
    VotdVerse,
}
