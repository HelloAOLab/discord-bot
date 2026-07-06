use crate::store::bibleapi::{
    Chapter, ChapterHeading, ChapterItem, ChapterItemContent, ChapterVerse,
};

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
    book: Option<&str>,
    chapter: Option<&str>,
    translation: Option<&str>,
    lang: Option<&str>,
) -> String {
    let mut url = "https://seedbible.org/?source=discord_bot".to_string();
    if let Some(b) = book {
        url.push_str(&format!("&book={}", b));
    }
    if let Some(c) = chapter {
        url.push_str(&format!("&chapter={}", c));
    }
    if let Some(t) = translation {
        url.push_str(&format!("&translation={}", t));
    }
    if let Some(l) = lang {
        url.push_str(&format!("&lang={}", l));
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verse(number: i64, text: &str) -> ChapterVerse {
        ChapterVerse {
            number,
            content: vec![ChapterItemContent::Text(text.to_string())],
        }
    }

    #[test]
    fn format_verse_content_joins_text_pieces() {
        let v = ChapterVerse {
            number: 16,
            content: vec![
                ChapterItemContent::Text("For God so loved".to_string()),
                ChapterItemContent::Text(" the world".to_string()),
            ],
        };
        assert_eq!(format_verse_content(&v), "For God so loved the world");
    }

    #[test]
    fn format_verse_content_skips_note_ids() {
        let v = ChapterVerse {
            number: 1,
            content: vec![
                ChapterItemContent::Text("Hello".to_string()),
                ChapterItemContent::NoteId(serde_json::from_str(r#"{"noteId":1}"#).unwrap()),
            ],
        };
        assert_eq!(format_verse_content(&v), "Hello");
    }

    #[test]
    fn format_verse_content_extracts_text_from_unknown_object_shape() {
        let v = ChapterVerse {
            number: 1,
            content: vec![ChapterItemContent::Unknown(
                serde_json::json!({"text": "nested"}),
            )],
        };
        assert_eq!(format_verse_content(&v), "nested");
    }

    #[test]
    fn format_chapter_content_includes_verse_numbers_and_headings() {
        let chapter = Chapter {
            number: 3,
            content: vec![
                ChapterItem::Heading(ChapterHeading {
                    content: vec!["Section".to_string()],
                }),
                ChapterItem::Verse(verse(16, "For God so loved the world")),
                ChapterItem::LineBreak,
            ],
        };
        let text = format_chapter_content(&chapter);
        assert!(text.contains("Section"));
        assert!(text.contains("**16**"));
        assert!(text.contains("For God so loved the world"));
    }

    #[test]
    fn split_into_embed_chunks_returns_single_chunk_when_under_limit() {
        let chunks = split_into_embed_chunks("short text");
        assert_eq!(chunks, vec!["short text".to_string()]);
    }

    #[test]
    fn split_into_embed_chunks_splits_long_text_on_newline_boundary() {
        let line = "a".repeat(100);
        let text = std::iter::repeat_n(line.clone(), 50)
            .collect::<Vec<_>>()
            .join("\n");
        let chunks = split_into_embed_chunks(&text);
        assert!(chunks.len() > 1);
        for chunk in &chunks {
            assert!(chunk.len() <= 4096);
        }
        assert_eq!(chunks.join("\n"), text);
    }

    #[test]
    fn split_into_embed_chunks_handles_empty_text() {
        assert!(split_into_embed_chunks("").is_empty());
    }

    #[test]
    fn get_passage_url_includes_only_provided_params() {
        let url = get_passage_url(None, None, None, None);
        assert_eq!(url, "https://seedbible.org/?source=discord_bot");
    }

    #[test]
    fn get_passage_url_includes_all_provided_params() {
        let url = get_passage_url(Some("JHN"), Some("3"), Some("BSB"), Some("en"));
        assert!(url.contains("book=JHN"));
        assert!(url.contains("chapter=3"));
        assert!(url.contains("translation=BSB"));
        assert!(url.contains("lang=en"));
    }
}
