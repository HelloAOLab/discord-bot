use std::{sync::Arc, time::Duration};

use serenity::{
    all::{ChannelId, CreateAllowedMentions, CreateMessage, MessageId, UserId},
    http::Http,
};

use crate::store::store::Store;

pub async fn run(store: Arc<dyn Store>, http: Arc<Http>) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        let now = chrono::Utc::now().timestamp();
        let due = store.get_due_sessions(now).await;
        for session in due {
            let subscribers = store.get_subscribers(session.id).await;
            store.mark_session_notified(session.id).await;

            if subscribers.is_empty() {
                continue;
            }

            let user_ids: Vec<UserId> = subscribers
                .iter()
                .filter_map(|uid| uid.parse::<u64>().ok())
                .map(UserId::from)
                .collect();

            let mentions = user_ids
                .iter()
                .map(|uid| format!("<@{}>", uid.get()))
                .collect::<Vec<_>>()
                .join(" ");

            let mut content =
                format!("🔔 **Session starting now!** <t:{}:F>", session.scheduled_time);
            if let Some(ref link) = session.session_link {
                content.push_str(&format!("\n{link}"));
            }
            content.push_str(&format!("\n{mentions}"));

            let Ok(channel_id) = session.channel_id.parse::<u64>() else {
                continue;
            };
            let Ok(message_id) = session.message_id.parse::<u64>() else {
                continue;
            };

            let _ = ChannelId::from(channel_id)
                .send_message(
                    &http,
                    CreateMessage::new()
                        .content(content)
                        .reference_message((ChannelId::from(channel_id), MessageId::from(message_id)))
                        .allowed_mentions(CreateAllowedMentions::new().users(user_ids)),
                )
                .await;
        }
    }
}
