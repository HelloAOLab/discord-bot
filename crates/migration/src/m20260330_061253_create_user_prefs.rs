use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPrefs::Table)
                    .if_not_exists()
                    .col(pk_auto(UserPrefs::Id))
                    .col(
                        ColumnDef::new(UserPrefs::LanguageCode)
                            .string()
                            .not_null()
                            .default("en"),
                    )
                    .col(
                        ColumnDef::new(UserPrefs::TranslationKey)
                            .string()
                            .not_null()
                            .default("BSB"),
                    )
                    .col(
                        ColumnDef::new(UserPrefs::UserId)
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
            .drop_table(Table::drop().table(UserPrefs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserPrefs {
    Table,
    Id,
    LanguageCode,
    TranslationKey,
    UserId,
}
