use crate::store::bibleapi::{Chapter, ChapterItem, ChapterItemContent};

const EMBED_DESCRIPTION_LIMIT: usize = 4096;

pub fn format_chapter_content(chapter: &Chapter) -> String {
    let mut text = String::new();
    for item in &chapter.content {
        match item {
            ChapterItem::Verse(v) => {
                text.push_str(&format!("**{}** ", v.number));
                for c in &v.content {
                    if let ChapterItemContent::Text(t) = c {
                        text.push_str(t);
                    }
                }
                text.push('\n');
            }
            ChapterItem::Heading(h) => {
                text.push('\n');
                text.push_str(&h.content.join(" "));
                text.push('\n');
            }
            ChapterItem::LineBreak(_) => {
                text.push('\n');
            }
        }
    }
    text
}

/// Splits text into chunks that fit within Discord's embed description limit.
/// Splits preferring newline boundaries to avoid cutting mid-sentence.
pub fn split_into_embed_chunks(text: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut remaining = text;
    while !remaining.is_empty() {
        if remaining.len() <= EMBED_DESCRIPTION_LIMIT {
            chunks.push(remaining.to_string());
            break;
        }
        let split_at = remaining[..EMBED_DESCRIPTION_LIMIT]
            .rfind('\n')
            .unwrap_or(EMBED_DESCRIPTION_LIMIT);
        chunks.push(remaining[..split_at].to_string());
        remaining = remaining[split_at..].trim_start_matches('\n');
    }
    chunks
}

pub fn get_passage_url(book: &str, chapter: &str, translation: Option<&str>, lang: Option<&str>) -> String {
    let mut url = format!(
        "https://seed.bible/?book={}&chapter={}&source=discord_bot",
        book, chapter
    );
    if let Some(t) = translation {
        url.push_str(&format!("&translation={}", t));
    }
    if let Some(l) = lang {
        url.push_str(&format!("&lang={}", l));
    }
    url
}
