use crate::store::bibleapi::{Chapter, ChapterItem, ChapterItemContent, ChapterVerse};

const EMBED_DESCRIPTION_LIMIT: usize = 4096;

/// Recursively extracts plain text from an unknown JSON content item.
/// Handles objects with `"text"` or `"content"` fields, and arrays.
/// Returns an empty string for shapes that contain no readable text (e.g. `{noteId}`).
fn extract_text_from_value(val: &serde_json::Value) -> String {
    match val {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => arr.iter().map(extract_text_from_value).collect(),
        serde_json::Value::Object(map) => {
            if let Some(text) = map.get("text") {
                return extract_text_from_value(text);
            }
            if let Some(content) = map.get("content") {
                return extract_text_from_value(content);
            }
            String::new()
        }
        _ => String::new(),
    }
}

fn text_from_content_item(c: &ChapterItemContent) -> Option<String> {
    match c {
        ChapterItemContent::Text(t) => Some(t.clone()),
        ChapterItemContent::NoteId(_) => None,
        ChapterItemContent::Unknown(v) => {
            let extracted = extract_text_from_value(v);
            if extracted.is_empty() {
                None
            } else {
                Some(extracted)
            }
        }
    }
}

pub fn format_chapter_content(chapter: &Chapter) -> String {
    let mut text = String::new();
    for item in &chapter.content {
        match item {
            ChapterItem::Verse(v) => {
                text.push_str(&format!("**{}** ", v.number));
                for c in &v.content {
                    if let Some(t) = text_from_content_item(c) {
                        if !t.starts_with(char::is_whitespace)
                            && !text.ends_with(char::is_whitespace)
                        {
                            text.push(' ');
                        }
                        text.push_str(&t);
                    }
                }
                text.push('\n');
            }
            ChapterItem::Heading(h) => {
                text.push('\n');
                text.push_str(&h.content.join(" "));
                text.push('\n');
            }
            ChapterItem::LineBreak | ChapterItem::Unknown => {
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
        // Find the largest byte offset <= EMBED_DESCRIPTION_LIMIT that lands on a char boundary.
        let boundary = (0..=EMBED_DESCRIPTION_LIMIT)
            .rev()
            .find(|&i| remaining.is_char_boundary(i))
            .unwrap_or(0);
        let split_at = remaining[..boundary].rfind('\n').unwrap_or(boundary);
        chunks.push(remaining[..split_at].to_string());
        remaining = remaining[split_at..].trim_start_matches('\n');
    }
    chunks
}

pub fn format_verse_content(verse: &ChapterVerse) -> String {
    let mut result = String::new();
    for t in verse.content.iter().filter_map(text_from_content_item) {
        if !t.starts_with(char::is_whitespace)
            && !result.ends_with(char::is_whitespace)
            && !result.is_empty()
        {
            result.push(' ');
        }
        result.push_str(&t);
    }
    result
}

pub fn get_passage_url(
    book: &str,
    chapter: &str,
    translation: Option<&str>,
    lang: Option<&str>,
) -> String {
    let mut url = format!(
        "https://ao.bot/?pattern=SeedBible&noGridPortal=true&book={}&chapter={}&source=discord_bot",
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
