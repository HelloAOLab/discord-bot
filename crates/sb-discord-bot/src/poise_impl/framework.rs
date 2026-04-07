use crate::poise_impl::{data::Data, types::Error};
use std::sync::Arc;

use crate::{
    poise_impl::slash_commands::{self},
    store::store::Store,
};

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
        .setup(|ctx, _ready, framework: &poise::Framework<Data, Error>| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { store })
            })
        })
        .build()
}
