use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, RoleId};
use strum::IntoEnumIterator;

use crate::{
    discord_util::user::user_is_admin,
    poise_impl::{
        data::Data,
        helpers::{
            chapter_in_range, parse_book, resolve_lang, resolve_translation, split_reference,
            validate_chapter,
        },
        replies::{
            apply_daily_verse_role, apply_set_votd, apply_translation_choice,
            build_chapter_replies, build_end_verse_not_found_reply, build_help_reply,
            build_invalid_chapter_number_reply, build_invalid_translation_reply,
            build_invalid_votd_book_reply, build_no_books_reply, build_no_verses_reply,
            build_no_votd_set_reply, build_open_reply, build_role_not_found_reply,
            build_translations_reply, build_verse_not_found_reply, build_verse_range_replies,
            build_verse_reply, build_verse_unavailable_reply, build_votd_reply, chapter_has_verse,
            eligible_books, find_verse, format_verse_range_content, not_in_server_reply,
            pick_random_book, pick_random_chapter, pick_random_verse, validate_verse_range,
            verse_numbers, verses_in_content, verses_in_range,
        },
        types::{Context, Error},
    },
    store::{
        bibleapi::{ChapterItem, get_chapter},
        contract::{BibleBooks, Translations},
        curated::random_curated_verse,
        valid_cache::{get_all_book_chapters, get_chapter_count, is_valid_translation},
    },
    util::format::{format_chapter_content, get_passage_url, split_into_embed_chunks},
};

async fn autocomplete_book<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    BibleBooks::iter()
        .map(|b| b.to_string())
        .filter(move |b| b.to_lowercase().starts_with(&partial.to_lowercase()))
}

async fn autocomplete_translation<'a>(
    _ctx: Context<'a>,
    partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    Translations::iter()
        .map(|t| t.to_string())
        .filter(move |t| t.to_lowercase().starts_with(&partial.to_lowercase()))
}

#[poise::command(
    slash_command,
    prefix_command,
    category = "General",
    description_localized("en-US", "Get a list of available commands")
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(build_help_reply()).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Open a passage in Seed Bible.")
)]
pub async fn open(
    ctx: Context<'_>,
    #[description = "Passage to open, e.g. \"John 3\" (optional)"] reference: Option<String>,
    #[autocomplete = "autocomplete_translation"]
    #[description = "Translation"]
    translation: Option<String>,
    #[description = "Seed Bible UI language"] lang: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    let lang = resolve_lang(&ctx, lang.as_deref()).await;

    let reference = reference
        .as_deref()
        .map(str::trim)
        .filter(|r| !r.is_empty());

    let mut book = None;
    let mut chapter: Option<i64> = None;

    if let Some(reference) = reference {
        let (book_name, chapter_str) = split_reference(reference);
        let Some(parsed_book) = parse_book(&ctx, &book_name).await? else {
            return Ok(());
        };

        if let Some(chapter_str) = chapter_str {
            let Ok(parsed_chapter) = chapter_str.parse::<i64>() else {
                ctx.send(build_invalid_chapter_number_reply(&chapter_str))
                    .await?;
                return Ok(());
            };
            if !validate_chapter(&ctx, &parsed_book, &translation, parsed_chapter).await? {
                return Ok(());
            }
            chapter = Some(parsed_chapter);
        }
        book = Some(parsed_book);
    }

    ctx.send(build_open_reply(
        book.as_ref(),
        chapter,
        &translation,
        &lang,
    ))
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized(
        "en-US",
        "Set your personal default translation for commands that support it"
    )
)]
pub async fn settranslation(
    ctx: Context<'_>,
    #[description = "Translation"]
    #[autocomplete = "autocomplete_translation"]
    translation: String,
) -> Result<(), Error> {
    if !is_valid_translation(&translation) {
        ctx.send(build_invalid_translation_reply(&translation))
            .await?;
        return Ok(());
    }
    let user_id = ctx.author().id.to_string();
    let reply = apply_translation_choice(ctx.data().store.as_ref(), user_id, &translation).await;
    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized(
        "en-US",
        "Set the role that will be pinged when the daily verse is posted. (Admin only)"
    ),
    required_permissions = "ADMINISTRATOR"
)]
pub async fn setdailyverserole(ctx: Context<'_>, role: RoleId) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string(),
        None => {
            ctx.send(not_in_server_reply()).await?;
            return Ok(());
        }
    };

    let role_exists = ctx
        .guild()
        .map(|g| g.roles.contains_key(&role))
        .unwrap_or(false);
    if !role_exists {
        ctx.send(build_role_not_found_reply()).await?;
        return Ok(());
    }
    let reply = apply_daily_verse_role(ctx.data().store.as_ref(), guild_id, role).await;
    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized(
        "en-US",
        "Toggle \"Open in Seed Bible\" links for this server. (Requires Manage Server)"
    ),
    required_permissions = "MANAGE_GUILD"
)]
pub async fn setseedbiblelinks(
    ctx: Context<'_>,
    #[description = "On or off"]
    #[choices("on", "off")]
    value: &'static str,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.send(CreateReply {
            content: Some("This command can only be used in a server.".into()),
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    let enabled = value == "on";
    ctx.data()
        .store
        .set_seed_bible_links_enabled(guild_id.to_string(), enabled)
        .await;
    let description = if enabled {
        "✅ Seed Bible links are now **on** for this server.".to_string()
    } else {
        "✅ Seed Bible links are now **off** for this server. Verses and chapters will be posted as plain text only.".to_string()
    };
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Seed Bible links updated")
                .description(description)
                .color(Colour::new(2736712)),
        ],
        ..Default::default()
    })
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized(
        "en-US",
        "Toggle automatic replies to Bible references typed in chat. (Requires Manage Server)"
    ),
    required_permissions = "MANAGE_GUILD"
)]
pub async fn setinlinedetection(
    ctx: Context<'_>,
    #[description = "On or off"]
    #[choices("on", "off")]
    value: &'static str,
) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.send(CreateReply {
            content: Some("This command can only be used in a server.".into()),
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    let enabled = value == "on";
    ctx.data()
        .store
        .set_inline_detection_enabled(guild_id.to_string(), enabled)
        .await;
    let description = if enabled {
        "✅ Inline verse detection is now **on** for this server.".to_string()
    } else {
        "✅ Inline verse detection is now **off** for this server.".to_string()
    };
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Inline verse detection updated")
                .description(description)
                .color(Colour::new(2736712)),
        ],
        ..Default::default()
    })
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "View all supported translations.")
)]
pub async fn translations(
    ctx: Context<'_>,
    #[description = "Page number (1-7)"]
    #[min = 1]
    #[max = 7]
    page: u8,
) -> Result<(), Error> {
    let mut all: Vec<String> = crate::store::valid_cache::get_valid_translations()
        .map(|set| set.iter().cloned().collect())
        .unwrap_or_default();
    all.sort();

    ctx.send(build_translations_reply(&all, page)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "See the verse of the day.")
)]
pub async fn votd(
    ctx: Context<'_>,
    #[description = "Translation"]
    #[autocomplete = "autocomplete_translation"]
    translation: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let Some(guild_id) = ctx.guild_id() else {
        ctx.send(not_in_server_reply()).await?;
        return Ok(());
    };
    let store = &ctx.data().store;
    let Some((book_3c_id, chapter, verse_num)) = store.get_server_votd(&guild_id.to_string()).await
    else {
        ctx.send(build_no_votd_set_reply()).await?;
        return Ok(());
    };
    let Some(book) = BibleBooks::from_3c_id(&book_3c_id) else {
        ctx.send(build_invalid_votd_book_reply()).await?;
        return Ok(());
    };
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    let response = get_chapter(&translation, &book, chapter).await?;
    let Some(verse_data) = find_verse(response.chapter.content, verse_num) else {
        ctx.send(build_verse_unavailable_reply(
            &book,
            chapter,
            verse_num,
            &translation,
        ))
        .await?;
        return Ok(());
    };
    ctx.send(build_votd_reply(&book, chapter, &verse_data, &translation))
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Set the verse of the day for this server. (Admin only)"),
    required_permissions = "ADMINISTRATOR"
)]
pub async fn setvotd(
    ctx: Context<'_>,
    #[description = "Book"]
    #[autocomplete = "autocomplete_book"]
    book: String,
    #[description = "Chapter"]
    #[min = 1]
    chapter: i64,
    #[description = "Verse"]
    #[min = 1]
    verse: i64,
) -> Result<(), Error> {
    ctx.defer().await?;
    let Some(guild_id) = ctx.guild_id() else {
        ctx.send(not_in_server_reply()).await?;
        return Ok(());
    };
    let Some(book) = parse_book(&ctx, &book).await? else {
        return Ok(());
    };
    let translation = resolve_translation(&ctx, None).await;
    if !validate_chapter(&ctx, &book, &translation, chapter).await? {
        return Ok(());
    }
    let response = get_chapter(&translation, &book, chapter).await?;
    if !chapter_has_verse(&response.chapter.content, verse) {
        ctx.send(build_verse_not_found_reply(&book, chapter, verse))
            .await?;
        return Ok(());
    }
    let reply = apply_set_votd(
        ctx.data().store.as_ref(),
        guild_id.to_string(),
        &book,
        chapter,
        verse,
    )
    .await;
    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a random bible verse.")
)]
pub async fn truerandom(
    ctx: Context<'_>,
    #[description = "Translation"]
    #[autocomplete = "autocomplete_translation"]
    translation: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    let book_chapters = get_all_book_chapters(&translation).await;
    let eligible = eligible_books(&book_chapters);
    let Some((book, max_chapters)) = pick_random_book(&eligible, &mut rand::thread_rng()) else {
        ctx.send(build_no_books_reply(&translation)).await?;
        return Ok(());
    };
    let chapter = pick_random_chapter(*max_chapters, &mut rand::thread_rng());
    let response = get_chapter(&translation, book, chapter).await?;
    let verses: Vec<_> = response
        .chapter
        .content
        .into_iter()
        .filter_map(|item| {
            if let ChapterItem::Verse(v) = item {
                Some(v)
            } else {
                None
            }
        })
        .collect();
    let Some(verse_data) = pick_random_verse(&verses, &mut rand::thread_rng()) else {
        ctx.send(build_no_verses_reply()).await?;
        return Ok(());
    };
    ctx.send(build_verse_reply(book, chapter, verse_data, &translation))
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a random bible verse from a curated pool.")
)]
pub async fn random(
    ctx: Context<'_>,
    #[description = "Translation"]
    #[autocomplete = "autocomplete_translation"]
    translation: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let verse_ref = random_curated_verse();
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    let max = get_chapter_count(&translation, verse_ref.book.get_3c_id()).await;
    if !chapter_in_range(verse_ref.chapter, max) {
        ctx.send(build_verse_unavailable_reply(
            &verse_ref.book,
            verse_ref.chapter,
            verse_ref.verse,
            &translation,
        ))
        .await?;
        return Ok(());
    }
    let response = get_chapter(&translation, &verse_ref.book, verse_ref.chapter).await?;
    let Some(verse_data) = find_verse(response.chapter.content, verse_ref.verse) else {
        ctx.send(build_verse_unavailable_reply(
            &verse_ref.book,
            verse_ref.chapter,
            verse_ref.verse,
            &translation,
        ))
        .await?;
        return Ok(());
    };
    ctx.send(build_verse_reply(
        &verse_ref.book,
        verse_ref.chapter,
        &verse_data,
        &translation,
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command, description_localized("en-US", "Get a full chapter."))]
pub async fn chapter(
    ctx: Context<'_>,
    #[description = "Book"]
    #[autocomplete = "autocomplete_book"]
    book: String,
    #[description = "Chapter"]
    #[min = 1]
    chapter: i64,
    #[description = "Translation"]
    #[autocomplete = "autocomplete_translation"]
    translation: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let Some(book) = parse_book(&ctx, &book).await? else {
        return Ok(());
    };
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    if !validate_chapter(&ctx, &book, &translation, chapter).await? {
        return Ok(());
    }
    let response = get_chapter(&translation, &book, chapter).await?;
    let content = format_chapter_content(&response.chapter);
    let chunks = split_into_embed_chunks(&content);
    let show_button = match ctx.guild_id() {
        Some(guild_id) => {
            ctx.data()
                .store
                .get_seed_bible_links_enabled(guild_id.to_string())
                .await
        }
        None => true,
    };
    let chapter_string = chapter.to_string();
    let open_url = show_button.then(|| {
        get_passage_url(
            Some(book.get_3c_id()),
            Some(&chapter_string),
            Some(&translation),
            None,
        )
    });
    for reply in build_chapter_replies(&book, chapter, chunks, open_url) {
        ctx.send(reply).await?;
    }
    Ok(())
}

#[poise::command(slash_command, description_localized("en-US", "Get a specific verse."))]
pub async fn verse(
    ctx: Context<'_>,
    #[description = "Book"]
    #[autocomplete = "autocomplete_book"]
    book: String,
    #[description = "Chapter"]
    #[min = 1]
    chapter: i64,
    #[description = "Verse"]
    #[min = 1]
    verse: i64,
    #[description = "End verse (optional, for a range like 1:1-5)"]
    #[min = 1]
    end_verse: Option<i64>,
    #[description = "Translation"]
    #[autocomplete = "autocomplete_translation"]
    translation: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;

    if let Err(reply) = validate_verse_range(verse, end_verse) {
        ctx.send(reply).await?;
        return Ok(());
    }

    let Some(book) = parse_book(&ctx, &book).await? else {
        return Ok(());
    };
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    if !validate_chapter(&ctx, &book, &translation, chapter).await? {
        return Ok(());
    }
    let response = get_chapter(&translation, &book, chapter).await?;

    let Some(end) = end_verse else {
        let Some(verse_data) = find_verse(response.chapter.content, verse) else {
            ctx.send(build_verse_not_found_reply(&book, chapter, verse))
                .await?;
            return Ok(());
        };
        ctx.send(build_verse_reply(&book, chapter, &verse_data, &translation))
            .await?;
        return Ok(());
    };

    let verses_in_chapter = verses_in_content(response.chapter.content);
    let numbers = verse_numbers(&verses_in_chapter);

    if !numbers.contains(&verse) {
        ctx.send(build_verse_not_found_reply(&book, chapter, verse))
            .await?;
        return Ok(());
    }

    if !numbers.contains(&end) {
        ctx.send(build_end_verse_not_found_reply(&book, chapter, end))
            .await?;
        return Ok(());
    }

    let range_verses = verses_in_range(&verses_in_chapter, verse, end);
    let content = format_verse_range_content(&range_verses);
    let chunks = split_into_embed_chunks(&content);
    for reply in build_verse_range_replies(&book, chapter, verse, end, &translation, chunks) {
        ctx.send(reply).await?;
    }
    Ok(())
}

pub fn all_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        help(),
        open(),
        settranslation(),
        setdailyverserole(),
        setseedbiblelinks(),
        setinlinedetection(),
        setvotd(),
        translations(),
        votd(),
        truerandom(),
        random(),
        chapter(),
        verse(),
    ]
}
