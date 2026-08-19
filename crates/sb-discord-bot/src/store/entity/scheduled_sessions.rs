use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "scheduled_sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    // UUID placed in the seedbible.org `sessionId` query param; globally unique.
    #[sea_orm(unique)]
    pub session_uuid: String,
    // The Discord guild this session was launched in.
    pub guild_id: String,
    // The channel the announcement (and later, the start/reminder messages) is posted in.
    pub channel_id: String,
    // The Discord user id of whoever ran /launchsession.
    pub creator_id: String,
    // Book 3-character id (e.g. "JHN"), matching BibleBooks::get_3c_id().
    pub book_id: String,
    pub chapter: i64,
    // The bot's own announcement message id, so reaction add/remove can look the session back up.
    // NULL for "now" sessions, which skip the announcement/reaction flow entirely.
    pub announcement_message_id: Option<String>,
    // Unix seconds (UTC) at which the session should start.
    pub scheduled_at: i64,
    // Minutes before scheduled_at to post a reminder ping; NULL means no reminder.
    pub reminder_minutes: Option<i64>,
    pub reminder_sent: bool,
    // "pending" | "started"
    pub status: String,
    pub created_at: i64,
}

impl ActiveModelBehavior for ActiveModel {}
