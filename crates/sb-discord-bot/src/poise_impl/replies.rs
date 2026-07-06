use std::collections::HashMap;

use poise::CreateReply;
use rand::{Rng, seq::SliceRandom};
use serenity::all::{
    Colour, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, RoleId,
};

use crate::{
    store::{
        bibleapi::{ChapterItem, ChapterVerse},
        contract::BibleBooks,
        store::Store,
    },
    util::format::{format_verse_content, get_passage_url},
};

pub fn not_in_server_reply() -> CreateReply {
    CreateReply {
        content: Some("This command can only be used in a server.".into()),
        ..Default::default()
    }
}

pub fn build_help_reply() -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Help")
                .description("Seed Bible Bot Commands:\n\n`/open` - Open a passage (or the app) in Seed Bible\n`/verse` - Get a specific verse\n`/chapter` — Get a full chapter\n`/random` — Random verse from curated pool\n`/truerandom` — Any verse in the Bible\n`/votd` — Verse of the day\n`/translations` — List available translations\n`/settranslation` — Set your personal default translation")
                .color(Colour::from_rgb(178, 255, 237))
                .footer(CreateEmbedFooter::new("(admin) /setservertranslation, /setseedbiblelinks, /setdailychannel, /setdailyverserole"))
        ],
        ..Default::default()
    }
}

pub fn build_invalid_translation_reply(translation: &str) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(format!("Invalid Translation: {}", translation))
                .color(Colour::new(16730184))
                .description(
                    "See this [link](https://bible.helloao.org/api/available_translations.json).",
                ),
        ],
        ..Default::default()
    }
}

pub fn build_translation_set_reply(translation: &str) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(format!("Translation Set To: {}", translation))
                .color(Colour::new(2736712)),
        ],
        ..Default::default()
    }
}

/// Persists the user's translation choice and returns the confirmation reply.
/// `Store` is dependency-injected, so this is testable without a live Discord context.
pub async fn apply_translation_choice(
    store: &dyn Store,
    user_id: String,
    translation: &str,
) -> CreateReply {
    store
        .set_user_translation(user_id, &translation.to_string())
        .await;
    build_translation_set_reply(translation)
}

pub fn build_role_not_found_reply() -> CreateReply {
    CreateReply {
        content: Some("That role does not exist in this server.".into()),
        ..Default::default()
    }
}

pub fn build_role_set_reply(role: RoleId) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Daily verse role set successfully!")
                .description(format!("<@{}>", role)),
        ],
        ..Default::default()
    }
}

/// Persists the server's daily verse role and returns the confirmation reply.
/// `Store` is dependency-injected, so this is testable without a live Discord context.
pub async fn apply_daily_verse_role(
    store: &dyn Store,
    guild_id: String,
    role: RoleId,
) -> CreateReply {
    store.set_daily_verse_role(guild_id, role.to_string()).await;
    build_role_set_reply(role)
}

/// Splits a sorted translation list into up to 7 pages and returns the
/// (title, description) for the requested page. Pure, no I/O.
pub fn paginate_translations(all: &[String], page: u8) -> (String, String) {
    let chunk_size = (all.len() / 7).max(1);
    let chunk = all
        .chunks(chunk_size)
        .nth((page - 1) as usize)
        .unwrap_or_default();
    (
        format!("Available Translations (Page {}/7)", page),
        chunk.join(", "),
    )
}

pub fn build_translations_reply(all: &[String], page: u8) -> CreateReply {
    let (title, description) = paginate_translations(all, page);
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(title)
                .description(description)
                .color(Colour::from_rgb(178, 255, 237)),
        ],
        ..Default::default()
    }
}

/// Finds the verse numbered `verse_num` within already-fetched chapter
/// content. Pure, no I/O — takes ownership to mirror how callers consume
/// an owned `ChapterResponse`.
pub fn find_verse(content: Vec<ChapterItem>, verse_num: i64) -> Option<ChapterVerse> {
    content.into_iter().find_map(|item| match item {
        ChapterItem::Verse(v) if v.number == verse_num => Some(v),
        _ => None,
    })
}

/// Whether already-fetched chapter content contains the given verse number. Pure, no I/O.
pub fn chapter_has_verse(content: &[ChapterItem], verse_num: i64) -> bool {
    content
        .iter()
        .any(|item| matches!(item, ChapterItem::Verse(v) if v.number == verse_num))
}

pub fn build_verse_embed_reply(title: String, verse_data: &ChapterVerse) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(title)
                .description(format_verse_content(verse_data))
                .color(Colour::from_rgb(178, 255, 237)),
        ],
        ..Default::default()
    }
}

pub fn build_verse_unavailable_reply(
    book: &BibleBooks,
    chapter: i64,
    verse: i64,
    translation: &str,
) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Verse unavailable")
                .description(format!(
                    "{} {}:{} is not available in {}. Try a different translation.",
                    book, chapter, verse, translation
                ))
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

pub fn build_verse_not_found_reply(book: &BibleBooks, chapter: i64, verse: i64) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Verse not found")
                .description(format!(
                    "{} {} does not have a verse {}.",
                    book, chapter, verse
                ))
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

pub fn build_no_votd_set_reply() -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("No verse of the day set")
                .description("An admin can set one with `/setvotd`.")
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

pub fn build_invalid_votd_book_reply() -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Invalid verse of the day")
                .description("The stored verse references an unrecognised book. An admin should reset it with `/setvotd`.")
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

pub fn build_votd_reply(
    book: &BibleBooks,
    chapter: i64,
    verse_data: &ChapterVerse,
    translation: &str,
) -> CreateReply {
    build_verse_embed_reply(
        format!(
            "Verse of the Day — {} {}:{} ({})",
            book, chapter, verse_data.number, translation
        ),
        verse_data,
    )
}

pub fn build_votd_set_reply(book: &BibleBooks, chapter: i64, verse: i64) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Verse of the day set")
                .description(format!("{} {}:{}", book, chapter, verse))
                .color(Colour::new(2736712)),
        ],
        ..Default::default()
    }
}

/// Persists the server's verse of the day and returns the confirmation reply.
/// `Store` is dependency-injected, so this is testable without a live Discord context.
pub async fn apply_set_votd(
    store: &dyn Store,
    guild_id: String,
    book: &BibleBooks,
    chapter: i64,
    verse: i64,
) -> CreateReply {
    store
        .set_server_votd(&guild_id, book.get_3c_id(), chapter, verse)
        .await;
    build_votd_set_reply(book, chapter, verse)
}

pub fn build_verse_reply(
    book: &BibleBooks,
    chapter: i64,
    verse_data: &ChapterVerse,
    translation: &str,
) -> CreateReply {
    build_verse_embed_reply(
        format!(
            "{} {}:{} ({})",
            book, chapter, verse_data.number, translation
        ),
        verse_data,
    )
}

pub fn build_no_books_reply(translation: &str) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("No books found")
                .description(format!(
                    "Could not load books for translation {}.",
                    translation
                ))
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

pub fn build_no_verses_reply() -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("No verses found")
                .description("The randomly selected chapter had no verses.")
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

/// Maps a translation's (book 3c id -> chapter count) map into eligible
/// `BibleBooks` entries, discarding any id that doesn't resolve to a known
/// book. Pure, no I/O.
pub fn eligible_books(book_chapters: &HashMap<String, i64>) -> Vec<(BibleBooks, i64)> {
    book_chapters
        .iter()
        .filter_map(|(id, count)| BibleBooks::from_3c_id(id).map(|b| (b, *count)))
        .collect()
}

pub fn pick_random_book<'a>(
    eligible: &'a [(BibleBooks, i64)],
    rng: &mut impl Rng,
) -> Option<&'a (BibleBooks, i64)> {
    eligible.choose(rng)
}

pub fn pick_random_chapter(max_chapters: i64, rng: &mut impl Rng) -> i64 {
    rng.gen_range(1..=max_chapters)
}

pub fn pick_random_verse<'a>(
    verses: &'a [ChapterVerse],
    rng: &mut impl Rng,
) -> Option<&'a ChapterVerse> {
    verses.choose(rng)
}

/// Builds the (possibly paginated) replies for a full chapter's already
/// chunked text. Pure, no I/O.
pub fn build_chapter_replies(
    book: &BibleBooks,
    chapter: i64,
    chunks: Vec<String>,
) -> Vec<CreateReply> {
    let total = chunks.len();
    chunks
        .into_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let title = if total > 1 {
                format!("{} {} ({}/{})", book, chapter, i + 1, total)
            } else {
                format!("{} {}", book, chapter)
            };
            CreateReply {
                embeds: vec![
                    CreateEmbed::default()
                        .title(title)
                        .description(chunk)
                        .color(Colour::from_rgb(178, 255, 237)),
                ],
                ..Default::default()
            }
        })
        .collect()
}

pub fn build_invalid_verse_range_reply() -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Invalid verse range")
                .description("End verse must be greater than or equal to the start verse.")
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

/// Validates that `end_verse` (if present) is not before `verse`. Pure, no I/O.
pub fn validate_verse_range(verse: i64, end_verse: Option<i64>) -> Result<(), CreateReply> {
    if let Some(end) = end_verse {
        if end < verse {
            return Err(build_invalid_verse_range_reply());
        }
    }
    Ok(())
}

pub fn build_end_verse_not_found_reply(book: &BibleBooks, chapter: i64, end: i64) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("End verse not found")
                .description(format!(
                    "{} {} does not have a verse {}.",
                    book, chapter, end
                ))
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

/// Extracts all verses from already-fetched chapter content, in order. Pure, no I/O.
pub fn verses_in_content(content: Vec<ChapterItem>) -> Vec<ChapterVerse> {
    content
        .into_iter()
        .filter_map(|item| match item {
            ChapterItem::Verse(v) => Some(v),
            _ => None,
        })
        .collect()
}

pub fn verse_numbers(verses: &[ChapterVerse]) -> Vec<i64> {
    verses.iter().map(|v| v.number).collect()
}

pub fn verses_in_range<'a>(
    verses: &'a [ChapterVerse],
    start: i64,
    end: i64,
) -> Vec<&'a ChapterVerse> {
    verses
        .iter()
        .filter(|v| v.number >= start && v.number <= end)
        .collect()
}

pub fn format_verse_range_content(range_verses: &[&ChapterVerse]) -> String {
    range_verses
        .iter()
        .map(|v| format!("**{}** {}\n", v.number, format_verse_content(v)))
        .collect()
}

/// Builds the (possibly paginated) replies for a verse range's already
/// chunked text. Pure, no I/O.
pub fn build_verse_range_replies(
    book: &BibleBooks,
    chapter: i64,
    verse: i64,
    end: i64,
    translation: &str,
    chunks: Vec<String>,
) -> Vec<CreateReply> {
    let total = chunks.len();
    chunks
        .into_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let title = if total > 1 {
                format!(
                    "{} {}:{}-{} ({}) ({}/{})",
                    book,
                    chapter,
                    verse,
                    end,
                    translation,
                    i + 1,
                    total
                )
            } else {
                format!("{} {}:{}-{} ({})", book, chapter, verse, end, translation)
            };
            CreateReply {
                embeds: vec![
                    CreateEmbed::default()
                        .title(title)
                        .description(chunk)
                        .color(Colour::from_rgb(178, 255, 237)),
                ],
                ..Default::default()
            }
        })
        .collect()
}

pub fn build_invalid_chapter_number_reply(chapter_str: &str) -> CreateReply {
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Invalid chapter")
                .description(format!(
                    "\"{}\" is not a valid chapter number.",
                    chapter_str
                ))
                .color(Colour::new(16730184)),
        ],
        ..Default::default()
    }
}

/// The descriptive line for `/open`, e.g. "Open John 3 in Seed Bible:". Pure, no I/O.
pub fn open_description(book: Option<&BibleBooks>, chapter: Option<i64>) -> String {
    match (book, chapter) {
        (Some(b), Some(c)) => format!("Open {} {} in Seed Bible:", b, c),
        (Some(b), None) => format!("Open {} in Seed Bible:", b),
        (None, _) => "Open Seed Bible:".to_string(),
    }
}

pub fn build_open_reply(
    book: Option<&BibleBooks>,
    chapter: Option<i64>,
    translation: &str,
    lang: &str,
) -> CreateReply {
    let description = open_description(book, chapter);
    let chapter_string = chapter.map(|c| c.to_string());
    let url = get_passage_url(
        book.map(BibleBooks::get_3c_id),
        chapter_string.as_deref(),
        Some(translation),
        Some(lang),
    );
    CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .description(description)
                .color(Colour::from_rgb(178, 255, 237)),
        ],
        components: Some(vec![CreateActionRow::Buttons(vec![
            CreateButton::new_link(url).label("Open →"),
        ])]),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        poise_impl::test_support::{embed_description, embed_title},
        store::fake::FakeStore,
    };

    #[test]
    fn help_reply_lists_commands() {
        let reply = build_help_reply();
        assert_eq!(embed_title(&reply, 0), "Help");
        assert!(embed_description(&reply, 0).contains("/verse"));
    }

    #[test]
    fn invalid_translation_reply_mentions_input() {
        let reply = build_invalid_translation_reply("XXX");
        assert!(embed_title(&reply, 0).contains("XXX"));
    }

    #[test]
    fn translation_set_reply_mentions_translation() {
        let reply = build_translation_set_reply("BSB");
        assert!(embed_title(&reply, 0).contains("BSB"));
    }

    #[tokio::test]
    async fn apply_translation_choice_persists_to_store() {
        let store = FakeStore::new();
        let reply = apply_translation_choice(&store, "u1".to_string(), "BSB").await;
        assert_eq!(store.user_translation("u1"), Some("BSB".to_string()));
        assert!(embed_title(&reply, 0).contains("BSB"));
    }

    #[tokio::test]
    async fn apply_daily_verse_role_persists_to_store() {
        let store = FakeStore::new();
        let role = RoleId::new(42);
        let reply = apply_daily_verse_role(&store, "g1".to_string(), role).await;
        assert_eq!(store.daily_verse_role("g1"), Some(role.to_string()));
        assert!(embed_description(&reply, 0).contains("42"));
    }

    #[test]
    fn paginate_translations_splits_evenly() {
        let all: Vec<String> = (1..=7).map(|n| format!("T{}", n)).collect();
        let (title, desc) = paginate_translations(&all, 1);
        assert_eq!(title, "Available Translations (Page 1/7)");
        assert_eq!(desc, "T1");
        let (_, last) = paginate_translations(&all, 7);
        assert_eq!(last, "T7");
    }

    #[test]
    fn paginate_translations_handles_fewer_than_seven_entries() {
        let all: Vec<String> = vec!["A".to_string(), "B".to_string()];
        let (_, first_page) = paginate_translations(&all, 1);
        assert_eq!(first_page, "A");
        let (_, out_of_range) = paginate_translations(&all, 7);
        assert_eq!(out_of_range, "");
    }

    #[test]
    fn paginate_translations_handles_empty_list() {
        let all: Vec<String> = vec![];
        let (_, desc) = paginate_translations(&all, 1);
        assert_eq!(desc, "");
    }

    fn verse_item(number: i64, text: &str) -> ChapterItem {
        ChapterItem::Verse(ChapterVerse {
            number,
            content: vec![crate::store::bibleapi::ChapterItemContent::Text(
                text.to_string(),
            )],
        })
    }

    #[test]
    fn find_verse_returns_matching_verse() {
        let content = vec![
            verse_item(15, "a"),
            verse_item(16, "b"),
            verse_item(17, "c"),
        ];
        let found = find_verse(content, 16).unwrap();
        assert_eq!(found.number, 16);
    }

    #[test]
    fn find_verse_returns_none_when_absent() {
        let content = vec![verse_item(15, "a")];
        assert!(find_verse(content, 99).is_none());
    }

    #[test]
    fn find_verse_skips_non_verse_items() {
        let content = vec![ChapterItem::LineBreak, verse_item(1, "a")];
        assert_eq!(find_verse(content, 1).unwrap().number, 1);
    }

    #[test]
    fn chapter_has_verse_true_when_present() {
        let content = vec![verse_item(1, "a"), verse_item(2, "b")];
        assert!(chapter_has_verse(&content, 2));
        assert!(!chapter_has_verse(&content, 3));
    }

    #[test]
    fn verse_unavailable_reply_mentions_reference_and_translation() {
        let reply = build_verse_unavailable_reply(&BibleBooks::John, 3, 16, "KJV");
        assert_eq!(embed_title(&reply, 0), "Verse unavailable");
        let desc = embed_description(&reply, 0);
        assert!(desc.contains("John"));
        assert!(desc.contains("3:16"));
        assert!(desc.contains("KJV"));
    }

    #[test]
    fn verse_not_found_reply_mentions_reference() {
        let reply = build_verse_not_found_reply(&BibleBooks::John, 99, 1);
        assert_eq!(embed_title(&reply, 0), "Verse not found");
        assert!(embed_description(&reply, 0).contains("99"));
    }

    #[test]
    fn no_votd_set_reply_mentions_setvotd() {
        let reply = build_no_votd_set_reply();
        assert!(embed_description(&reply, 0).contains("/setvotd"));
    }

    #[test]
    fn invalid_votd_book_reply_mentions_setvotd() {
        let reply = build_invalid_votd_book_reply();
        assert!(embed_description(&reply, 0).contains("/setvotd"));
    }

    #[test]
    fn votd_reply_contains_verse_text() {
        let verse_data = ChapterVerse {
            number: 16,
            content: vec![crate::store::bibleapi::ChapterItemContent::Text(
                "For God so loved the world".to_string(),
            )],
        };
        let reply = build_votd_reply(&BibleBooks::John, 3, &verse_data, "BSB");
        assert!(embed_title(&reply, 0).contains("Verse of the Day"));
        assert!(embed_description(&reply, 0).contains("For God so loved the world"));
    }

    #[tokio::test]
    async fn apply_set_votd_persists_to_store() {
        let store = FakeStore::new();
        let reply = apply_set_votd(&store, "g1".to_string(), &BibleBooks::John, 3, 16).await;
        assert_eq!(store.votd_for("g1"), Some(("JHN".to_string(), 3, 16)));
        assert!(embed_description(&reply, 0).contains("John 3:16"));
    }

    #[test]
    fn random_verse_reply_contains_reference_and_text() {
        let verse_data = ChapterVerse {
            number: 16,
            content: vec![crate::store::bibleapi::ChapterItemContent::Text(
                "For God so loved the world".to_string(),
            )],
        };
        let reply = build_verse_reply(&BibleBooks::John, 3, &verse_data, "BSB");
        assert!(embed_title(&reply, 0).contains("John 3:16"));
        assert!(embed_description(&reply, 0).contains("For God so loved the world"));
    }

    #[test]
    fn no_books_reply_mentions_translation() {
        let reply = build_no_books_reply("XYZ");
        assert!(embed_description(&reply, 0).contains("XYZ"));
    }

    #[test]
    fn eligible_books_maps_known_ids_and_drops_unknown() {
        let mut map = HashMap::new();
        map.insert("JHN".to_string(), 21);
        map.insert("UNKNOWN".to_string(), 5);
        let eligible = eligible_books(&map);
        assert_eq!(eligible.len(), 1);
        assert_eq!(eligible[0].0.get_3c_id(), "JHN");
        assert_eq!(eligible[0].1, 21);
    }

    #[test]
    fn eligible_books_handles_empty_map() {
        assert!(eligible_books(&HashMap::new()).is_empty());
    }

    #[test]
    fn pick_random_book_returns_none_for_empty_list() {
        let mut rng = rand::thread_rng();
        assert!(pick_random_book(&[], &mut rng).is_none());
    }

    #[test]
    fn pick_random_book_always_returns_an_eligible_entry() {
        let eligible = vec![(BibleBooks::John, 21), (BibleBooks::Genesis, 50)];
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let (book, _) = pick_random_book(&eligible, &mut rng).unwrap();
            assert!(matches!(book, BibleBooks::John | BibleBooks::Genesis));
        }
    }

    #[test]
    fn pick_random_chapter_stays_within_range() {
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let chapter = pick_random_chapter(5, &mut rng);
            assert!((1..=5).contains(&chapter));
        }
    }

    #[test]
    fn pick_random_chapter_handles_single_chapter_book() {
        let mut rng = rand::thread_rng();
        assert_eq!(pick_random_chapter(1, &mut rng), 1);
    }

    #[test]
    fn pick_random_verse_returns_none_for_empty_list() {
        let mut rng = rand::thread_rng();
        assert!(pick_random_verse(&[], &mut rng).is_none());
    }

    #[test]
    fn pick_random_verse_always_returns_a_provided_verse() {
        let verses = vec![
            ChapterVerse {
                number: 1,
                content: vec![],
            },
            ChapterVerse {
                number: 2,
                content: vec![],
            },
        ];
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let v = pick_random_verse(&verses, &mut rng).unwrap();
            assert!(v.number == 1 || v.number == 2);
        }
    }

    #[test]
    fn chapter_replies_single_chunk_has_no_pagination_suffix() {
        let replies = build_chapter_replies(&BibleBooks::John, 3, vec!["content".to_string()]);
        assert_eq!(replies.len(), 1);
        assert_eq!(embed_title(&replies[0], 0), "John 3");
        assert_eq!(embed_description(&replies[0], 0), "content");
    }

    #[test]
    fn chapter_replies_multiple_chunks_are_numbered() {
        let replies = build_chapter_replies(
            &BibleBooks::John,
            3,
            vec!["part1".to_string(), "part2".to_string()],
        );
        assert_eq!(replies.len(), 2);
        assert_eq!(embed_title(&replies[0], 0), "John 3 (1/2)");
        assert_eq!(embed_title(&replies[1], 0), "John 3 (2/2)");
    }

    #[test]
    fn validate_verse_range_accepts_missing_end() {
        assert!(validate_verse_range(5, None).is_ok());
    }

    #[test]
    fn validate_verse_range_accepts_end_equal_to_start() {
        assert!(validate_verse_range(5, Some(5)).is_ok());
    }

    #[test]
    fn validate_verse_range_accepts_end_after_start() {
        assert!(validate_verse_range(5, Some(10)).is_ok());
    }

    #[test]
    fn validate_verse_range_rejects_end_before_start() {
        let err = validate_verse_range(10, Some(5)).unwrap_err();
        assert_eq!(embed_title(&err, 0), "Invalid verse range");
    }

    #[test]
    fn end_verse_not_found_reply_mentions_end() {
        let reply = build_end_verse_not_found_reply(&BibleBooks::John, 3, 99);
        assert_eq!(embed_title(&reply, 0), "End verse not found");
        assert!(embed_description(&reply, 0).contains("99"));
    }

    #[test]
    fn verses_in_content_extracts_only_verses() {
        let content = vec![
            ChapterItem::LineBreak,
            verse_item(1, "a"),
            verse_item(2, "b"),
        ];
        let verses = verses_in_content(content);
        assert_eq!(verses.len(), 2);
        assert_eq!(verse_numbers(&verses), vec![1, 2]);
    }

    #[test]
    fn verses_in_range_filters_inclusive_bounds() {
        let content = vec![verse_item(1, "a"), verse_item(2, "b"), verse_item(3, "c")];
        let verses = verses_in_content(content);
        let range = verses_in_range(&verses, 2, 3);
        assert_eq!(verse_numbers_of_refs(&range), vec![2, 3]);
    }

    fn verse_numbers_of_refs(verses: &[&ChapterVerse]) -> Vec<i64> {
        verses.iter().map(|v| v.number).collect()
    }

    #[test]
    fn format_verse_range_content_includes_each_verse_number() {
        let content = vec![verse_item(1, "one"), verse_item(2, "two")];
        let verses = verses_in_content(content);
        let range: Vec<&ChapterVerse> = verses.iter().collect();
        let text = format_verse_range_content(&range);
        assert!(text.contains("**1** one"));
        assert!(text.contains("**2** two"));
    }

    #[test]
    fn verse_range_replies_single_chunk_has_no_pagination_suffix() {
        let replies = build_verse_range_replies(
            &BibleBooks::John,
            3,
            16,
            17,
            "BSB",
            vec!["content".to_string()],
        );
        assert_eq!(replies.len(), 1);
        assert_eq!(embed_title(&replies[0], 0), "John 3:16-17 (BSB)");
    }

    #[test]
    fn verse_range_replies_multiple_chunks_are_numbered() {
        let replies = build_verse_range_replies(
            &BibleBooks::John,
            3,
            16,
            17,
            "BSB",
            vec!["a".to_string(), "b".to_string()],
        );
        assert_eq!(embed_title(&replies[0], 0), "John 3:16-17 (BSB) (1/2)");
        assert_eq!(embed_title(&replies[1], 0), "John 3:16-17 (BSB) (2/2)");
    }

    #[test]
    fn invalid_chapter_number_reply_mentions_input() {
        let reply = build_invalid_chapter_number_reply("abc");
        assert!(embed_description(&reply, 0).contains("abc"));
    }

    #[test]
    fn open_description_with_book_and_chapter() {
        assert_eq!(
            open_description(Some(&BibleBooks::John), Some(3)),
            "Open John 3 in Seed Bible:"
        );
    }

    #[test]
    fn open_description_with_book_only() {
        assert_eq!(
            open_description(Some(&BibleBooks::John), None),
            "Open John in Seed Bible:"
        );
    }

    #[test]
    fn open_description_with_nothing() {
        assert_eq!(open_description(None, None), "Open Seed Bible:");
    }

    #[test]
    fn open_reply_uses_description_and_a_link_button() {
        let reply = build_open_reply(Some(&BibleBooks::John), Some(3), "BSB", "en");
        assert_eq!(embed_description(&reply, 0), "Open John 3 in Seed Bible:");
        assert!(reply.components.is_some());
    }
}
