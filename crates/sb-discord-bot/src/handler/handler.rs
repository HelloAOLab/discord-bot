use serenity::all::EventHandler;
use serenity::async_trait;

use crate::handler::{message, reaction, ready};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::all::Context, msg: serenity::all::Message) {
        message::on_message(ctx, msg).await;
    }

    async fn ready(&self, ctx: serenity::all::Context, ready: serenity::all::Ready) {
        ready::on_ready(ctx, ready).await;
    }

    async fn reaction_add(&self, ctx: serenity::all::Context, add_reaction: serenity::all::Reaction) {
        reaction::on_reaction_add(ctx, add_reaction).await;
    }

    async fn reaction_remove(&self, ctx: serenity::all::Context, removed_reaction: serenity::all::Reaction) {
        reaction::on_reaction_remove(ctx, removed_reaction).await;
    }
}
