use serenity::all::{Context, Message};

pub async fn on_message(ctx: Context, msg: Message) {
    // if msg.content == "!ping" {
    //     if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
    //         println!("Error sending message: {why:?}");
    //     }
    // }
}
