use serenity::{
    all::{Context, Message},
    model::user,
};

use crate::store::store::StoreKey;

pub async fn on_message(ctx: Context, msg: Message) {
    if msg.content == "!gmt" {
        let data = ctx.data.read().await;
        let store = data.get::<StoreKey>().unwrap();
        let user_id = msg.author.id.to_string();
        let translation = store.get_user_translation(user_id).await;
        if let Err(why) = msg
            .channel_id
            .say(
                &ctx.http,
                format!("Test successful! Translation: {}", translation),
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }
}
