use std::sync::Arc;

use serenity::all::{ChannelId, Colour, CreateEmbed, CreateMessage, Http};

use crate::{
    store::store::Store,
    util::{
        format::get_session_url,
        prefs::{calc_lang, calc_translation},
    },
};

/// Fires when a scheduled session's start time arrives: posts a personalized
/// link for every current subscriber (same shared `sessionId`, each in their
/// own language/translation) and marks the session started.
pub async fn run_start_job(http: Arc<Http>, store: Arc<dyn Store>, session_id: i32) {
    let Some(session) = store.get_session(session_id).await else {
        return;
    };
    if session.status != "pending" {
        return;
    }

    let subscribers = store.get_subscribers(session_id).await;
    let chapter_string = session.chapter.to_string();

    let mut description = String::new();
    for user_id in &subscribers {
        let translation =
            calc_translation(None, user_id, Some(&session.guild_id), store.as_ref()).await;
        let lang = calc_lang(None, user_id, Some(&session.guild_id), store.as_ref()).await;
        let url = get_session_url(
            &session.session_uuid,
            &session.book_id,
            &chapter_string,
            &translation,
            &lang,
        );
        description.push_str(&format!("<@{user_id}> — {url}\n"));
    }
    if description.is_empty() {
        description.push_str("No one subscribed to this session.");
    }

    let Ok(channel_id) = session.channel_id.parse::<u64>() else {
        return;
    };
    let embed = CreateEmbed::default()
        .title(format!(
            "📖 Session starting: {} {}",
            session.book_id, session.chapter
        ))
        .description(description)
        .color(Colour::new(2736712));
    let _ = ChannelId::new(channel_id)
        .send_message(&http, CreateMessage::new().embed(embed))
        .await;

    store.mark_session_started(session_id).await;
}

/// Fires `reminder_minutes` before the session start: pings current
/// subscribers that it's coming up soon, without yet handing out links.
pub async fn run_reminder_job(http: Arc<Http>, store: Arc<dyn Store>, session_id: i32) {
    let Some(session) = store.get_session(session_id).await else {
        return;
    };
    if session.status != "pending" || session.reminder_sent {
        return;
    }

    let subscribers = store.get_subscribers(session_id).await;
    if subscribers.is_empty() {
        store.mark_reminder_sent(session_id).await;
        return;
    }
    let mentions = subscribers
        .iter()
        .map(|u| format!("<@{u}>"))
        .collect::<Vec<_>>()
        .join(" ");

    let Ok(channel_id) = session.channel_id.parse::<u64>() else {
        return;
    };
    let content = format!(
        "⏰ {} — the **{} {}** session is starting soon!",
        mentions, session.book_id, session.chapter
    );
    let _ = ChannelId::new(channel_id)
        .send_message(&http, CreateMessage::new().content(content))
        .await;

    store.mark_reminder_sent(session_id).await;
}
