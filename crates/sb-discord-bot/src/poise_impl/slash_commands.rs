use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter, RoleId};
use strum::IntoEnumIterator;

use crate::{
    discord_util::user::user_is_admin,
    poise_impl::{
        data::Data,
        types::{Context, Error},
    },
    store::{
        bibleapi::get_chapter,
        contract::{BibleBooks, Translations},
        valid_cache::{get_chapter_count, is_valid_translation},
    },
    util::{
        format::{format_chapter_content, get_passage_url, split_into_embed_chunks},
        prefs::{calc_lang, calc_translation},
    },
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
    let store = &ctx.data().store;
    let user_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|g| g.to_string());
    let translation = calc_translation(translation.as_deref(), &user_id, guild_id.as_deref(), store.as_ref()).await;
    let lang = calc_lang(langauge.as_deref(), &user_id, guild_id.as_deref(), store.as_ref()).await;
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
pub async fn votd(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a random bible verse.")
)]
pub async fn truerandom(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get a random bible verse from a curated pool.")
)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
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
    let Ok(book) = book.parse::<BibleBooks>() else {
        ctx.send(CreateReply {
            content: Some(format!("Unknown book: {}", book)),
            ..Default::default()
        })
        .await?;
        return Ok(());
    };
    let store = &ctx.data().store;
    let user_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|g| g.to_string());
    let translation = calc_translation(translation.as_deref(), &user_id, guild_id.as_deref(), store.as_ref()).await;
    if let Some(max_chapter) = get_chapter_count(&translation, book.get_3c_id()).await {
        if chapter > max_chapter {
            ctx.send(CreateReply {
                embeds: vec![
                    CreateEmbed::default()
                        .title("Invalid chapter")
                        .description(format!("{} only has {} chapter(s) in {}.", book, max_chapter, translation))
                        .color(Colour::new(16730184)),
                ],
                ..Default::default()
            })
            .await?;
            return Ok(());
        }
    }
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
    #[description = "The translation to use"]
    #[autocomplete = "autocomplete_translation"]
    translation: Option<String>,
) -> Result<(), Error> {
    let store = &ctx.data().store;
    let user_id = ctx.author().id.to_string();
    let guild_id = ctx.guild_id().map(|g| g.to_string());
    let _translation = calc_translation(translation.as_deref(), &user_id, guild_id.as_deref(), store.as_ref()).await;
    Ok(())
}

pub fn all_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        help(),
        open(),
        settranslation(),
        setdailyverserole(),
        translations(),
        votd(),
        truerandom(),
        random(),
        chapter(),
        verse(),
    ]
}
