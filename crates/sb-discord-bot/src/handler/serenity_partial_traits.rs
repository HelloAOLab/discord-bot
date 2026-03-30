use serenity::all::{Context, Message, Ready};

pub trait MessageEventHandler: Send + Sync {
    /// Dispatched when a message is created.
    ///
    /// Provides the message's data.
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn message<'life0, 'async_trait>(
        &'life0 self,
        ctx: Context,
        new_message: Message,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let ctx = ctx;
            let new_message = new_message;
            let _: () = { drop((ctx, new_message)) };
        })
    }
}

pub trait ReadyEventHandler: Send + Sync {
    /// Dispatched upon startup.
    ///
    /// Provides data about the bot and the guilds it's in.
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn ready<'life0, 'async_trait>(
        &'life0 self,
        ctx: Context,
        data_about_bot: Ready,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let ctx = ctx;
            let data_about_bot = data_about_bot;
            let _: () = { drop((ctx, data_about_bot)) };
        })
    }
}
