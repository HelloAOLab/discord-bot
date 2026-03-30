use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServerPrefs::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(ServerPrefs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ServerPrefs::LanguageCode)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServerPrefs::TranslationKey)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServerPrefs::GuildId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServerPrefs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ServerPrefs {
    Table,
    Id,
    LanguageCode,
    TranslationKey,
    GuildId,
}
