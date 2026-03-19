use serenity::all::EventHandler;
use serenity::async_trait;

use crate::handler::{message, ready};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::all::Context, msg: serenity::all::Message) {
        message::on_message(ctx, msg).await;
    }

    async fn ready(&self, ctx: serenity::all::Context, ready: serenity::all::Ready) {
        ready::on_ready(ctx, ready).await;
    }
}
