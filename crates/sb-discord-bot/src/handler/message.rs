use serenity::all::{
    ChannelId, Colour, Context, CreateActionRow, CreateButton, CreateEmbed, CreateMessage, Message,
};

use crate::{
    store::{
        bibleapi::{ChapterItem, get_chapter},
        store::{Store, StoreKey},
        valid_cache::get_valid_translations,
    },
    util::{
        format::{
            format_chapter_content, format_verse_content, get_passage_url, split_into_embed_chunks,
        },
        prefs::calc_translation,
        reference_parser::{ParsedReference, find_references, format_verse_numbers},
    },
};

pub async fn on_message(ctx: Context, msg: Message) {
    if msg.content == "!gmt" {
        let mut translations: Vec<String> = get_valid_translations()
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default();
        translations.sort();

        let sixth = translations.len() / 6;
        for chunk in translations.chunks(sixth.max(1)) {
            if let Err(why) = msg.channel_id.say(&ctx.http, chunk.join(", ")).await {
                println!("Error sending message: {:?}", why);
            }
        }
        return;
    }

    if msg.author.bot {
        return;
    }

    let store = {
        let data = ctx.data.read().await;
        match data.get::<StoreKey>() {
            Some(store) => store.clone(),
            None => return,
        }
    };

    let guild_id = msg.guild_id.map(|id| id.to_string());
    if let Some(gid) = &guild_id
        && !store.get_inline_detection_enabled(gid.clone()).await
    {
        return;
    }

    let references = find_references(&msg.content);
    if references.is_empty() {
        return;
    }

    let user_id = msg.author.id.to_string();
    let translation = calc_translation(None, &user_id, guild_id.as_deref(), store.as_ref()).await;

    for reference in &references {
        respond_to_reference(
            &ctx,
            msg.channel_id,
            reference,
            &translation,
            guild_id.as_deref(),
            store.as_ref(),
        )
        .await;
    }
}

async fn respond_to_reference(
    ctx: &Context,
    channel_id: ChannelId,
    reference: &ParsedReference,
    translation: &str,
    guild_id: Option<&str>,
    store: &dyn Store,
) {
    let Ok(response) = get_chapter(translation, &reference.book, reference.chapter).await else {
        return;
    };

    match &reference.verses {
        None => {
            let content = format_chapter_content(&response.chapter);
            let chunks = split_into_embed_chunks(&content);
            let total = chunks.len();
            let show_button = match guild_id {
                Some(gid) => store.get_seed_bible_links_enabled(gid.to_string()).await,
                None => true,
            };
            for (i, chunk) in chunks.into_iter().enumerate() {
                let title = if total > 1 {
                    format!(
                        "{} {} ({}) ({}/{})",
                        reference.book,
                        reference.chapter,
                        translation,
                        i + 1,
                        total
                    )
                } else {
                    format!("{} {} ({})", reference.book, reference.chapter, translation)
                };
                let mut builder = CreateMessage::new().embed(
                    CreateEmbed::default()
                        .title(title)
                        .description(chunk)
                        .color(Colour::from_rgb(178, 255, 237)),
                );
                if show_button && i + 1 == total {
                    let chapter_string = reference.chapter.to_string();
                    builder = builder.components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new_link(get_passage_url(
                            Some(reference.book.get_3c_id()),
                            Some(&chapter_string),
                            Some(translation),
                            None,
                        ))
                        .label("Open in Seed Bible"),
                    ])]);
                }
                if let Err(why) = channel_id.send_message(&ctx.http, builder).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
        Some(verses) => {
            let found: Vec<_> = response
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
                .filter(|v| verses.contains(&v.number))
                .collect();
            if found.is_empty() {
                return;
            }
            let title = format!(
                "{} {}:{} ({})",
                reference.book,
                reference.chapter,
                format_verse_numbers(verses),
                translation
            );
            let mut content = format!("**{}**\n", title);
            for v in &found {
                content.push_str(&format_verse_content(v));
                content.push(' ');
            }
            if let Err(why) = channel_id.say(&ctx.http, content.trim()).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
