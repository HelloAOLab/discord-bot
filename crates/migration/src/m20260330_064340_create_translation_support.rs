use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Translations::Table)
                    .if_not_exists()
                    .col(pk_auto(Translations::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TranslationSupportItem::Table)
                    .if_not_exists()
                    .col(pk_auto(TranslationSupportItem::Id))
                    .col(integer(TranslationSupportItem::TranslationId))
                    .col(string(TranslationSupportItem::TranslationKey))
                    .col(string(TranslationSupportItem::LanguageCode))
                    .col(boolean(TranslationSupportItem::Complete))
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                TranslationSupportItem::Table,
                                TranslationSupportItem::TranslationId,
                            )
                            .to(Translations::Table, Translations::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(TranslationSupportItem::Table)
                    .col(TranslationSupportItem::LanguageCode)
                    .col(TranslationSupportItem::TranslationKey)
                    .unique()
                    .name("idx_translation_support_item_unique")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_translation_support_item_unique")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(TranslationSupportItem::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Translations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Translations {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum TranslationSupportItem {
    Table,
    Id,
    TranslationId,
    TranslationKey,
    LanguageCode,
    Complete,
}
