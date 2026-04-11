use serenity::all::{Colour, CreateEmbed};
use poise::CreateReply;

use crate::{
    poise_impl::types::{Context, Error},
    store::{
        contract::BibleBooks,
        valid_cache::get_chapter_count,
    },
    util::prefs::{calc_lang, calc_translation},
};

/// Parses a book name string into a `BibleBooks` variant.
/// Sends an error embed and returns `None` if unrecognised.
pub async fn parse_book(ctx: &Context<'_>, book: &str) -> Result<Option<BibleBooks>, Error> {
    match book.parse::<BibleBooks>() {
        Ok(b) => Ok(Some(b)),
        Err(_) => {
            ctx.send(CreateReply {
                embeds: vec![
                    CreateEmbed::default()
                        .title("Unknown book")
                        .description(format!("\"{}\" is not a recognised Bible book.", book))
                        .color(Colour::new(16730184)),
                ],
                ..Default::default()
            })
            .await?;
            Ok(None)
        }
    }
}

/// Validates that `chapter` is within range for `book` in `translation`.
/// Sends an error embed and returns `false` if out of range.
pub async fn validate_chapter(
    ctx: &Context<'_>,
    book: &BibleBooks,
    translation: &str,
    chapter: i64,
) -> Result<bool, Error> {
    let Some(max) = get_chapter_count(translation, book.get_3c_id()).await else {
        return Ok(true); // can't validate, let the API respond
    };
    if chapter > max {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("Invalid chapter")
                    .description(format!(
                        "{} only has {} chapter(s) in {}.",
                        book, max, translation
                    ))
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(false);
    }
    Ok(true)
}

/// Resolves the effective translation for the calling user, honouring the
/// priority chain: explicit choice → user pref → server pref → default.
pub async fn resolve_translation(ctx: &Context<'_>, user_choice: Option<&str>) -> String {
    let store = &ctx.data().store;
    let user_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|g| g.to_string());
    calc_translation(user_choice, &user_id, guild_id.as_deref(), store.as_ref()).await
}

/// Resolves the effective language for the calling user, honouring the
/// priority chain: explicit choice → user pref → server pref → default.
pub async fn resolve_lang(ctx: &Context<'_>, user_choice: Option<&str>) -> String {
    let store = &ctx.data().store;
    let user_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|g| g.to_string());
    calc_lang(user_choice, &user_id, guild_id.as_deref(), store.as_ref()).await
}
