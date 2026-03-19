use serenity::all::{Context, Ready};

pub async fn on_ready(_: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
}
