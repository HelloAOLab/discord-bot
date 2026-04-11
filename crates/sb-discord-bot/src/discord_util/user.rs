use serenity::model::permissions::Permissions;

use crate::poise_impl::types::Context;

pub async fn user_is_admin(ctx: Context<'_>) -> bool {
    if let Some(member) = ctx.author_member().await {
        if let Some(permissions) = member.permissions {
            return permissions.contains(Permissions::ADMINISTRATOR);
        }
    }
    false
}
