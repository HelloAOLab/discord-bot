mod event_handler {
    use super::context::Context;
    use crate::gateway::ShardStageUpdateEvent;
    use crate::http::RatelimitInfo;
    use crate::model::prelude::*;
    use async_trait::async_trait;
    /// The core trait for handling events by serenity.
    pub trait EventHandler: Send + Sync {
        /// Dispatched when the permissions of an application command was updated.
        ///
        /// Provides said permission's data.
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
        fn command_permissions_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            permission: CommandPermissions,
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
                let permission = permission;
                let _: () = { drop((ctx, permission)) };
            })
        }
        /// Dispatched when an auto moderation rule was created.
        ///
        /// Provides said rule's data.
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
        fn auto_moderation_rule_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            rule: Rule,
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
                let rule = rule;
                let _: () = { drop((ctx, rule)) };
            })
        }
        /// Dispatched when an auto moderation rule was updated.
        ///
        /// Provides said rule's data.
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
        fn auto_moderation_rule_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            rule: Rule,
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
                let rule = rule;
                let _: () = { drop((ctx, rule)) };
            })
        }
        /// Dispatched when an auto moderation rule was deleted.
        ///
        /// Provides said rule's data.
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
        fn auto_moderation_rule_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            rule: Rule,
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
                let rule = rule;
                let _: () = { drop((ctx, rule)) };
            })
        }
        /// Dispatched when an auto moderation rule was triggered and an action was executed.
        ///
        /// Provides said action execution's data.
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
        fn auto_moderation_action_execution<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            execution: ActionExecution,
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
                let execution = execution;
                let _: () = { drop((ctx, execution)) };
            })
        }
        /// Dispatched when the cache has received and inserted all data from guilds.
        ///
        /// This process happens upon starting your bot and should be fairly quick. However, cache
        /// actions performed prior this event may fail as the data could be not inserted yet.
        ///
        /// Provides the cached guilds' ids.
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
        fn cache_ready<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guilds: Vec<GuildId>,
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
                let guilds = guilds;
                let _: () = { drop((ctx, guilds)) };
            })
        }
        /// Dispatched when every shard has received a Ready event
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
        fn shards_ready<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            total_shards: u32,
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
                let total_shards = total_shards;
                let _: () = { drop((ctx, total_shards)) };
            })
        }
        /// Dispatched when a channel is created.
        ///
        /// Provides said channel's data.
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
        fn channel_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            channel: GuildChannel,
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
                let channel = channel;
                let _: () = { drop((ctx, channel)) };
            })
        }
        /// Dispatched when a category is created.
        ///
        /// Provides said category's data.
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
        fn category_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            category: GuildChannel,
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
                let category = category;
                let _: () = { drop((ctx, category)) };
            })
        }
        /// Dispatched when a category is deleted.
        ///
        /// Provides said category's data.
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
        fn category_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            category: GuildChannel,
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
                let category = category;
                let _: () = { drop((ctx, category)) };
            })
        }
        /// Dispatched when a channel is deleted.
        ///
        /// Provides said channel's data.
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
        fn channel_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            channel: GuildChannel,
            messages: Option<Vec<Message>>,
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
                let channel = channel;
                let messages = messages;
                let _: () = { drop((ctx, channel, messages)) };
            })
        }
        /// Dispatched when a pin is added, deleted.
        ///
        /// Provides said pin's data.
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
        fn channel_pins_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            pin: ChannelPinsUpdateEvent,
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
                let pin = pin;
                let _: () = { drop((ctx, pin)) };
            })
        }
        /// Dispatched when a channel is updated.
        ///
        /// The old channel data is only provided when the cache feature is enabled.
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
        fn channel_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old: Option<GuildChannel>,
            new: GuildChannel,
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
                let old = old;
                let new = new;
                let _: () = { drop((ctx, old, new)) };
            })
        }
        /// Dispatched when a new audit log entry is created.
        ///
        /// Provides said entry's data and the id of the guild where it was created.
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
        fn guild_audit_log_entry_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            entry: AuditLogEntry,
            guild_id: GuildId,
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
                let entry = entry;
                let guild_id = guild_id;
                let _: () = { drop((ctx, entry, guild_id)) };
            })
        }
        /// Dispatched when a user is banned from a guild.
        ///
        /// Provides the guild's id and the banned user's data.
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
        fn guild_ban_addition<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild_id: GuildId,
            banned_user: User,
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
                let guild_id = guild_id;
                let banned_user = banned_user;
                let _: () = { drop((ctx, guild_id, banned_user)) };
            })
        }
        /// Dispatched when a user's ban is lifted from a guild.
        ///
        /// Provides the guild's id and the lifted user's data.
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
        fn guild_ban_removal<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild_id: GuildId,
            unbanned_user: User,
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
                let guild_id = guild_id;
                let unbanned_user = unbanned_user;
                let _: () = { drop((ctx, guild_id, unbanned_user)) };
            })
        }
        /// Dispatched when a guild is created; or an existing guild's data is sent to us.
        ///
        /// Provides the guild's data and whether the guild is new (only when cache feature is enabled).
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
        fn guild_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild: Guild,
            is_new: Option<bool>,
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
                let guild = guild;
                let is_new = is_new;
                let _: () = { drop((ctx, guild, is_new)) };
            })
        }
        /// Dispatched when a guild is deleted.
        ///
        /// Provides the partial data of the guild sent by discord, and the full data from the cache,
        /// if cache feature is enabled and the data is available.
        ///
        /// The [`unavailable`] flag in the partial data determines the status of the guild. If the
        /// flag is false, the bot was removed from the guild, either by being kicked or banned. If the
        /// flag is true, the guild went offline.
        ///
        /// [`unavailable`]: UnavailableGuild::unavailable
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
        fn guild_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            incomplete: UnavailableGuild,
            full: Option<Guild>,
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
                let incomplete = incomplete;
                let full = full;
                let _: () = { drop((ctx, incomplete, full)) };
            })
        }
        /// Dispatched when the emojis are updated.
        ///
        /// Provides the guild's id and the new state of the emojis in the guild.
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
        fn guild_emojis_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild_id: GuildId,
            current_state: HashMap<EmojiId, Emoji>,
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
                let guild_id = guild_id;
                let current_state = current_state;
                let _: () = { drop((ctx, guild_id, current_state)) };
            })
        }
        /// Dispatched when a guild's integration is added, updated or removed.
        ///
        /// Provides the guild's id.
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
        fn guild_integrations_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild_id: GuildId,
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
                let guild_id = guild_id;
                let _: () = { drop((ctx, guild_id)) };
            })
        }
        /// Dispatched when a user joins a guild.
        ///
        /// Provides the guild's id and the user's member data.
        ///
        /// Note: This event will not trigger unless the "guild members" privileged intent is enabled
        /// on the bot application page.
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
        fn guild_member_addition<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            new_member: Member,
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
                let new_member = new_member;
                let _: () = { drop((ctx, new_member)) };
            })
        }
        /// Dispatched when a user's membership ends by leaving, getting kicked, or being banned.
        ///
        /// Provides the guild's id, the user's data, and the user's member data if cache feature is
        /// enabled and the data is available.
        ///
        /// Note: This event will not trigger unless the "guild members" privileged intent is enabled
        /// on the bot application page.
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
        fn guild_member_removal<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild_id: GuildId,
            user: User,
            member_data_if_available: Option<Member>,
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
                let guild_id = guild_id;
                let user = user;
                let member_data_if_available = member_data_if_available;
                let _: () = { drop((ctx, guild_id, user, member_data_if_available)) };
            })
        }
        /// Dispatched when a member is updated (e.g their nickname is updated).
        ///
        /// Provides the member's old and new data (if cache feature is enabled and data is available)
        /// and the new raw data about updated fields.
        ///
        /// Note: This event will not trigger unless the "guild members" privileged intent is enabled
        /// on the bot application page.
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
        fn guild_member_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old_if_available: Option<Member>,
            new: Option<Member>,
            event: GuildMemberUpdateEvent,
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
                let old_if_available = old_if_available;
                let new = new;
                let event = event;
                let _: () = { drop((ctx, old_if_available, new, event)) };
            })
        }
        /// Dispatched when the data for offline members was requested.
        ///
        /// Provides the guild's id and the data.
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
        fn guild_members_chunk<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            chunk: GuildMembersChunkEvent,
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
                let chunk = chunk;
                let _: () = { drop((ctx, chunk)) };
            })
        }
        /// Dispatched when a role is created.
        ///
        /// Provides the guild's id and the new role's data.
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
        fn guild_role_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            new: Role,
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
                let new = new;
                let _: () = { drop((ctx, new)) };
            })
        }
        /// Dispatched when a role is deleted.
        ///
        /// Provides the guild's id, the role's id and its data (if cache feature is enabled and the
        /// data is available).
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
        fn guild_role_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild_id: GuildId,
            removed_role_id: RoleId,
            removed_role_data_if_available: Option<Role>,
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
                let guild_id = guild_id;
                let removed_role_id = removed_role_id;
                let removed_role_data_if_available = removed_role_data_if_available;
                let _: () = {
                    drop((
                        ctx,
                        guild_id,
                        removed_role_id,
                        removed_role_data_if_available,
                    ))
                };
            })
        }
        /// Dispatched when a role is updated.
        ///
        /// Provides the guild's id, the role's old (if cache feature is enabled and the data is
        /// available) and new data.
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
        fn guild_role_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old_data_if_available: Option<Role>,
            new: Role,
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
                let old_data_if_available = old_data_if_available;
                let new = new;
                let _: () = { drop((ctx, old_data_if_available, new)) };
            })
        }
        /// Dispatched when the stickers are updated.
        ///
        /// Provides the guild's id and the new state of the stickers in the guild.
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
        fn guild_stickers_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild_id: GuildId,
            current_state: HashMap<StickerId, Sticker>,
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
                let guild_id = guild_id;
                let current_state = current_state;
                let _: () = { drop((ctx, guild_id, current_state)) };
            })
        }
        /// Dispatched when the guild is updated.
        ///
        /// Provides the guild's old data (if cache feature is enabled and the data is available)
        /// and the new data.
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
        fn guild_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old_data_if_available: Option<Guild>,
            new_data: PartialGuild,
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
                let old_data_if_available = old_data_if_available;
                let new_data = new_data;
                let _: () = { drop((ctx, old_data_if_available, new_data)) };
            })
        }
        /// Dispatched when a invite is created.
        ///
        /// Provides data about the invite.
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
        fn invite_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            data: InviteCreateEvent,
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
                let data = data;
                let _: () = { drop((ctx, data)) };
            })
        }
        /// Dispatched when a invite is deleted.
        ///
        /// Provides data about the invite.
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
        fn invite_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            data: InviteDeleteEvent,
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
                let data = data;
                let _: () = { drop((ctx, data)) };
            })
        }
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
        /// Dispatched when a message is deleted.
        ///
        /// Provides the guild's id, the channel's id and the message's id.
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
        fn message_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            channel_id: ChannelId,
            deleted_message_id: MessageId,
            guild_id: Option<GuildId>,
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
                let channel_id = channel_id;
                let deleted_message_id = deleted_message_id;
                let guild_id = guild_id;
                let _: () = { drop((ctx, channel_id, deleted_message_id, guild_id)) };
            })
        }
        /// Dispatched when multiple messages were deleted at once.
        ///
        /// Provides the guild's id, channel's id and the deleted messages' ids.
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
        fn message_delete_bulk<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            channel_id: ChannelId,
            multiple_deleted_messages_ids: Vec<MessageId>,
            guild_id: Option<GuildId>,
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
                let channel_id = channel_id;
                let multiple_deleted_messages_ids = multiple_deleted_messages_ids;
                let guild_id = guild_id;
                let _: () = { drop((ctx, channel_id, multiple_deleted_messages_ids, guild_id)) };
            })
        }
        /// Dispatched when a message is updated.
        ///
        /// Provides the message update data, as well as the actual old and new message if cache
        /// feature is enabled and the data is available.
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
        fn message_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old_if_available: Option<Message>,
            new: Option<Message>,
            event: MessageUpdateEvent,
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
                let old_if_available = old_if_available;
                let new = new;
                let event = event;
                let _: () = { drop((ctx, old_if_available, new, event)) };
            })
        }
        /// Dispatched when a new reaction is attached to a message.
        ///
        /// Provides the reaction's data.
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
        fn reaction_add<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            add_reaction: Reaction,
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
                let add_reaction = add_reaction;
                let _: () = { drop((ctx, add_reaction)) };
            })
        }
        /// Dispatched when a reaction is detached from a message.
        ///
        /// Provides the reaction's data.
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
        fn reaction_remove<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            removed_reaction: Reaction,
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
                let removed_reaction = removed_reaction;
                let _: () = { drop((ctx, removed_reaction)) };
            })
        }
        /// Dispatched when all reactions of a message are detached from a message.
        ///
        /// Provides the channel's id and the message's id.
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
        fn reaction_remove_all<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            channel_id: ChannelId,
            removed_from_message_id: MessageId,
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
                let channel_id = channel_id;
                let removed_from_message_id = removed_from_message_id;
                let _: () = { drop((ctx, channel_id, removed_from_message_id)) };
            })
        }
        /// Dispatched when all reactions of a message are detached from a message.
        ///
        /// Provides the channel's id and the message's id.
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
        fn reaction_remove_emoji<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            removed_reactions: Reaction,
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
                let removed_reactions = removed_reactions;
                let _: () = { drop((ctx, removed_reactions)) };
            })
        }
        #[deprecated = "This event does not exist"]
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
        fn presence_replace<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            presences: Vec<Presence>,
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
                let presences = presences;
                let _: () = { drop((ctx, presences)) };
            })
        }
        /// Dispatched when a user's presence is updated (e.g off -> on).
        ///
        /// Provides the presence's new data.
        ///
        /// Note: This event will not trigger unless the "guild presences" privileged intent is enabled
        /// on the bot application page.
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
        fn presence_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            new_data: Presence,
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
                let new_data = new_data;
                let _: () = { drop((ctx, new_data)) };
            })
        }
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
        /// Dispatched upon reconnection.
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
        fn resume<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: ResumedEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a shard's connection stage is updated
        ///
        /// Provides the context of the shard and the event information about the update.
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
        fn shard_stage_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: ShardStageUpdateEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when the data for soundboard sounds is requested.
        ///
        /// Provides the guild's id and the data.
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
        fn soundboard_sounds<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: SoundboardSoundsEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a soundboard sound is created.
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
        fn soundboard_sound_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: SoundboardSoundCreateEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a soundboard sound is updated.
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
        fn soundboard_sound_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: SoundboardSoundUpdateEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when multiple soundboard sounds at once are updated.
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
        fn soundboard_sounds_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: SoundboardSoundsUpdateEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a soundboard sound is deleted.
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
        fn soundboard_sound_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: SoundboardSoundDeleteEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a user starts typing.
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
        fn typing_start<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: TypingStartEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when the bot's data is updated.
        ///
        /// Provides the old (if cache feature is enabled and the data is available) and new data.
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
        fn user_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old_data: Option<CurrentUser>,
            new: CurrentUser,
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
                let old_data = old_data;
                let new = new;
                let _: () = { drop((ctx, old_data, new)) };
            })
        }
        /// Dispatched when a guild's voice server was updated (or changed to another one).
        ///
        /// Provides the voice server's data.
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
        fn voice_server_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: VoiceServerUpdateEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a user joins, leaves or moves to a voice channel.
        ///
        /// Provides the guild's id (if available) and the old state (if cache feature is enabled and
        /// [`GatewayIntents::GUILDS`] is enabled) and the new state of the guild's voice channels.
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
        fn voice_state_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old: Option<VoiceState>,
            new: VoiceState,
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
                let old = old;
                let new = new;
                let _: () = { drop((ctx, old, new)) };
            })
        }
        /// Dispatched when a voice channel's status is updated.
        ///
        /// Provides the status, channel's id and the guild's id.
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
        fn voice_channel_status_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old: Option<String>,
            status: Option<String>,
            id: ChannelId,
            guild_id: GuildId,
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
                let old = old;
                let status = status;
                let id = id;
                let guild_id = guild_id;
                let _: () = { drop((ctx, old, status, id, guild_id)) };
            })
        }
        /// Dispatched when a guild's webhook is updated.
        ///
        /// Provides the guild's id and the channel's id the webhook belongs in.
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
        fn webhook_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            guild_id: GuildId,
            belongs_to_channel_id: ChannelId,
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
                let guild_id = guild_id;
                let belongs_to_channel_id = belongs_to_channel_id;
                let _: () = { drop((ctx, guild_id, belongs_to_channel_id)) };
            })
        }
        /// Dispatched when an interaction is created (e.g a slash command was used or a button was clicked).
        ///
        /// Provides the created interaction.
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
        fn interaction_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            interaction: Interaction,
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
                let interaction = interaction;
                let _: () = { drop((ctx, interaction)) };
            })
        }
        /// Dispatched when a guild integration is created.
        ///
        /// Provides the created integration.
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
        fn integration_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            integration: Integration,
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
                let integration = integration;
                let _: () = { drop((ctx, integration)) };
            })
        }
        /// Dispatched when a guild integration is updated.
        ///
        /// Provides the updated integration.
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
        fn integration_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            integration: Integration,
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
                let integration = integration;
                let _: () = { drop((ctx, integration)) };
            })
        }
        /// Dispatched when a guild integration is deleted.
        ///
        /// Provides the integration's id, the id of the guild it belongs to, and its associated application id
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
        fn integration_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            integration_id: IntegrationId,
            guild_id: GuildId,
            application_id: Option<ApplicationId>,
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
                let integration_id = integration_id;
                let guild_id = guild_id;
                let application_id = application_id;
                let _: () = { drop((ctx, integration_id, guild_id, application_id)) };
            })
        }
        /// Dispatched when a stage instance is created.
        ///
        /// Provides the created stage instance.
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
        fn stage_instance_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            stage_instance: StageInstance,
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
                let stage_instance = stage_instance;
                let _: () = { drop((ctx, stage_instance)) };
            })
        }
        /// Dispatched when a stage instance is updated.
        ///
        /// Provides the updated stage instance.
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
        fn stage_instance_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            stage_instance: StageInstance,
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
                let stage_instance = stage_instance;
                let _: () = { drop((ctx, stage_instance)) };
            })
        }
        /// Dispatched when a stage instance is deleted.
        ///
        /// Provides the deleted stage instance.
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
        fn stage_instance_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            stage_instance: StageInstance,
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
                let stage_instance = stage_instance;
                let _: () = { drop((ctx, stage_instance)) };
            })
        }
        /// Dispatched when a thread is created or the current user is added to a private thread.
        ///
        /// Provides the thread.
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
        fn thread_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            thread: GuildChannel,
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
                let thread = thread;
                let _: () = { drop((ctx, thread)) };
            })
        }
        /// Dispatched when a thread is updated.
        ///
        /// Provides the updated thread and the old thread data, provided the thread was cached prior to dispatch.
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
        fn thread_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            old: Option<GuildChannel>,
            new: GuildChannel,
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
                let old = old;
                let new = new;
                let _: () = { drop((ctx, old, new)) };
            })
        }
        /// Dispatched when a thread is deleted.
        ///
        /// Provides the partial data about the deleted thread and, if it was present in the cache
        /// before its deletion, its full data.
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
        fn thread_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            thread: PartialGuildChannel,
            full_thread_data: Option<GuildChannel>,
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
                let thread = thread;
                let full_thread_data = full_thread_data;
                let _: () = { drop((ctx, thread, full_thread_data)) };
            })
        }
        /// Dispatched when the current user gains access to a channel.
        ///
        /// Provides the threads the current user can access, the thread members, the guild Id, and the
        /// channel Ids of the parent channels being synced.
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
        fn thread_list_sync<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            thread_list_sync: ThreadListSyncEvent,
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
                let thread_list_sync = thread_list_sync;
                let _: () = { drop((ctx, thread_list_sync)) };
            })
        }
        /// Dispatched when the [`ThreadMember`] for the current user is updated.
        ///
        /// Provides the updated thread member.
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
        fn thread_member_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            thread_member: ThreadMember,
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
                let thread_member = thread_member;
                let _: () = { drop((ctx, thread_member)) };
            })
        }
        /// Dispatched when anyone is added to or removed from a thread. If the current user does not
        /// have the [`GatewayIntents::GUILDS`], then this event will only be sent if the current user
        /// was added to or removed from the thread.
        ///
        /// Provides the added/removed members, the approximate member count of members in the thread,
        /// the thread Id and its guild Id.
        ///
        /// [`GatewayIntents::GUILDS`]: crate::model::gateway::GatewayIntents::GUILDS
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
        fn thread_members_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            thread_members_update: ThreadMembersUpdateEvent,
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
                let thread_members_update = thread_members_update;
                let _: () = { drop((ctx, thread_members_update)) };
            })
        }
        /// Dispatched when a scheduled event is created.
        ///
        /// Provides data about the scheduled event.
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
        fn guild_scheduled_event_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: ScheduledEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a scheduled event is updated.
        ///
        /// Provides data about the scheduled event.
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
        fn guild_scheduled_event_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: ScheduledEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a scheduled event is deleted.
        ///
        /// Provides data about the scheduled event.
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
        fn guild_scheduled_event_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: ScheduledEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a guild member has subscribed to a scheduled event.
        ///
        /// Provides data about the subscription.
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
        fn guild_scheduled_event_user_add<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            subscribed: GuildScheduledEventUserAddEvent,
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
                let subscribed = subscribed;
                let _: () = { drop((ctx, subscribed)) };
            })
        }
        /// Dispatched when a guild member has unsubscribed from a scheduled event.
        ///
        /// Provides data about the cancelled subscription.
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
        fn guild_scheduled_event_user_remove<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            unsubscribed: GuildScheduledEventUserRemoveEvent,
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
                let unsubscribed = unsubscribed;
                let _: () = { drop((ctx, unsubscribed)) };
            })
        }
        /// Dispatched when a user subscribes to a SKU.
        ///
        /// Provides data about the subscription.
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
        fn entitlement_create<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            entitlement: Entitlement,
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
                let entitlement = entitlement;
                let _: () = { drop((ctx, entitlement)) };
            })
        }
        /// Dispatched when a user's entitlement has been updated, such as when a subscription is
        /// renewed for the next billing period.
        ///
        /// Provides data abut the updated subscription. If the entitlement is renewed, the
        /// [`Entitlement::ends_at`] field will have changed.
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
        fn entitlement_update<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            entitlement: Entitlement,
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
                let entitlement = entitlement;
                let _: () = { drop((ctx, entitlement)) };
            })
        }
        /// Dispatched when a user's entitlement has been deleted. This happens rarely, but can occur
        /// if a subscription is refunded or otherwise deleted by Discord. Entitlements are not deleted
        /// when they expire.
        ///
        /// Provides data about the subscription. Specifically, the [`Entitlement::deleted`] field will
        /// be set.
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
        fn entitlement_delete<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            entitlement: Entitlement,
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
                let entitlement = entitlement;
                let _: () = { drop((ctx, entitlement)) };
            })
        }
        /// Dispatched when a user votes on a message poll.
        ///
        /// This will be dispatched multiple times if multiple answers are selected.
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
        fn poll_vote_add<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: MessagePollVoteAddEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when a user removes a previous vote on a poll.
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
        fn poll_vote_remove<'life0, 'async_trait>(
            &'life0 self,
            ctx: Context,
            event: MessagePollVoteRemoveEvent,
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
                let event = event;
                let _: () = { drop((ctx, event)) };
            })
        }
        /// Dispatched when an HTTP rate limit is hit
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
        fn ratelimit<'life0, 'async_trait>(
            &'life0 self,
            data: RatelimitInfo,
        ) -> ::core::pin::Pin<
            Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                let __self = self;
                let data = data;
                let _: () = { drop((data,)) };
            })
        }
    }
    /// This enum stores every possible event that an [`EventHandler`] can receive.
    #[non_exhaustive]
    #[allow(clippy::large_enum_variant)]
    pub enum FullEvent {
        /// Dispatched when the permissions of an application command was updated.
        ///
        /// Provides said permission's data.
        CommandPermissionsUpdate { permission: CommandPermissions },
        /// Dispatched when an auto moderation rule was created.
        ///
        /// Provides said rule's data.
        AutoModRuleCreate { rule: Rule },
        /// Dispatched when an auto moderation rule was updated.
        ///
        /// Provides said rule's data.
        AutoModRuleUpdate { rule: Rule },
        /// Dispatched when an auto moderation rule was deleted.
        ///
        /// Provides said rule's data.
        AutoModRuleDelete { rule: Rule },
        /// Dispatched when an auto moderation rule was triggered and an action was executed.
        ///
        /// Provides said action execution's data.
        AutoModActionExecution { execution: ActionExecution },
        /// Dispatched when the cache has received and inserted all data from guilds.
        ///
        /// This process happens upon starting your bot and should be fairly quick. However, cache
        /// actions performed prior this event may fail as the data could be not inserted yet.
        ///
        /// Provides the cached guilds' ids.
        CacheReady { guilds: Vec<GuildId> },
        /// Dispatched when every shard has received a Ready event
        ShardsReady { total_shards: u32 },
        /// Dispatched when a channel is created.
        ///
        /// Provides said channel's data.
        ChannelCreate { channel: GuildChannel },
        /// Dispatched when a category is created.
        ///
        /// Provides said category's data.
        CategoryCreate { category: GuildChannel },
        /// Dispatched when a category is deleted.
        ///
        /// Provides said category's data.
        CategoryDelete { category: GuildChannel },
        /// Dispatched when a channel is deleted.
        ///
        /// Provides said channel's data.
        ChannelDelete {
            channel: GuildChannel,
            messages: Option<Vec<Message>>,
        },
        /// Dispatched when a pin is added, deleted.
        ///
        /// Provides said pin's data.
        ChannelPinsUpdate { pin: ChannelPinsUpdateEvent },
        /// Dispatched when a channel is updated.
        ///
        /// The old channel data is only provided when the cache feature is enabled.
        ChannelUpdate {
            old: Option<GuildChannel>,
            new: GuildChannel,
        },
        /// Dispatched when a new audit log entry is created.
        ///
        /// Provides said entry's data and the id of the guild where it was created.
        GuildAuditLogEntryCreate {
            entry: AuditLogEntry,
            guild_id: GuildId,
        },
        /// Dispatched when a user is banned from a guild.
        ///
        /// Provides the guild's id and the banned user's data.
        GuildBanAddition {
            guild_id: GuildId,
            banned_user: User,
        },
        /// Dispatched when a user's ban is lifted from a guild.
        ///
        /// Provides the guild's id and the lifted user's data.
        GuildBanRemoval {
            guild_id: GuildId,
            unbanned_user: User,
        },
        /// Dispatched when a guild is created; or an existing guild's data is sent to us.
        ///
        /// Provides the guild's data and whether the guild is new (only when cache feature is enabled).
        GuildCreate { guild: Guild, is_new: Option<bool> },
        /// Dispatched when a guild is deleted.
        ///
        /// Provides the partial data of the guild sent by discord, and the full data from the cache,
        /// if cache feature is enabled and the data is available.
        ///
        /// The [`unavailable`] flag in the partial data determines the status of the guild. If the
        /// flag is false, the bot was removed from the guild, either by being kicked or banned. If the
        /// flag is true, the guild went offline.
        ///
        /// [`unavailable`]: UnavailableGuild::unavailable
        GuildDelete {
            incomplete: UnavailableGuild,
            full: Option<Guild>,
        },
        /// Dispatched when the emojis are updated.
        ///
        /// Provides the guild's id and the new state of the emojis in the guild.
        GuildEmojisUpdate {
            guild_id: GuildId,
            current_state: HashMap<EmojiId, Emoji>,
        },
        /// Dispatched when a guild's integration is added, updated or removed.
        ///
        /// Provides the guild's id.
        GuildIntegrationsUpdate { guild_id: GuildId },
        /// Dispatched when a user joins a guild.
        ///
        /// Provides the guild's id and the user's member data.
        ///
        /// Note: This event will not trigger unless the "guild members" privileged intent is enabled
        /// on the bot application page.
        GuildMemberAddition { new_member: Member },
        /// Dispatched when a user's membership ends by leaving, getting kicked, or being banned.
        ///
        /// Provides the guild's id, the user's data, and the user's member data if cache feature is
        /// enabled and the data is available.
        ///
        /// Note: This event will not trigger unless the "guild members" privileged intent is enabled
        /// on the bot application page.
        GuildMemberRemoval {
            guild_id: GuildId,
            user: User,
            member_data_if_available: Option<Member>,
        },
        /// Dispatched when a member is updated (e.g their nickname is updated).
        ///
        /// Provides the member's old and new data (if cache feature is enabled and data is available)
        /// and the new raw data about updated fields.
        ///
        /// Note: This event will not trigger unless the "guild members" privileged intent is enabled
        /// on the bot application page.
        GuildMemberUpdate {
            old_if_available: Option<Member>,
            new: Option<Member>,
            event: GuildMemberUpdateEvent,
        },
        /// Dispatched when the data for offline members was requested.
        ///
        /// Provides the guild's id and the data.
        GuildMembersChunk { chunk: GuildMembersChunkEvent },
        /// Dispatched when a role is created.
        ///
        /// Provides the guild's id and the new role's data.
        GuildRoleCreate { new: Role },
        /// Dispatched when a role is deleted.
        ///
        /// Provides the guild's id, the role's id and its data (if cache feature is enabled and the
        /// data is available).
        GuildRoleDelete {
            guild_id: GuildId,
            removed_role_id: RoleId,
            removed_role_data_if_available: Option<Role>,
        },
        /// Dispatched when a role is updated.
        ///
        /// Provides the guild's id, the role's old (if cache feature is enabled and the data is
        /// available) and new data.
        GuildRoleUpdate {
            old_data_if_available: Option<Role>,
            new: Role,
        },
        /// Dispatched when the stickers are updated.
        ///
        /// Provides the guild's id and the new state of the stickers in the guild.
        GuildStickersUpdate {
            guild_id: GuildId,
            current_state: HashMap<StickerId, Sticker>,
        },
        /// Dispatched when the guild is updated.
        ///
        /// Provides the guild's old data (if cache feature is enabled and the data is available)
        /// and the new data.
        GuildUpdate {
            old_data_if_available: Option<Guild>,
            new_data: PartialGuild,
        },
        /// Dispatched when a invite is created.
        ///
        /// Provides data about the invite.
        InviteCreate { data: InviteCreateEvent },
        /// Dispatched when a invite is deleted.
        ///
        /// Provides data about the invite.
        InviteDelete { data: InviteDeleteEvent },
        /// Dispatched when a message is created.
        ///
        /// Provides the message's data.
        Message { new_message: Message },
        /// Dispatched when a message is deleted.
        ///
        /// Provides the guild's id, the channel's id and the message's id.
        MessageDelete {
            channel_id: ChannelId,
            deleted_message_id: MessageId,
            guild_id: Option<GuildId>,
        },
        /// Dispatched when multiple messages were deleted at once.
        ///
        /// Provides the guild's id, channel's id and the deleted messages' ids.
        MessageDeleteBulk {
            channel_id: ChannelId,
            multiple_deleted_messages_ids: Vec<MessageId>,
            guild_id: Option<GuildId>,
        },
        /// Dispatched when a message is updated.
        ///
        /// Provides the message update data, as well as the actual old and new message if cache
        /// feature is enabled and the data is available.
        MessageUpdate {
            old_if_available: Option<Message>,
            new: Option<Message>,
            event: MessageUpdateEvent,
        },
        /// Dispatched when a new reaction is attached to a message.
        ///
        /// Provides the reaction's data.
        ReactionAdd { add_reaction: Reaction },
        /// Dispatched when a reaction is detached from a message.
        ///
        /// Provides the reaction's data.
        ReactionRemove { removed_reaction: Reaction },
        /// Dispatched when all reactions of a message are detached from a message.
        ///
        /// Provides the channel's id and the message's id.
        ReactionRemoveAll {
            channel_id: ChannelId,
            removed_from_message_id: MessageId,
        },
        /// Dispatched when all reactions of a message are detached from a message.
        ///
        /// Provides the channel's id and the message's id.
        ReactionRemoveEmoji { removed_reactions: Reaction },
        #[deprecated = "This event does not exist"]
        PresenceReplace { presences: Vec<Presence> },
        /// Dispatched when a user's presence is updated (e.g off -> on).
        ///
        /// Provides the presence's new data.
        ///
        /// Note: This event will not trigger unless the "guild presences" privileged intent is enabled
        /// on the bot application page.
        PresenceUpdate { new_data: Presence },
        /// Dispatched upon startup.
        ///
        /// Provides data about the bot and the guilds it's in.
        Ready { data_about_bot: Ready },
        /// Dispatched upon reconnection.
        Resume { event: ResumedEvent },
        /// Dispatched when a shard's connection stage is updated
        ///
        /// Provides the context of the shard and the event information about the update.
        ShardStageUpdate { event: ShardStageUpdateEvent },
        /// Dispatched when the data for soundboard sounds is requested.
        ///
        /// Provides the guild's id and the data.
        SoundboardSounds { event: SoundboardSoundsEvent },
        /// Dispatched when a soundboard sound is created.
        SoundboardSoundCreate { event: SoundboardSoundCreateEvent },
        /// Dispatched when a soundboard sound is updated.
        SoundboardSoundUpdate { event: SoundboardSoundUpdateEvent },
        /// Dispatched when multiple soundboard sounds at once are updated.
        SoundboardSoundsUpdate { event: SoundboardSoundsUpdateEvent },
        /// Dispatched when a soundboard sound is deleted.
        SoundboardSoundDelete { event: SoundboardSoundDeleteEvent },
        /// Dispatched when a user starts typing.
        TypingStart { event: TypingStartEvent },
        /// Dispatched when the bot's data is updated.
        ///
        /// Provides the old (if cache feature is enabled and the data is available) and new data.
        UserUpdate {
            old_data: Option<CurrentUser>,
            new: CurrentUser,
        },
        /// Dispatched when a guild's voice server was updated (or changed to another one).
        ///
        /// Provides the voice server's data.
        VoiceServerUpdate { event: VoiceServerUpdateEvent },
        /// Dispatched when a user joins, leaves or moves to a voice channel.
        ///
        /// Provides the guild's id (if available) and the old state (if cache feature is enabled and
        /// [`GatewayIntents::GUILDS`] is enabled) and the new state of the guild's voice channels.
        VoiceStateUpdate {
            old: Option<VoiceState>,
            new: VoiceState,
        },
        /// Dispatched when a voice channel's status is updated.
        ///
        /// Provides the status, channel's id and the guild's id.
        VoiceChannelStatusUpdate {
            old: Option<String>,
            status: Option<String>,
            id: ChannelId,
            guild_id: GuildId,
        },
        /// Dispatched when a guild's webhook is updated.
        ///
        /// Provides the guild's id and the channel's id the webhook belongs in.
        WebhookUpdate {
            guild_id: GuildId,
            belongs_to_channel_id: ChannelId,
        },
        /// Dispatched when an interaction is created (e.g a slash command was used or a button was clicked).
        ///
        /// Provides the created interaction.
        InteractionCreate { interaction: Interaction },
        /// Dispatched when a guild integration is created.
        ///
        /// Provides the created integration.
        IntegrationCreate { integration: Integration },
        /// Dispatched when a guild integration is updated.
        ///
        /// Provides the updated integration.
        IntegrationUpdate { integration: Integration },
        /// Dispatched when a guild integration is deleted.
        ///
        /// Provides the integration's id, the id of the guild it belongs to, and its associated application id
        IntegrationDelete {
            integration_id: IntegrationId,
            guild_id: GuildId,
            application_id: Option<ApplicationId>,
        },
        /// Dispatched when a stage instance is created.
        ///
        /// Provides the created stage instance.
        StageInstanceCreate { stage_instance: StageInstance },
        /// Dispatched when a stage instance is updated.
        ///
        /// Provides the updated stage instance.
        StageInstanceUpdate { stage_instance: StageInstance },
        /// Dispatched when a stage instance is deleted.
        ///
        /// Provides the deleted stage instance.
        StageInstanceDelete { stage_instance: StageInstance },
        /// Dispatched when a thread is created or the current user is added to a private thread.
        ///
        /// Provides the thread.
        ThreadCreate { thread: GuildChannel },
        /// Dispatched when a thread is updated.
        ///
        /// Provides the updated thread and the old thread data, provided the thread was cached prior to dispatch.
        ThreadUpdate {
            old: Option<GuildChannel>,
            new: GuildChannel,
        },
        /// Dispatched when a thread is deleted.
        ///
        /// Provides the partial data about the deleted thread and, if it was present in the cache
        /// before its deletion, its full data.
        ThreadDelete {
            thread: PartialGuildChannel,
            full_thread_data: Option<GuildChannel>,
        },
        /// Dispatched when the current user gains access to a channel.
        ///
        /// Provides the threads the current user can access, the thread members, the guild Id, and the
        /// channel Ids of the parent channels being synced.
        ThreadListSync {
            thread_list_sync: ThreadListSyncEvent,
        },
        /// Dispatched when the [`ThreadMember`] for the current user is updated.
        ///
        /// Provides the updated thread member.
        ThreadMemberUpdate { thread_member: ThreadMember },
        /// Dispatched when anyone is added to or removed from a thread. If the current user does not
        /// have the [`GatewayIntents::GUILDS`], then this event will only be sent if the current user
        /// was added to or removed from the thread.
        ///
        /// Provides the added/removed members, the approximate member count of members in the thread,
        /// the thread Id and its guild Id.
        ///
        /// [`GatewayIntents::GUILDS`]: crate::model::gateway::GatewayIntents::GUILDS
        ThreadMembersUpdate {
            thread_members_update: ThreadMembersUpdateEvent,
        },
        /// Dispatched when a scheduled event is created.
        ///
        /// Provides data about the scheduled event.
        GuildScheduledEventCreate { event: ScheduledEvent },
        /// Dispatched when a scheduled event is updated.
        ///
        /// Provides data about the scheduled event.
        GuildScheduledEventUpdate { event: ScheduledEvent },
        /// Dispatched when a scheduled event is deleted.
        ///
        /// Provides data about the scheduled event.
        GuildScheduledEventDelete { event: ScheduledEvent },
        /// Dispatched when a guild member has subscribed to a scheduled event.
        ///
        /// Provides data about the subscription.
        GuildScheduledEventUserAdd {
            subscribed: GuildScheduledEventUserAddEvent,
        },
        /// Dispatched when a guild member has unsubscribed from a scheduled event.
        ///
        /// Provides data about the cancelled subscription.
        GuildScheduledEventUserRemove {
            unsubscribed: GuildScheduledEventUserRemoveEvent,
        },
        /// Dispatched when a user subscribes to a SKU.
        ///
        /// Provides data about the subscription.
        EntitlementCreate { entitlement: Entitlement },
        /// Dispatched when a user's entitlement has been updated, such as when a subscription is
        /// renewed for the next billing period.
        ///
        /// Provides data abut the updated subscription. If the entitlement is renewed, the
        /// [`Entitlement::ends_at`] field will have changed.
        EntitlementUpdate { entitlement: Entitlement },
        /// Dispatched when a user's entitlement has been deleted. This happens rarely, but can occur
        /// if a subscription is refunded or otherwise deleted by Discord. Entitlements are not deleted
        /// when they expire.
        ///
        /// Provides data about the subscription. Specifically, the [`Entitlement::deleted`] field will
        /// be set.
        EntitlementDelete { entitlement: Entitlement },
        /// Dispatched when a user votes on a message poll.
        ///
        /// This will be dispatched multiple times if multiple answers are selected.
        MessagePollVoteAdd { event: MessagePollVoteAddEvent },
        /// Dispatched when a user removes a previous vote on a poll.
        MessagePollVoteRemove { event: MessagePollVoteRemoveEvent },
        /// Dispatched when an HTTP rate limit is hit
        Ratelimit { data: RatelimitInfo },
    }
    #[automatically_derived]
    #[allow(clippy::large_enum_variant)]
    impl ::core::clone::Clone for FullEvent {
        #[inline]
        fn clone(&self) -> FullEvent {
            match self {
                FullEvent::CommandPermissionsUpdate {
                    permission: __self_0,
                } => FullEvent::CommandPermissionsUpdate {
                    permission: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::AutoModRuleCreate { rule: __self_0 } => FullEvent::AutoModRuleCreate {
                    rule: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::AutoModRuleUpdate { rule: __self_0 } => FullEvent::AutoModRuleUpdate {
                    rule: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::AutoModRuleDelete { rule: __self_0 } => FullEvent::AutoModRuleDelete {
                    rule: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::AutoModActionExecution {
                    execution: __self_0,
                } => FullEvent::AutoModActionExecution {
                    execution: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::CacheReady { guilds: __self_0 } => FullEvent::CacheReady {
                    guilds: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ShardsReady {
                    total_shards: __self_0,
                } => FullEvent::ShardsReady {
                    total_shards: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ChannelCreate { channel: __self_0 } => FullEvent::ChannelCreate {
                    channel: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::CategoryCreate { category: __self_0 } => FullEvent::CategoryCreate {
                    category: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::CategoryDelete { category: __self_0 } => FullEvent::CategoryDelete {
                    category: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ChannelDelete {
                    channel: __self_0,
                    messages: __self_1,
                } => FullEvent::ChannelDelete {
                    channel: ::core::clone::Clone::clone(__self_0),
                    messages: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::ChannelPinsUpdate { pin: __self_0 } => FullEvent::ChannelPinsUpdate {
                    pin: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ChannelUpdate {
                    old: __self_0,
                    new: __self_1,
                } => FullEvent::ChannelUpdate {
                    old: ::core::clone::Clone::clone(__self_0),
                    new: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildAuditLogEntryCreate {
                    entry: __self_0,
                    guild_id: __self_1,
                } => FullEvent::GuildAuditLogEntryCreate {
                    entry: ::core::clone::Clone::clone(__self_0),
                    guild_id: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildBanAddition {
                    guild_id: __self_0,
                    banned_user: __self_1,
                } => FullEvent::GuildBanAddition {
                    guild_id: ::core::clone::Clone::clone(__self_0),
                    banned_user: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildBanRemoval {
                    guild_id: __self_0,
                    unbanned_user: __self_1,
                } => FullEvent::GuildBanRemoval {
                    guild_id: ::core::clone::Clone::clone(__self_0),
                    unbanned_user: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildCreate {
                    guild: __self_0,
                    is_new: __self_1,
                } => FullEvent::GuildCreate {
                    guild: ::core::clone::Clone::clone(__self_0),
                    is_new: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildDelete {
                    incomplete: __self_0,
                    full: __self_1,
                } => FullEvent::GuildDelete {
                    incomplete: ::core::clone::Clone::clone(__self_0),
                    full: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildEmojisUpdate {
                    guild_id: __self_0,
                    current_state: __self_1,
                } => FullEvent::GuildEmojisUpdate {
                    guild_id: ::core::clone::Clone::clone(__self_0),
                    current_state: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildIntegrationsUpdate { guild_id: __self_0 } => {
                    FullEvent::GuildIntegrationsUpdate {
                        guild_id: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::GuildMemberAddition {
                    new_member: __self_0,
                } => FullEvent::GuildMemberAddition {
                    new_member: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::GuildMemberRemoval {
                    guild_id: __self_0,
                    user: __self_1,
                    member_data_if_available: __self_2,
                } => FullEvent::GuildMemberRemoval {
                    guild_id: ::core::clone::Clone::clone(__self_0),
                    user: ::core::clone::Clone::clone(__self_1),
                    member_data_if_available: ::core::clone::Clone::clone(__self_2),
                },
                FullEvent::GuildMemberUpdate {
                    old_if_available: __self_0,
                    new: __self_1,
                    event: __self_2,
                } => FullEvent::GuildMemberUpdate {
                    old_if_available: ::core::clone::Clone::clone(__self_0),
                    new: ::core::clone::Clone::clone(__self_1),
                    event: ::core::clone::Clone::clone(__self_2),
                },
                FullEvent::GuildMembersChunk { chunk: __self_0 } => FullEvent::GuildMembersChunk {
                    chunk: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::GuildRoleCreate { new: __self_0 } => FullEvent::GuildRoleCreate {
                    new: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::GuildRoleDelete {
                    guild_id: __self_0,
                    removed_role_id: __self_1,
                    removed_role_data_if_available: __self_2,
                } => FullEvent::GuildRoleDelete {
                    guild_id: ::core::clone::Clone::clone(__self_0),
                    removed_role_id: ::core::clone::Clone::clone(__self_1),
                    removed_role_data_if_available: ::core::clone::Clone::clone(__self_2),
                },
                FullEvent::GuildRoleUpdate {
                    old_data_if_available: __self_0,
                    new: __self_1,
                } => FullEvent::GuildRoleUpdate {
                    old_data_if_available: ::core::clone::Clone::clone(__self_0),
                    new: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildStickersUpdate {
                    guild_id: __self_0,
                    current_state: __self_1,
                } => FullEvent::GuildStickersUpdate {
                    guild_id: ::core::clone::Clone::clone(__self_0),
                    current_state: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::GuildUpdate {
                    old_data_if_available: __self_0,
                    new_data: __self_1,
                } => FullEvent::GuildUpdate {
                    old_data_if_available: ::core::clone::Clone::clone(__self_0),
                    new_data: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::InviteCreate { data: __self_0 } => FullEvent::InviteCreate {
                    data: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::InviteDelete { data: __self_0 } => FullEvent::InviteDelete {
                    data: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::Message {
                    new_message: __self_0,
                } => FullEvent::Message {
                    new_message: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::MessageDelete {
                    channel_id: __self_0,
                    deleted_message_id: __self_1,
                    guild_id: __self_2,
                } => FullEvent::MessageDelete {
                    channel_id: ::core::clone::Clone::clone(__self_0),
                    deleted_message_id: ::core::clone::Clone::clone(__self_1),
                    guild_id: ::core::clone::Clone::clone(__self_2),
                },
                FullEvent::MessageDeleteBulk {
                    channel_id: __self_0,
                    multiple_deleted_messages_ids: __self_1,
                    guild_id: __self_2,
                } => FullEvent::MessageDeleteBulk {
                    channel_id: ::core::clone::Clone::clone(__self_0),
                    multiple_deleted_messages_ids: ::core::clone::Clone::clone(__self_1),
                    guild_id: ::core::clone::Clone::clone(__self_2),
                },
                FullEvent::MessageUpdate {
                    old_if_available: __self_0,
                    new: __self_1,
                    event: __self_2,
                } => FullEvent::MessageUpdate {
                    old_if_available: ::core::clone::Clone::clone(__self_0),
                    new: ::core::clone::Clone::clone(__self_1),
                    event: ::core::clone::Clone::clone(__self_2),
                },
                FullEvent::ReactionAdd {
                    add_reaction: __self_0,
                } => FullEvent::ReactionAdd {
                    add_reaction: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ReactionRemove {
                    removed_reaction: __self_0,
                } => FullEvent::ReactionRemove {
                    removed_reaction: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ReactionRemoveAll {
                    channel_id: __self_0,
                    removed_from_message_id: __self_1,
                } => FullEvent::ReactionRemoveAll {
                    channel_id: ::core::clone::Clone::clone(__self_0),
                    removed_from_message_id: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::ReactionRemoveEmoji {
                    removed_reactions: __self_0,
                } => FullEvent::ReactionRemoveEmoji {
                    removed_reactions: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::PresenceReplace {
                    presences: __self_0,
                } => FullEvent::PresenceReplace {
                    presences: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::PresenceUpdate { new_data: __self_0 } => FullEvent::PresenceUpdate {
                    new_data: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::Ready {
                    data_about_bot: __self_0,
                } => FullEvent::Ready {
                    data_about_bot: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::Resume { event: __self_0 } => FullEvent::Resume {
                    event: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ShardStageUpdate { event: __self_0 } => FullEvent::ShardStageUpdate {
                    event: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::SoundboardSounds { event: __self_0 } => FullEvent::SoundboardSounds {
                    event: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::SoundboardSoundCreate { event: __self_0 } => {
                    FullEvent::SoundboardSoundCreate {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::SoundboardSoundUpdate { event: __self_0 } => {
                    FullEvent::SoundboardSoundUpdate {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::SoundboardSoundsUpdate { event: __self_0 } => {
                    FullEvent::SoundboardSoundsUpdate {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::SoundboardSoundDelete { event: __self_0 } => {
                    FullEvent::SoundboardSoundDelete {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::TypingStart { event: __self_0 } => FullEvent::TypingStart {
                    event: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::UserUpdate {
                    old_data: __self_0,
                    new: __self_1,
                } => FullEvent::UserUpdate {
                    old_data: ::core::clone::Clone::clone(__self_0),
                    new: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::VoiceServerUpdate { event: __self_0 } => FullEvent::VoiceServerUpdate {
                    event: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::VoiceStateUpdate {
                    old: __self_0,
                    new: __self_1,
                } => FullEvent::VoiceStateUpdate {
                    old: ::core::clone::Clone::clone(__self_0),
                    new: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::VoiceChannelStatusUpdate {
                    old: __self_0,
                    status: __self_1,
                    id: __self_2,
                    guild_id: __self_3,
                } => FullEvent::VoiceChannelStatusUpdate {
                    old: ::core::clone::Clone::clone(__self_0),
                    status: ::core::clone::Clone::clone(__self_1),
                    id: ::core::clone::Clone::clone(__self_2),
                    guild_id: ::core::clone::Clone::clone(__self_3),
                },
                FullEvent::WebhookUpdate {
                    guild_id: __self_0,
                    belongs_to_channel_id: __self_1,
                } => FullEvent::WebhookUpdate {
                    guild_id: ::core::clone::Clone::clone(__self_0),
                    belongs_to_channel_id: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::InteractionCreate {
                    interaction: __self_0,
                } => FullEvent::InteractionCreate {
                    interaction: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::IntegrationCreate {
                    integration: __self_0,
                } => FullEvent::IntegrationCreate {
                    integration: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::IntegrationUpdate {
                    integration: __self_0,
                } => FullEvent::IntegrationUpdate {
                    integration: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::IntegrationDelete {
                    integration_id: __self_0,
                    guild_id: __self_1,
                    application_id: __self_2,
                } => FullEvent::IntegrationDelete {
                    integration_id: ::core::clone::Clone::clone(__self_0),
                    guild_id: ::core::clone::Clone::clone(__self_1),
                    application_id: ::core::clone::Clone::clone(__self_2),
                },
                FullEvent::StageInstanceCreate {
                    stage_instance: __self_0,
                } => FullEvent::StageInstanceCreate {
                    stage_instance: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::StageInstanceUpdate {
                    stage_instance: __self_0,
                } => FullEvent::StageInstanceUpdate {
                    stage_instance: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::StageInstanceDelete {
                    stage_instance: __self_0,
                } => FullEvent::StageInstanceDelete {
                    stage_instance: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ThreadCreate { thread: __self_0 } => FullEvent::ThreadCreate {
                    thread: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ThreadUpdate {
                    old: __self_0,
                    new: __self_1,
                } => FullEvent::ThreadUpdate {
                    old: ::core::clone::Clone::clone(__self_0),
                    new: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::ThreadDelete {
                    thread: __self_0,
                    full_thread_data: __self_1,
                } => FullEvent::ThreadDelete {
                    thread: ::core::clone::Clone::clone(__self_0),
                    full_thread_data: ::core::clone::Clone::clone(__self_1),
                },
                FullEvent::ThreadListSync {
                    thread_list_sync: __self_0,
                } => FullEvent::ThreadListSync {
                    thread_list_sync: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ThreadMemberUpdate {
                    thread_member: __self_0,
                } => FullEvent::ThreadMemberUpdate {
                    thread_member: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::ThreadMembersUpdate {
                    thread_members_update: __self_0,
                } => FullEvent::ThreadMembersUpdate {
                    thread_members_update: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::GuildScheduledEventCreate { event: __self_0 } => {
                    FullEvent::GuildScheduledEventCreate {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::GuildScheduledEventUpdate { event: __self_0 } => {
                    FullEvent::GuildScheduledEventUpdate {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::GuildScheduledEventDelete { event: __self_0 } => {
                    FullEvent::GuildScheduledEventDelete {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::GuildScheduledEventUserAdd {
                    subscribed: __self_0,
                } => FullEvent::GuildScheduledEventUserAdd {
                    subscribed: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::GuildScheduledEventUserRemove {
                    unsubscribed: __self_0,
                } => FullEvent::GuildScheduledEventUserRemove {
                    unsubscribed: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::EntitlementCreate {
                    entitlement: __self_0,
                } => FullEvent::EntitlementCreate {
                    entitlement: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::EntitlementUpdate {
                    entitlement: __self_0,
                } => FullEvent::EntitlementUpdate {
                    entitlement: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::EntitlementDelete {
                    entitlement: __self_0,
                } => FullEvent::EntitlementDelete {
                    entitlement: ::core::clone::Clone::clone(__self_0),
                },
                FullEvent::MessagePollVoteAdd { event: __self_0 } => {
                    FullEvent::MessagePollVoteAdd {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::MessagePollVoteRemove { event: __self_0 } => {
                    FullEvent::MessagePollVoteRemove {
                        event: ::core::clone::Clone::clone(__self_0),
                    }
                }
                FullEvent::Ratelimit { data: __self_0 } => FullEvent::Ratelimit {
                    data: ::core::clone::Clone::clone(__self_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(clippy::large_enum_variant)]
    impl ::core::fmt::Debug for FullEvent {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                FullEvent::CommandPermissionsUpdate {
                    permission: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CommandPermissionsUpdate",
                    "permission",
                    &__self_0,
                ),
                FullEvent::AutoModRuleCreate { rule: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "AutoModRuleCreate",
                        "rule",
                        &__self_0,
                    )
                }
                FullEvent::AutoModRuleUpdate { rule: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "AutoModRuleUpdate",
                        "rule",
                        &__self_0,
                    )
                }
                FullEvent::AutoModRuleDelete { rule: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "AutoModRuleDelete",
                        "rule",
                        &__self_0,
                    )
                }
                FullEvent::AutoModActionExecution {
                    execution: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AutoModActionExecution",
                    "execution",
                    &__self_0,
                ),
                FullEvent::CacheReady { guilds: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "CacheReady",
                        "guilds",
                        &__self_0,
                    )
                }
                FullEvent::ShardsReady {
                    total_shards: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ShardsReady",
                    "total_shards",
                    &__self_0,
                ),
                FullEvent::ChannelCreate { channel: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ChannelCreate",
                        "channel",
                        &__self_0,
                    )
                }
                FullEvent::CategoryCreate { category: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "CategoryCreate",
                        "category",
                        &__self_0,
                    )
                }
                FullEvent::CategoryDelete { category: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "CategoryDelete",
                        "category",
                        &__self_0,
                    )
                }
                FullEvent::ChannelDelete {
                    channel: __self_0,
                    messages: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ChannelDelete",
                    "channel",
                    __self_0,
                    "messages",
                    &__self_1,
                ),
                FullEvent::ChannelPinsUpdate { pin: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ChannelPinsUpdate",
                        "pin",
                        &__self_0,
                    )
                }
                FullEvent::ChannelUpdate {
                    old: __self_0,
                    new: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ChannelUpdate",
                    "old",
                    __self_0,
                    "new",
                    &__self_1,
                ),
                FullEvent::GuildAuditLogEntryCreate {
                    entry: __self_0,
                    guild_id: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildAuditLogEntryCreate",
                    "entry",
                    __self_0,
                    "guild_id",
                    &__self_1,
                ),
                FullEvent::GuildBanAddition {
                    guild_id: __self_0,
                    banned_user: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildBanAddition",
                    "guild_id",
                    __self_0,
                    "banned_user",
                    &__self_1,
                ),
                FullEvent::GuildBanRemoval {
                    guild_id: __self_0,
                    unbanned_user: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildBanRemoval",
                    "guild_id",
                    __self_0,
                    "unbanned_user",
                    &__self_1,
                ),
                FullEvent::GuildCreate {
                    guild: __self_0,
                    is_new: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildCreate",
                    "guild",
                    __self_0,
                    "is_new",
                    &__self_1,
                ),
                FullEvent::GuildDelete {
                    incomplete: __self_0,
                    full: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildDelete",
                    "incomplete",
                    __self_0,
                    "full",
                    &__self_1,
                ),
                FullEvent::GuildEmojisUpdate {
                    guild_id: __self_0,
                    current_state: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildEmojisUpdate",
                    "guild_id",
                    __self_0,
                    "current_state",
                    &__self_1,
                ),
                FullEvent::GuildIntegrationsUpdate { guild_id: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "GuildIntegrationsUpdate",
                        "guild_id",
                        &__self_0,
                    )
                }
                FullEvent::GuildMemberAddition {
                    new_member: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "GuildMemberAddition",
                    "new_member",
                    &__self_0,
                ),
                FullEvent::GuildMemberRemoval {
                    guild_id: __self_0,
                    user: __self_1,
                    member_data_if_available: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "GuildMemberRemoval",
                    "guild_id",
                    __self_0,
                    "user",
                    __self_1,
                    "member_data_if_available",
                    &__self_2,
                ),
                FullEvent::GuildMemberUpdate {
                    old_if_available: __self_0,
                    new: __self_1,
                    event: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "GuildMemberUpdate",
                    "old_if_available",
                    __self_0,
                    "new",
                    __self_1,
                    "event",
                    &__self_2,
                ),
                FullEvent::GuildMembersChunk { chunk: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "GuildMembersChunk",
                        "chunk",
                        &__self_0,
                    )
                }
                FullEvent::GuildRoleCreate { new: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "GuildRoleCreate",
                        "new",
                        &__self_0,
                    )
                }
                FullEvent::GuildRoleDelete {
                    guild_id: __self_0,
                    removed_role_id: __self_1,
                    removed_role_data_if_available: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "GuildRoleDelete",
                    "guild_id",
                    __self_0,
                    "removed_role_id",
                    __self_1,
                    "removed_role_data_if_available",
                    &__self_2,
                ),
                FullEvent::GuildRoleUpdate {
                    old_data_if_available: __self_0,
                    new: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildRoleUpdate",
                    "old_data_if_available",
                    __self_0,
                    "new",
                    &__self_1,
                ),
                FullEvent::GuildStickersUpdate {
                    guild_id: __self_0,
                    current_state: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildStickersUpdate",
                    "guild_id",
                    __self_0,
                    "current_state",
                    &__self_1,
                ),
                FullEvent::GuildUpdate {
                    old_data_if_available: __self_0,
                    new_data: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GuildUpdate",
                    "old_data_if_available",
                    __self_0,
                    "new_data",
                    &__self_1,
                ),
                FullEvent::InviteCreate { data: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InviteCreate",
                        "data",
                        &__self_0,
                    )
                }
                FullEvent::InviteDelete { data: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InviteDelete",
                        "data",
                        &__self_0,
                    )
                }
                FullEvent::Message {
                    new_message: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Message",
                    "new_message",
                    &__self_0,
                ),
                FullEvent::MessageDelete {
                    channel_id: __self_0,
                    deleted_message_id: __self_1,
                    guild_id: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "MessageDelete",
                    "channel_id",
                    __self_0,
                    "deleted_message_id",
                    __self_1,
                    "guild_id",
                    &__self_2,
                ),
                FullEvent::MessageDeleteBulk {
                    channel_id: __self_0,
                    multiple_deleted_messages_ids: __self_1,
                    guild_id: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "MessageDeleteBulk",
                    "channel_id",
                    __self_0,
                    "multiple_deleted_messages_ids",
                    __self_1,
                    "guild_id",
                    &__self_2,
                ),
                FullEvent::MessageUpdate {
                    old_if_available: __self_0,
                    new: __self_1,
                    event: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "MessageUpdate",
                    "old_if_available",
                    __self_0,
                    "new",
                    __self_1,
                    "event",
                    &__self_2,
                ),
                FullEvent::ReactionAdd {
                    add_reaction: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ReactionAdd",
                    "add_reaction",
                    &__self_0,
                ),
                FullEvent::ReactionRemove {
                    removed_reaction: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ReactionRemove",
                    "removed_reaction",
                    &__self_0,
                ),
                FullEvent::ReactionRemoveAll {
                    channel_id: __self_0,
                    removed_from_message_id: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ReactionRemoveAll",
                    "channel_id",
                    __self_0,
                    "removed_from_message_id",
                    &__self_1,
                ),
                FullEvent::ReactionRemoveEmoji {
                    removed_reactions: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ReactionRemoveEmoji",
                    "removed_reactions",
                    &__self_0,
                ),
                FullEvent::PresenceReplace {
                    presences: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "PresenceReplace",
                    "presences",
                    &__self_0,
                ),
                FullEvent::PresenceUpdate { new_data: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "PresenceUpdate",
                        "new_data",
                        &__self_0,
                    )
                }
                FullEvent::Ready {
                    data_about_bot: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Ready",
                    "data_about_bot",
                    &__self_0,
                ),
                FullEvent::Resume { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f, "Resume", "event", &__self_0,
                    )
                }
                FullEvent::ShardStageUpdate { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ShardStageUpdate",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::SoundboardSounds { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SoundboardSounds",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::SoundboardSoundCreate { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SoundboardSoundCreate",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::SoundboardSoundUpdate { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SoundboardSoundUpdate",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::SoundboardSoundsUpdate { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SoundboardSoundsUpdate",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::SoundboardSoundDelete { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SoundboardSoundDelete",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::TypingStart { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "TypingStart",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::UserUpdate {
                    old_data: __self_0,
                    new: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UserUpdate",
                    "old_data",
                    __self_0,
                    "new",
                    &__self_1,
                ),
                FullEvent::VoiceServerUpdate { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "VoiceServerUpdate",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::VoiceStateUpdate {
                    old: __self_0,
                    new: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "VoiceStateUpdate",
                    "old",
                    __self_0,
                    "new",
                    &__self_1,
                ),
                FullEvent::VoiceChannelStatusUpdate {
                    old: __self_0,
                    status: __self_1,
                    id: __self_2,
                    guild_id: __self_3,
                } => ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "VoiceChannelStatusUpdate",
                    "old",
                    __self_0,
                    "status",
                    __self_1,
                    "id",
                    __self_2,
                    "guild_id",
                    &__self_3,
                ),
                FullEvent::WebhookUpdate {
                    guild_id: __self_0,
                    belongs_to_channel_id: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "WebhookUpdate",
                    "guild_id",
                    __self_0,
                    "belongs_to_channel_id",
                    &__self_1,
                ),
                FullEvent::InteractionCreate {
                    interaction: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "InteractionCreate",
                    "interaction",
                    &__self_0,
                ),
                FullEvent::IntegrationCreate {
                    integration: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "IntegrationCreate",
                    "integration",
                    &__self_0,
                ),
                FullEvent::IntegrationUpdate {
                    integration: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "IntegrationUpdate",
                    "integration",
                    &__self_0,
                ),
                FullEvent::IntegrationDelete {
                    integration_id: __self_0,
                    guild_id: __self_1,
                    application_id: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "IntegrationDelete",
                    "integration_id",
                    __self_0,
                    "guild_id",
                    __self_1,
                    "application_id",
                    &__self_2,
                ),
                FullEvent::StageInstanceCreate {
                    stage_instance: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "StageInstanceCreate",
                    "stage_instance",
                    &__self_0,
                ),
                FullEvent::StageInstanceUpdate {
                    stage_instance: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "StageInstanceUpdate",
                    "stage_instance",
                    &__self_0,
                ),
                FullEvent::StageInstanceDelete {
                    stage_instance: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "StageInstanceDelete",
                    "stage_instance",
                    &__self_0,
                ),
                FullEvent::ThreadCreate { thread: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ThreadCreate",
                        "thread",
                        &__self_0,
                    )
                }
                FullEvent::ThreadUpdate {
                    old: __self_0,
                    new: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ThreadUpdate",
                    "old",
                    __self_0,
                    "new",
                    &__self_1,
                ),
                FullEvent::ThreadDelete {
                    thread: __self_0,
                    full_thread_data: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ThreadDelete",
                    "thread",
                    __self_0,
                    "full_thread_data",
                    &__self_1,
                ),
                FullEvent::ThreadListSync {
                    thread_list_sync: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ThreadListSync",
                    "thread_list_sync",
                    &__self_0,
                ),
                FullEvent::ThreadMemberUpdate {
                    thread_member: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ThreadMemberUpdate",
                    "thread_member",
                    &__self_0,
                ),
                FullEvent::ThreadMembersUpdate {
                    thread_members_update: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ThreadMembersUpdate",
                    "thread_members_update",
                    &__self_0,
                ),
                FullEvent::GuildScheduledEventCreate { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "GuildScheduledEventCreate",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::GuildScheduledEventUpdate { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "GuildScheduledEventUpdate",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::GuildScheduledEventDelete { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "GuildScheduledEventDelete",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::GuildScheduledEventUserAdd {
                    subscribed: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "GuildScheduledEventUserAdd",
                    "subscribed",
                    &__self_0,
                ),
                FullEvent::GuildScheduledEventUserRemove {
                    unsubscribed: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "GuildScheduledEventUserRemove",
                    "unsubscribed",
                    &__self_0,
                ),
                FullEvent::EntitlementCreate {
                    entitlement: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "EntitlementCreate",
                    "entitlement",
                    &__self_0,
                ),
                FullEvent::EntitlementUpdate {
                    entitlement: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "EntitlementUpdate",
                    "entitlement",
                    &__self_0,
                ),
                FullEvent::EntitlementDelete {
                    entitlement: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "EntitlementDelete",
                    "entitlement",
                    &__self_0,
                ),
                FullEvent::MessagePollVoteAdd { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "MessagePollVoteAdd",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::MessagePollVoteRemove { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "MessagePollVoteRemove",
                        "event",
                        &__self_0,
                    )
                }
                FullEvent::Ratelimit { data: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Ratelimit",
                        "data",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl FullEvent {
        /// Returns the name of this event as a snake case string
        ///
        /// ```rust,no_run
        /// # use serenity::client::{Context, FullEvent};
        /// # fn foo_(ctx: Context, event: FullEvent) {
        /// if let FullEvent::Message { .. } = &event {
        ///     assert_eq!(event.snake_case_name(), "message");
        /// }
        /// # }
        /// ```
        #[must_use]
        pub fn snake_case_name(&self) -> &'static str {
            #[allow(deprecated)]
            match self {
                Self::CommandPermissionsUpdate { .. } => "command_permissions_update",
                Self::AutoModRuleCreate { .. } => "auto_moderation_rule_create",
                Self::AutoModRuleUpdate { .. } => "auto_moderation_rule_update",
                Self::AutoModRuleDelete { .. } => "auto_moderation_rule_delete",
                Self::AutoModActionExecution { .. } => "auto_moderation_action_execution",
                Self::CacheReady { .. } => "cache_ready",
                Self::ShardsReady { .. } => "shards_ready",
                Self::ChannelCreate { .. } => "channel_create",
                Self::CategoryCreate { .. } => "category_create",
                Self::CategoryDelete { .. } => "category_delete",
                Self::ChannelDelete { .. } => "channel_delete",
                Self::ChannelPinsUpdate { .. } => "channel_pins_update",
                Self::ChannelUpdate { .. } => "channel_update",
                Self::GuildAuditLogEntryCreate { .. } => "guild_audit_log_entry_create",
                Self::GuildBanAddition { .. } => "guild_ban_addition",
                Self::GuildBanRemoval { .. } => "guild_ban_removal",
                Self::GuildCreate { .. } => "guild_create",
                Self::GuildDelete { .. } => "guild_delete",
                Self::GuildEmojisUpdate { .. } => "guild_emojis_update",
                Self::GuildIntegrationsUpdate { .. } => "guild_integrations_update",
                Self::GuildMemberAddition { .. } => "guild_member_addition",
                Self::GuildMemberRemoval { .. } => "guild_member_removal",
                Self::GuildMemberUpdate { .. } => "guild_member_update",
                Self::GuildMembersChunk { .. } => "guild_members_chunk",
                Self::GuildRoleCreate { .. } => "guild_role_create",
                Self::GuildRoleDelete { .. } => "guild_role_delete",
                Self::GuildRoleUpdate { .. } => "guild_role_update",
                Self::GuildStickersUpdate { .. } => "guild_stickers_update",
                Self::GuildUpdate { .. } => "guild_update",
                Self::InviteCreate { .. } => "invite_create",
                Self::InviteDelete { .. } => "invite_delete",
                Self::Message { .. } => "message",
                Self::MessageDelete { .. } => "message_delete",
                Self::MessageDeleteBulk { .. } => "message_delete_bulk",
                Self::MessageUpdate { .. } => "message_update",
                Self::ReactionAdd { .. } => "reaction_add",
                Self::ReactionRemove { .. } => "reaction_remove",
                Self::ReactionRemoveAll { .. } => "reaction_remove_all",
                Self::ReactionRemoveEmoji { .. } => "reaction_remove_emoji",
                Self::PresenceReplace { .. } => "presence_replace",
                Self::PresenceUpdate { .. } => "presence_update",
                Self::Ready { .. } => "ready",
                Self::Resume { .. } => "resume",
                Self::ShardStageUpdate { .. } => "shard_stage_update",
                Self::SoundboardSounds { .. } => "soundboard_sounds",
                Self::SoundboardSoundCreate { .. } => "soundboard_sound_create",
                Self::SoundboardSoundUpdate { .. } => "soundboard_sound_update",
                Self::SoundboardSoundsUpdate { .. } => "soundboard_sounds_update",
                Self::SoundboardSoundDelete { .. } => "soundboard_sound_delete",
                Self::TypingStart { .. } => "typing_start",
                Self::UserUpdate { .. } => "user_update",
                Self::VoiceServerUpdate { .. } => "voice_server_update",
                Self::VoiceStateUpdate { .. } => "voice_state_update",
                Self::VoiceChannelStatusUpdate { .. } => "voice_channel_status_update",
                Self::WebhookUpdate { .. } => "webhook_update",
                Self::InteractionCreate { .. } => "interaction_create",
                Self::IntegrationCreate { .. } => "integration_create",
                Self::IntegrationUpdate { .. } => "integration_update",
                Self::IntegrationDelete { .. } => "integration_delete",
                Self::StageInstanceCreate { .. } => "stage_instance_create",
                Self::StageInstanceUpdate { .. } => "stage_instance_update",
                Self::StageInstanceDelete { .. } => "stage_instance_delete",
                Self::ThreadCreate { .. } => "thread_create",
                Self::ThreadUpdate { .. } => "thread_update",
                Self::ThreadDelete { .. } => "thread_delete",
                Self::ThreadListSync { .. } => "thread_list_sync",
                Self::ThreadMemberUpdate { .. } => "thread_member_update",
                Self::ThreadMembersUpdate { .. } => "thread_members_update",
                Self::GuildScheduledEventCreate { .. } => "guild_scheduled_event_create",
                Self::GuildScheduledEventUpdate { .. } => "guild_scheduled_event_update",
                Self::GuildScheduledEventDelete { .. } => "guild_scheduled_event_delete",
                Self::GuildScheduledEventUserAdd { .. } => "guild_scheduled_event_user_add",
                Self::GuildScheduledEventUserRemove { .. } => "guild_scheduled_event_user_remove",
                Self::EntitlementCreate { .. } => "entitlement_create",
                Self::EntitlementUpdate { .. } => "entitlement_update",
                Self::EntitlementDelete { .. } => "entitlement_delete",
                Self::MessagePollVoteAdd { .. } => "poll_vote_add",
                Self::MessagePollVoteRemove { .. } => "poll_vote_remove",
                Self::Ratelimit { .. } => "ratelimit",
            }
        }
        /// Runs the given [`EventHandler`]'s code for this event.
        pub async fn dispatch(self, ctx: Context, handler: &dyn EventHandler) {
            #[allow(deprecated)]
            match self {
                Self::CommandPermissionsUpdate { permission } => {
                    let ctx = ctx;
                    handler.command_permissions_update(ctx, permission).await;
                }
                Self::AutoModRuleCreate { rule } => {
                    let ctx = ctx;
                    handler.auto_moderation_rule_create(ctx, rule).await;
                }
                Self::AutoModRuleUpdate { rule } => {
                    let ctx = ctx;
                    handler.auto_moderation_rule_update(ctx, rule).await;
                }
                Self::AutoModRuleDelete { rule } => {
                    let ctx = ctx;
                    handler.auto_moderation_rule_delete(ctx, rule).await;
                }
                Self::AutoModActionExecution { execution } => {
                    let ctx = ctx;
                    handler
                        .auto_moderation_action_execution(ctx, execution)
                        .await;
                }
                Self::CacheReady { guilds } => {
                    let ctx = ctx;
                    handler.cache_ready(ctx, guilds).await;
                }
                Self::ShardsReady { total_shards } => {
                    let ctx = ctx;
                    handler.shards_ready(ctx, total_shards).await;
                }
                Self::ChannelCreate { channel } => {
                    let ctx = ctx;
                    handler.channel_create(ctx, channel).await;
                }
                Self::CategoryCreate { category } => {
                    let ctx = ctx;
                    handler.category_create(ctx, category).await;
                }
                Self::CategoryDelete { category } => {
                    let ctx = ctx;
                    handler.category_delete(ctx, category).await;
                }
                Self::ChannelDelete { channel, messages } => {
                    let ctx = ctx;
                    handler.channel_delete(ctx, channel, messages).await;
                }
                Self::ChannelPinsUpdate { pin } => {
                    let ctx = ctx;
                    handler.channel_pins_update(ctx, pin).await;
                }
                Self::ChannelUpdate { old, new } => {
                    let ctx = ctx;
                    handler.channel_update(ctx, old, new).await;
                }
                Self::GuildAuditLogEntryCreate { entry, guild_id } => {
                    let ctx = ctx;
                    handler
                        .guild_audit_log_entry_create(ctx, entry, guild_id)
                        .await;
                }
                Self::GuildBanAddition {
                    guild_id,
                    banned_user,
                } => {
                    let ctx = ctx;
                    handler.guild_ban_addition(ctx, guild_id, banned_user).await;
                }
                Self::GuildBanRemoval {
                    guild_id,
                    unbanned_user,
                } => {
                    let ctx = ctx;
                    handler
                        .guild_ban_removal(ctx, guild_id, unbanned_user)
                        .await;
                }
                Self::GuildCreate { guild, is_new } => {
                    let ctx = ctx;
                    handler.guild_create(ctx, guild, is_new).await;
                }
                Self::GuildDelete { incomplete, full } => {
                    let ctx = ctx;
                    handler.guild_delete(ctx, incomplete, full).await;
                }
                Self::GuildEmojisUpdate {
                    guild_id,
                    current_state,
                } => {
                    let ctx = ctx;
                    handler
                        .guild_emojis_update(ctx, guild_id, current_state)
                        .await;
                }
                Self::GuildIntegrationsUpdate { guild_id } => {
                    let ctx = ctx;
                    handler.guild_integrations_update(ctx, guild_id).await;
                }
                Self::GuildMemberAddition { new_member } => {
                    let ctx = ctx;
                    handler.guild_member_addition(ctx, new_member).await;
                }
                Self::GuildMemberRemoval {
                    guild_id,
                    user,
                    member_data_if_available,
                } => {
                    let ctx = ctx;
                    handler
                        .guild_member_removal(ctx, guild_id, user, member_data_if_available)
                        .await;
                }
                Self::GuildMemberUpdate {
                    old_if_available,
                    new,
                    event,
                } => {
                    let ctx = ctx;
                    handler
                        .guild_member_update(ctx, old_if_available, new, event)
                        .await;
                }
                Self::GuildMembersChunk { chunk } => {
                    let ctx = ctx;
                    handler.guild_members_chunk(ctx, chunk).await;
                }
                Self::GuildRoleCreate { new } => {
                    let ctx = ctx;
                    handler.guild_role_create(ctx, new).await;
                }
                Self::GuildRoleDelete {
                    guild_id,
                    removed_role_id,
                    removed_role_data_if_available,
                } => {
                    let ctx = ctx;
                    handler
                        .guild_role_delete(
                            ctx,
                            guild_id,
                            removed_role_id,
                            removed_role_data_if_available,
                        )
                        .await;
                }
                Self::GuildRoleUpdate {
                    old_data_if_available,
                    new,
                } => {
                    let ctx = ctx;
                    handler
                        .guild_role_update(ctx, old_data_if_available, new)
                        .await;
                }
                Self::GuildStickersUpdate {
                    guild_id,
                    current_state,
                } => {
                    let ctx = ctx;
                    handler
                        .guild_stickers_update(ctx, guild_id, current_state)
                        .await;
                }
                Self::GuildUpdate {
                    old_data_if_available,
                    new_data,
                } => {
                    let ctx = ctx;
                    handler
                        .guild_update(ctx, old_data_if_available, new_data)
                        .await;
                }
                Self::InviteCreate { data } => {
                    let ctx = ctx;
                    handler.invite_create(ctx, data).await;
                }
                Self::InviteDelete { data } => {
                    let ctx = ctx;
                    handler.invite_delete(ctx, data).await;
                }
                Self::Message { new_message } => {
                    let ctx = ctx;
                    handler.message(ctx, new_message).await;
                }
                Self::MessageDelete {
                    channel_id,
                    deleted_message_id,
                    guild_id,
                } => {
                    let ctx = ctx;
                    handler
                        .message_delete(ctx, channel_id, deleted_message_id, guild_id)
                        .await;
                }
                Self::MessageDeleteBulk {
                    channel_id,
                    multiple_deleted_messages_ids,
                    guild_id,
                } => {
                    let ctx = ctx;
                    handler
                        .message_delete_bulk(
                            ctx,
                            channel_id,
                            multiple_deleted_messages_ids,
                            guild_id,
                        )
                        .await;
                }
                Self::MessageUpdate {
                    old_if_available,
                    new,
                    event,
                } => {
                    let ctx = ctx;
                    handler
                        .message_update(ctx, old_if_available, new, event)
                        .await;
                }
                Self::ReactionAdd { add_reaction } => {
                    let ctx = ctx;
                    handler.reaction_add(ctx, add_reaction).await;
                }
                Self::ReactionRemove { removed_reaction } => {
                    let ctx = ctx;
                    handler.reaction_remove(ctx, removed_reaction).await;
                }
                Self::ReactionRemoveAll {
                    channel_id,
                    removed_from_message_id,
                } => {
                    let ctx = ctx;
                    handler
                        .reaction_remove_all(ctx, channel_id, removed_from_message_id)
                        .await;
                }
                Self::ReactionRemoveEmoji { removed_reactions } => {
                    let ctx = ctx;
                    handler.reaction_remove_emoji(ctx, removed_reactions).await;
                }
                Self::PresenceReplace { presences } => {
                    let ctx = ctx;
                    handler.presence_replace(ctx, presences).await;
                }
                Self::PresenceUpdate { new_data } => {
                    let ctx = ctx;
                    handler.presence_update(ctx, new_data).await;
                }
                Self::Ready { data_about_bot } => {
                    let ctx = ctx;
                    handler.ready(ctx, data_about_bot).await;
                }
                Self::Resume { event } => {
                    let ctx = ctx;
                    handler.resume(ctx, event).await;
                }
                Self::ShardStageUpdate { event } => {
                    let ctx = ctx;
                    handler.shard_stage_update(ctx, event).await;
                }
                Self::SoundboardSounds { event } => {
                    let ctx = ctx;
                    handler.soundboard_sounds(ctx, event).await;
                }
                Self::SoundboardSoundCreate { event } => {
                    let ctx = ctx;
                    handler.soundboard_sound_create(ctx, event).await;
                }
                Self::SoundboardSoundUpdate { event } => {
                    let ctx = ctx;
                    handler.soundboard_sound_update(ctx, event).await;
                }
                Self::SoundboardSoundsUpdate { event } => {
                    let ctx = ctx;
                    handler.soundboard_sounds_update(ctx, event).await;
                }
                Self::SoundboardSoundDelete { event } => {
                    let ctx = ctx;
                    handler.soundboard_sound_delete(ctx, event).await;
                }
                Self::TypingStart { event } => {
                    let ctx = ctx;
                    handler.typing_start(ctx, event).await;
                }
                Self::UserUpdate { old_data, new } => {
                    let ctx = ctx;
                    handler.user_update(ctx, old_data, new).await;
                }
                Self::VoiceServerUpdate { event } => {
                    let ctx = ctx;
                    handler.voice_server_update(ctx, event).await;
                }
                Self::VoiceStateUpdate { old, new } => {
                    let ctx = ctx;
                    handler.voice_state_update(ctx, old, new).await;
                }
                Self::VoiceChannelStatusUpdate {
                    old,
                    status,
                    id,
                    guild_id,
                } => {
                    let ctx = ctx;
                    handler
                        .voice_channel_status_update(ctx, old, status, id, guild_id)
                        .await;
                }
                Self::WebhookUpdate {
                    guild_id,
                    belongs_to_channel_id,
                } => {
                    let ctx = ctx;
                    handler
                        .webhook_update(ctx, guild_id, belongs_to_channel_id)
                        .await;
                }
                Self::InteractionCreate { interaction } => {
                    let ctx = ctx;
                    handler.interaction_create(ctx, interaction).await;
                }
                Self::IntegrationCreate { integration } => {
                    let ctx = ctx;
                    handler.integration_create(ctx, integration).await;
                }
                Self::IntegrationUpdate { integration } => {
                    let ctx = ctx;
                    handler.integration_update(ctx, integration).await;
                }
                Self::IntegrationDelete {
                    integration_id,
                    guild_id,
                    application_id,
                } => {
                    let ctx = ctx;
                    handler
                        .integration_delete(ctx, integration_id, guild_id, application_id)
                        .await;
                }
                Self::StageInstanceCreate { stage_instance } => {
                    let ctx = ctx;
                    handler.stage_instance_create(ctx, stage_instance).await;
                }
                Self::StageInstanceUpdate { stage_instance } => {
                    let ctx = ctx;
                    handler.stage_instance_update(ctx, stage_instance).await;
                }
                Self::StageInstanceDelete { stage_instance } => {
                    let ctx = ctx;
                    handler.stage_instance_delete(ctx, stage_instance).await;
                }
                Self::ThreadCreate { thread } => {
                    let ctx = ctx;
                    handler.thread_create(ctx, thread).await;
                }
                Self::ThreadUpdate { old, new } => {
                    let ctx = ctx;
                    handler.thread_update(ctx, old, new).await;
                }
                Self::ThreadDelete {
                    thread,
                    full_thread_data,
                } => {
                    let ctx = ctx;
                    handler.thread_delete(ctx, thread, full_thread_data).await;
                }
                Self::ThreadListSync { thread_list_sync } => {
                    let ctx = ctx;
                    handler.thread_list_sync(ctx, thread_list_sync).await;
                }
                Self::ThreadMemberUpdate { thread_member } => {
                    let ctx = ctx;
                    handler.thread_member_update(ctx, thread_member).await;
                }
                Self::ThreadMembersUpdate {
                    thread_members_update,
                } => {
                    let ctx = ctx;
                    handler
                        .thread_members_update(ctx, thread_members_update)
                        .await;
                }
                Self::GuildScheduledEventCreate { event } => {
                    let ctx = ctx;
                    handler.guild_scheduled_event_create(ctx, event).await;
                }
                Self::GuildScheduledEventUpdate { event } => {
                    let ctx = ctx;
                    handler.guild_scheduled_event_update(ctx, event).await;
                }
                Self::GuildScheduledEventDelete { event } => {
                    let ctx = ctx;
                    handler.guild_scheduled_event_delete(ctx, event).await;
                }
                Self::GuildScheduledEventUserAdd { subscribed } => {
                    let ctx = ctx;
                    handler
                        .guild_scheduled_event_user_add(ctx, subscribed)
                        .await;
                }
                Self::GuildScheduledEventUserRemove { unsubscribed } => {
                    let ctx = ctx;
                    handler
                        .guild_scheduled_event_user_remove(ctx, unsubscribed)
                        .await;
                }
                Self::EntitlementCreate { entitlement } => {
                    let ctx = ctx;
                    handler.entitlement_create(ctx, entitlement).await;
                }
                Self::EntitlementUpdate { entitlement } => {
                    let ctx = ctx;
                    handler.entitlement_update(ctx, entitlement).await;
                }
                Self::EntitlementDelete { entitlement } => {
                    let ctx = ctx;
                    handler.entitlement_delete(ctx, entitlement).await;
                }
                Self::MessagePollVoteAdd { event } => {
                    let ctx = ctx;
                    handler.poll_vote_add(ctx, event).await;
                }
                Self::MessagePollVoteRemove { event } => {
                    let ctx = ctx;
                    handler.poll_vote_remove(ctx, event).await;
                }
                Self::Ratelimit { data } => {
                    handler.ratelimit(data).await;
                }
            }
        }
    }
    /// This core trait for handling raw events
    pub trait RawEventHandler: Send + Sync {
        /// Dispatched when any event occurs
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
        fn raw_event<'life0, 'async_trait>(
            &'life0 self,
            _ctx: Context,
            _ev: Event,
        ) -> ::core::pin::Pin<
            Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                let __self = self;
                let _ctx = _ctx;
                let _ev = _ev;
                let _: () = {};
            })
        }
    }
}
