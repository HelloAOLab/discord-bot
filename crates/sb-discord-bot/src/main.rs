pub mod commands;
pub mod handler;
pub mod util;

use crate::{
    commands::framework::create_poise_framework,
    handler::handler::Handler,
    util::{
        client::{create_client, start_client},
        gateway::get_niche_intents,
        token::get_token,
    },
};

#[tokio::main]
async fn main() {
    let client = create_client(
        get_token(),
        get_niche_intents(),
        Handler,
        create_poise_framework().await,
    )
    .await;
    start_client(client).await;
}
