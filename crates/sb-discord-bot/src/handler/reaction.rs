use serenity::all::{Context, Reaction, ReactionType};

use crate::store::store::StoreKey;

const SUBSCRIBE_EMOJI: char = '📖';

pub async fn on_reaction_add(ctx: Context, reaction: Reaction) {
    handle(ctx, reaction, true).await;
}

pub async fn on_reaction_remove(ctx: Context, reaction: Reaction) {
    handle(ctx, reaction, false).await;
}

async fn handle(ctx: Context, reaction: Reaction, subscribing: bool) {
    if reaction.emoji != ReactionType::Unicode(SUBSCRIBE_EMOJI.to_string()) {
        return;
    }
    let Some(user_id) = reaction.user_id else {
        return;
    };
    if user_id == ctx.cache.current_user().id {
        return;
    }

    let store = {
        let data = ctx.data.read().await;
        match data.get::<StoreKey>() {
            Some(store) => store.clone(),
            None => return,
        }
    };

    let Some(session) = store
        .get_session_by_announcement_message(&reaction.message_id.to_string())
        .await
    else {
        return;
    };
    if session.status != "pending" {
        return;
    }

    if subscribing {
        store.add_subscriber(session.id, user_id.to_string()).await;
    } else {
        store
            .remove_subscriber(session.id, user_id.to_string())
            .await;
    }
}
