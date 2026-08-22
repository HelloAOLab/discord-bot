use serenity::all::{Context, Reaction, ReactionType};

use crate::store::store::StoreKey;

pub async fn on_reaction_add(ctx: Context, reaction: Reaction) {
    let ReactionType::Unicode(ref emoji) = reaction.emoji else {
        return;
    };
    if emoji != "🔔" {
        return;
    }

    let Some(user_id) = reaction.user_id else {
        return;
    };

    // Ignore the bot's own reaction
    if user_id == ctx.cache.current_user().id {
        return;
    }

    let data = ctx.data.read().await;
    let Some(store) = data.get::<StoreKey>() else {
        return;
    };

    let message_id = reaction.message_id.to_string();
    if let Some(session_id) = store.get_session_id_by_message_id(&message_id).await {
        store.add_subscriber(session_id, &user_id.to_string()).await;
    }
}

pub async fn on_reaction_remove(ctx: Context, reaction: Reaction) {
    let ReactionType::Unicode(ref emoji) = reaction.emoji else {
        return;
    };
    if emoji != "🔔" {
        return;
    }

    let Some(user_id) = reaction.user_id else {
        return;
    };

    let data = ctx.data.read().await;
    let Some(store) = data.get::<StoreKey>() else {
        return;
    };

    let message_id = reaction.message_id.to_string();
    if let Some(session_id) = store.get_session_id_by_message_id(&message_id).await {
        store
            .remove_subscriber(session_id, &user_id.to_string())
            .await;
    }
}
