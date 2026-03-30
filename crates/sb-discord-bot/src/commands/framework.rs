use std::sync::Arc;

use crate::{commands::slash_commands::{self, Data}, store::store::Store};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn create_poise_framework(store: Arc<dyn Store>) -> poise::Framework<Data, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: slash_commands::all_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { store })
            })
        })
        .build()
}
