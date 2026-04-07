use crate::{handler::handler::Handler, poise_impl::data::Data};
use poise::Framework;
use serenity::{Client, all::GatewayIntents};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn create_client(
    token: String,
    intents: GatewayIntents,
    handler: Handler,
    framework: Framework<Data, Error>,
) -> Client {
    Client::builder(&token, intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Err creating client")
}

pub async fn start_client(mut client: Client) {
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
