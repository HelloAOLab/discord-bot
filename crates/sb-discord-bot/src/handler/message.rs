use serenity::{
    all::{Context, Message},
    model::user,
};

use crate::store::{store::StoreKey, valid_cache::get_valid_translations};

pub async fn on_message(ctx: Context, msg: Message) {
    if msg.content == "!gmt" {
        // let data = ctx.data.read().await;
        // let store = data.get::<StoreKey>().unwrap();
        let mut translations: Vec<String> = get_valid_translations()
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default();
        translations.sort();

        let sixth = translations.len() / 6;
        for chunk in translations.chunks(sixth.max(1)) {
            if let Err(why) = msg.channel_id.say(&ctx.http, chunk.join(", ")).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
