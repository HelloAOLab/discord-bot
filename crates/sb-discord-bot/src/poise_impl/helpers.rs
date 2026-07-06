use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};

use crate::{
    poise_impl::types::{Context, Error},
    store::{contract::BibleBooks, valid_cache::get_chapter_count},
    util::prefs::{calc_lang, calc_translation},
};

/// Reply shown when a user-supplied book name doesn't parse into a `BibleBooks` variant.
pub fn unknown_book_reply(book: &str) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Unknown book")
                .description(format!("\"{}\" is not a recognised Bible book.", book))
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

/// Parses a book name string into a `BibleBooks` variant. Pure, no I/O.
pub fn parse_book_str(book: &str) -> Option<BibleBooks> {
    book.parse::<BibleBooks>().ok()
}

/// Parses a book name string into a `BibleBooks` variant.
/// Sends an error embed and returns `None` if unrecognised.
pub async fn parse_book(ctx: &Context<'_>, book: &str) -> Result<Option<BibleBooks>, Error> {
    match parse_book_str(book) {
        Some(b) => Ok(Some(b)),
        None => {
            ctx.send(unknown_book_reply(book)).await?;
            Ok(None)
        }
    }
}

/// Reply shown when `chapter` exceeds `max` chapters available for `book` in `translation`.
pub fn invalid_chapter_reply(book: &BibleBooks, max: i64, translation: &str) -> CreateReply {
    CreateReply {
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
    }
}

/// Whether `chapter` is within range given an optional known chapter count.
/// `None` means the count couldn't be determined, in which case validation
/// is skipped and the API call itself is left to respond. Pure, no I/O.
pub fn chapter_in_range(chapter: i64, max_chapters: Option<i64>) -> bool {
    match max_chapters {
        Some(max) => chapter <= max,
        None => true,
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
    let max = get_chapter_count(translation, book.get_3c_id()).await;
    if !chapter_in_range(chapter, max) {
        let max = max.expect("chapter_in_range only rejects a known max");
        ctx.send(invalid_chapter_reply(book, max, translation))
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

/// Splits a free-form passage reference like `"1 John 3"` into a book name
/// and an optional chapter token. The final token is treated as the chapter
/// if it's numeric and there's more than one token; otherwise the whole
/// reference is treated as the book name (e.g. `"Genesis"`).
pub fn split_reference(reference: &str) -> (String, Option<String>) {
    let tokens: Vec<&str> = reference.split_whitespace().collect();
    if tokens.len() > 1 {
        if let Some(last) = tokens.last() {
            if !last.is_empty() && last.chars().all(|c| c.is_ascii_digit()) {
                let book = tokens[..tokens.len() - 1].join(" ");
                return (book, Some(last.to_string()));
            }
        }
    }
    (tokens.join(" "), None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poise_impl::test_support::{embed_description, embed_title};

    #[test]
    fn split_reference_separates_trailing_numeric_chapter() {
        assert_eq!(
            split_reference("John 3"),
            ("John".to_string(), Some("3".to_string()))
        );
    }

    #[test]
    fn split_reference_handles_multi_word_book_with_chapter() {
        assert_eq!(
            split_reference("1 John 3"),
            ("1 John".to_string(), Some("3".to_string()))
        );
    }

    #[test]
    fn split_reference_treats_whole_reference_as_book_without_chapter() {
        assert_eq!(split_reference("Genesis"), ("Genesis".to_string(), None));
    }

    #[test]
    fn split_reference_treats_multi_word_book_without_trailing_number_as_book_only() {
        assert_eq!(
            split_reference("Song of Solomon"),
            ("Song of Solomon".to_string(), None)
        );
    }

    #[test]
    fn split_reference_handles_empty_input() {
        assert_eq!(split_reference(""), ("".to_string(), None));
    }

    #[test]
    fn parse_book_str_accepts_known_book_case_insensitively() {
        assert!(matches!(parse_book_str("john"), Some(BibleBooks::John)));
        assert!(matches!(
            parse_book_str("1 Samuel"),
            Some(BibleBooks::Samuel1)
        ));
    }

    #[test]
    fn parse_book_str_rejects_unknown_book() {
        assert!(parse_book_str("Not A Book").is_none());
    }

    #[test]
    fn unknown_book_reply_mentions_the_input() {
        let reply = unknown_book_reply("Frodo");
        assert_eq!(embed_title(&reply, 0), "Unknown book");
        assert!(embed_description(&reply, 0).contains("Frodo"));
    }

    #[test]
    fn chapter_in_range_allows_unknown_max() {
        assert!(chapter_in_range(999, None));
    }

    #[test]
    fn chapter_in_range_allows_chapter_at_max() {
        assert!(chapter_in_range(5, Some(5)));
    }

    #[test]
    fn chapter_in_range_rejects_chapter_over_max() {
        assert!(!chapter_in_range(6, Some(5)));
    }

    #[test]
    fn invalid_chapter_reply_mentions_book_and_max() {
        let reply = invalid_chapter_reply(&BibleBooks::John, 21, "BSB");
        assert_eq!(embed_title(&reply, 0), "Invalid chapter");
        let description = embed_description(&reply, 0);
        assert!(description.contains("John"));
        assert!(description.contains("21"));
        assert!(description.contains("BSB"));
    }
}
