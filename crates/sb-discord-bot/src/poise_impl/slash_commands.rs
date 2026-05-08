use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter, RoleId};
use strum::IntoEnumIterator;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::{
    discord_util::user::user_is_admin,
    poise_impl::{
        data::Data,
        helpers::{parse_book, resolve_lang, resolve_translation, validate_chapter},
        types::{Context, Error},
    },
    store::{
        bibleapi::{get_chapter, ChapterItem},
        contract::{BibleBooks, Translations},
        curated::random_curated_verse,
        valid_cache::{get_all_book_chapters, get_chapter_count, is_valid_translation},
    },
    util::format::{format_chapter_content, format_verse_content, get_passage_url, split_into_embed_chunks},
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
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Help")
                .description("Seed Bible Bot Commands:\n\n`/open` - Open a passage in Seed Bible\n`/verse` - Get a specific verse\n`/chapter` — Get a full chapter\n`/random` — Random verse from curated pool\n`/truerandom` — Any verse in the Bible\n`/votd` — Verse of the day\n`/translations` — List available translations\n`/settranslation` — Set your personal default translation")
                .color(Colour::from_rgb(178, 255, 237))
                .footer(CreateEmbedFooter::new("(admin) /setservertranslation, /setseedbiblelinks, /setdailychannel, /setdailyverserole"))
        ],
        ..Default::default()
    })
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Open a passage at the given location ")
)]
pub async fn open(
    ctx: Context<'_>,
    #[description = "Book"]
    #[autocomplete = "autocomplete_book"]
    book: String,
    #[description = "Chapter"] chapter: String,
    #[autocomplete = "autocomplete_translation"]
    #[description = "Translation"]
    translation: Option<String>,
    #[description = "Language"] langauge: Option<String>,
) -> Result<(), Error> {
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    let lang = resolve_lang(&ctx, langauge.as_deref()).await;
    ctx.send(CreateReply {
        embeds: vec![CreateEmbed::default().title("Open").url(get_passage_url(
            &book,
            &chapter,
            Some(&translation),
            Some(&lang),
        ))],
        ..Default::default()
    })
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
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title(format!("Invalid Translation: {}", translation))
                    .color(Colour::new(16730184))
                    .description(
                        "See this [link](https://bible.helloao.org/api/available_translations.json).",
                    ),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
    let user_id = ctx.author().id.to_string();
    ctx.data()
        .store
        .set_user_translation(user_id, &translation)
        .await;
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(format!("Translation Set To: {}", translation))
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
        "Set the role that will be pinged when the daily verse is posted. (Admin only)"
    ),
    required_permissions = "ADMINISTRATOR"
)]
pub async fn setdailyverserole(ctx: Context<'_>, role: RoleId) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string(),
        None => {
            ctx.send(CreateReply {
                content: Some("This command can only be used in a server.".into()),
                ..Default::default()
            })
            .await?;
            return Ok(());
        }
    };

    let role_exists = ctx
        .guild()
        .map(|g| g.roles.contains_key(&role))
        .unwrap_or(false);
    if !role_exists {
        ctx.send(CreateReply {
            content: Some("That role does not exist in this server.".into()),
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
    ctx.data()
        .store
        .set_daily_verse_role(guild_id, role.to_string())
        .await;
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Daily verse role set successfully!")
                .description(format!("<@{}>", role)),
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

    let chunk_size = (all.len() / 7).max(1);
    let chunk = all
        .chunks(chunk_size)
        .nth((page - 1) as usize)
        .unwrap_or_default();

    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(format!("Available Translations (Page {}/7)", page))
                .description(chunk.join(", "))
                .color(Colour::from_rgb(178, 255, 237)),
        ],
        ..Default::default()
    })
    .await?;
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
        ctx.send(CreateReply {
            content: Some("This command can only be used in a server.".into()),
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    let store = &ctx.data().store;
    let Some((book_3c_id, chapter, verse_num)) = store.get_server_votd(&guild_id.to_string()).await else {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("No verse of the day set")
                    .description("An admin can set one with `/setvotd`.")
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    let Some(book) = BibleBooks::from_3c_id(&book_3c_id) else {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("Invalid verse of the day")
                    .description("The stored verse references an unrecognised book. An admin should reset it with `/setvotd`.")
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    let response = get_chapter(&translation, &book, chapter).await?;
    let found = response.chapter.content.into_iter().find_map(|item| {
        if let ChapterItem::Verse(v) = item {
            if v.number == verse_num { Some(v) } else { None }
        } else {
            None
        }
    });
    let Some(verse_data) = found else {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("Verse unavailable")
                    .description(format!(
                        "{} {}:{} is not available in {}. Try a different translation.",
                        book, chapter, verse_num, translation
                    ))
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(format!("Verse of the Day — {} {}:{} ({})", book, chapter, verse_data.number, translation))
                .description(format_verse_content(&verse_data))
                .color(Colour::from_rgb(178, 255, 237)),
        ],
        ..Default::default()
    })
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
        ctx.send(CreateReply {
            content: Some("This command can only be used in a server.".into()),
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    let Some(book) = parse_book(&ctx, &book).await? else { return Ok(()); };
    let translation = resolve_translation(&ctx, None).await;
    if !validate_chapter(&ctx, &book, &translation, chapter).await? { return Ok(()); }
    let response = get_chapter(&translation, &book, chapter).await?;
    let verse_exists = response.chapter.content.iter().any(|item| {
        matches!(item, ChapterItem::Verse(v) if v.number == verse)
    });
    if !verse_exists {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("Verse not found")
                    .description(format!("{} {} does not have a verse {}.", book, chapter, verse))
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
    ctx.data().store.set_server_votd(&guild_id.to_string(), book.get_3c_id(), chapter, verse).await;
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Verse of the day set")
                .description(format!("{} {}:{}", book, chapter, verse))
                .color(Colour::new(2736712)),
        ],
        ..Default::default()
    })
    .await?;
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
    let eligible: Vec<(BibleBooks, i64)> = book_chapters
        .into_iter()
        .filter_map(|(id, count)| BibleBooks::from_3c_id(&id).map(|b| (b, count)))
        .collect();
    let Some((book, max_chapters)) = eligible.choose(&mut rand::thread_rng()) else {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("No books found")
                    .description(format!("Could not load books for translation {}.", translation))
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    let chapter = rand::thread_rng().gen_range(1..=*max_chapters);
    let response = get_chapter(&translation, book, chapter).await?;
    let verses: Vec<_> = response.chapter.content.into_iter().filter_map(|item| {
        if let ChapterItem::Verse(v) = item { Some(v) } else { None }
    }).collect();
    let Some(verse_data) = verses.choose(&mut rand::thread_rng()) else {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("No verses found")
                    .description("The randomly selected chapter had no verses.")
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(format!("{} {}:{} ({})", book, chapter, verse_data.number, translation))
                .description(format_verse_content(verse_data))
                .color(Colour::from_rgb(178, 255, 237)),
        ],
        ..Default::default()
    })
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
    let unavailable = match get_chapter_count(&translation, verse_ref.book.get_3c_id()).await {
        None => true,
        Some(max) => verse_ref.chapter > max,
    };
    if unavailable {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("Verse unavailable")
                    .description(format!(
                        "{} {}:{} is not available in {}. Try a different translation.",
                        verse_ref.book, verse_ref.chapter, verse_ref.verse, translation
                    ))
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
    let response = get_chapter(&translation, &verse_ref.book, verse_ref.chapter).await?;
    let found = response.chapter.content.into_iter().find_map(|item| {
        if let ChapterItem::Verse(v) = item {
            if v.number == verse_ref.verse { Some(v) } else { None }
        } else {
            None
        }
    });
    let Some(verse_data) = found else {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("Verse unavailable")
                    .description(format!(
                        "{} {}:{} is not available in {}. Try a different translation.",
                        verse_ref.book, verse_ref.chapter, verse_ref.verse, translation
                    ))
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title(format!(
                    "{} {}:{} ({})",
                    verse_ref.book, verse_ref.chapter, verse_data.number, translation
                ))
                .description(format_verse_content(&verse_data))
                .color(Colour::from_rgb(178, 255, 237)),
        ],
        ..Default::default()
    })
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
    let Some(book) = parse_book(&ctx, &book).await? else { return Ok(()); };
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    if !validate_chapter(&ctx, &book, &translation, chapter).await? { return Ok(()); }
    let response = get_chapter(&translation, &book, chapter).await?;
    let content = format_chapter_content(&response.chapter);
    let chunks = split_into_embed_chunks(&content);
    let total = chunks.len();
    for (i, chunk) in chunks.into_iter().enumerate() {
        let title = if total > 1 {
            format!("{} {} ({}/{})", book, chapter, i + 1, total)
        } else {
            format!("{} {}", book, chapter)
        };
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title(title)
                    .description(chunk)
                    .color(Colour::from_rgb(178, 255, 237)),
            ],
            ..Default::default()
        })
        .await?;
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

    if let Some(end) = end_verse {
        if end < verse {
            ctx.send(CreateReply {
                embeds: vec![
                    CreateEmbed::default()
                        .title("Invalid verse range")
                        .description("End verse must be greater than or equal to the start verse.")
                        .color(Colour::new(16730184)),
                ],
                ..Default::default()
            })
            .await?;
            return Ok(());
        }
    }

    let Some(book) = parse_book(&ctx, &book).await? else { return Ok(()); };
    let translation = resolve_translation(&ctx, translation.as_deref()).await;
    if !validate_chapter(&ctx, &book, &translation, chapter).await? { return Ok(()); }
    let response = get_chapter(&translation, &book, chapter).await?;

    let Some(end) = end_verse else {
        let found = response.chapter.content.into_iter().find_map(|item| {
            if let ChapterItem::Verse(v) = item {
                if v.number == verse { Some(v) } else { None }
            } else {
                None
            }
        });
        let Some(verse_data) = found else {
            ctx.send(CreateReply {
                embeds: vec![
                    CreateEmbed::default()
                        .title("Verse not found")
                        .description(format!("{} {} does not have a verse {}.", book, chapter, verse))
                        .color(Colour::new(16730184)),
                ],
                ..Default::default()
            })
            .await?;
            return Ok(());
        };
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title(format!("{} {}:{} ({})", book, chapter, verse_data.number, translation))
                    .description(format_verse_content(&verse_data))
                    .color(Colour::from_rgb(178, 255, 237)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    };

    let verses_in_chapter: Vec<_> = response.chapter.content.into_iter().filter_map(|item| {
        if let ChapterItem::Verse(v) = item { Some(v) } else { None }
    }).collect();

    let verse_numbers: Vec<i64> = verses_in_chapter.iter().map(|v| v.number).collect();

    if !verse_numbers.contains(&verse) {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("Verse not found")
                    .description(format!("{} {} does not have a verse {}.", book, chapter, verse))
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }

    if !verse_numbers.contains(&end) {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title("End verse not found")
                    .description(format!("{} {} does not have a verse {}.", book, chapter, end))
                    .color(Colour::new(16730184)),
            ],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }

    let range_verses: Vec<_> = verses_in_chapter.iter()
        .filter(|v| v.number >= verse && v.number <= end)
        .collect();

    let content: String = range_verses.iter().map(|v| {
        format!("**{}** {}\n", v.number, format_verse_content(v))
    }).collect();

    let chunks = split_into_embed_chunks(&content);
    let total = chunks.len();
    for (i, chunk) in chunks.into_iter().enumerate() {
        let title = if total > 1 {
            format!("{} {}:{}-{} ({}) ({}/{})", book, chapter, verse, end, translation, i + 1, total)
        } else {
            format!("{} {}:{}-{} ({})", book, chapter, verse, end, translation)
        };
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::default()
                    .title(title)
                    .description(chunk)
                    .color(Colour::from_rgb(178, 255, 237)),
            ],
            ..Default::default()
        })
        .await?;
    }
    Ok(())
}

pub fn all_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        help(),
        open(),
        settranslation(),
        setdailyverserole(),
        setvotd(),
        translations(),
        votd(),
        truerandom(),
        random(),
        chapter(),
        verse(),
    ]
}
