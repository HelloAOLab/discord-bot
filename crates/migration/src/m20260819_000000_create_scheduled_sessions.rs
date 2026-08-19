use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ScheduledSessions::Table)
                    .if_not_exists()
                    .col(pk_auto(ScheduledSessions::Id))
                    .col(string(ScheduledSessions::SessionUuid).unique_key())
                    .col(string(ScheduledSessions::GuildId))
                    .col(string(ScheduledSessions::ChannelId))
                    .col(string(ScheduledSessions::CreatorId))
                    .col(string(ScheduledSessions::BookId))
                    .col(big_integer(ScheduledSessions::Chapter))
                    .col(string_null(ScheduledSessions::AnnouncementMessageId))
                    .col(big_integer(ScheduledSessions::ScheduledAt))
                    .col(big_integer_null(ScheduledSessions::ReminderMinutes))
                    .col(boolean(ScheduledSessions::ReminderSent).default(false))
                    .col(string(ScheduledSessions::Status).default("pending"))
                    .col(big_integer(ScheduledSessions::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ScheduledSessionSubscribers::Table)
                    .if_not_exists()
                    .col(pk_auto(ScheduledSessionSubscribers::Id))
                    .col(integer(ScheduledSessionSubscribers::SessionId))
                    .col(string(ScheduledSessionSubscribers::UserId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-session-subscriber-unique")
                    .table(ScheduledSessionSubscribers::Table)
                    .col(ScheduledSessionSubscribers::SessionId)
                    .col(ScheduledSessionSubscribers::UserId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ScheduledSessionSubscribers::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(ScheduledSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ScheduledSessions {
    Table,
    Id,
    SessionUuid,
    GuildId,
    ChannelId,
    CreatorId,
    BookId,
    Chapter,
    AnnouncementMessageId,
    ScheduledAt,
    ReminderMinutes,
    ReminderSent,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
enum ScheduledSessionSubscribers {
    Table,
    Id,
    SessionId,
    UserId,
}
