use crate::store::contract::{BibleBooks, Translations};

#[derive(serde::Deserialize)]
pub struct Translation {
    pub id: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub language: String,
    #[serde(rename = "languageName")]
    pub language_name: String,
}

#[derive(serde::Deserialize)]
pub struct AvailableTranslationsResponse {
    pub translations: Vec<Translation>,
}

pub async fn get_available_translations() -> Result<AvailableTranslationsResponse, reqwest::Error> {
    reqwest::get("https://bible.helloao.org/api/available_translations.json")
        .await?
        .json()
        .await
}

#[derive(serde::Deserialize)]
pub struct ChapterHeading {
    pub content: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct NoteId {
    #[serde(rename = "noteId")]
    note_id: i64,
}

/// A single piece of content within a verse. Unknown shapes are preserved as
/// raw JSON so they never cause the enclosing verse to be silently dropped.
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum ChapterItemContent {
    Text(String),
    NoteId(NoteId),
    Unknown(serde_json::Value),
}

#[derive(serde::Deserialize)]
pub struct ChapterVerse {
    pub content: Vec<ChapterItemContent>,
    pub number: i64,
}

/// Internally tagged on the `"type"` field so the correct variant is always
/// chosen by discriminant rather than by trial-and-error struct matching.
/// `Unknown` catches any type strings not recognised by this client.
#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChapterItem {
    Verse(ChapterVerse),
    LineBreak,
    Heading(ChapterHeading),
    #[serde(other)]
    Unknown,
}

#[derive(serde::Deserialize)]
pub struct Chapter {
    // The chapter number
    pub number: i64,
    pub content: Vec<ChapterItem>,
}

#[derive(serde::Deserialize)]
pub struct ChapterResponse {
    pub chapter: Chapter,
}

pub async fn get_chapter(
    translation: &str,
    book: &BibleBooks,
    chapter: i64,
) -> Result<ChapterResponse, reqwest::Error> {
    reqwest::get(format!(
        "https://bible.helloao.org/api/{}/{}/{}.json",
        translation,
        book.get_3c_id(),
        chapter
    ))
    .await?
    .json()
    .await
}

#[derive(serde::Deserialize)]
pub struct TranslationBooksResponseTranslation {
    id: String,
    name: String,
    website: String,
    #[serde(rename = "licenseUrl")]
    license_url: String,
    #[serde(rename = "shortName")]
    short_name: String,
    language: String,
    #[serde(rename = "textDirection")]
    text_direction: String,
    #[serde(rename = "numberOfBooks")]
    number_of_books: i64,
    #[serde(rename = "totalNumberOfChapters")]
    total_number_of_chapters: i64,
    #[serde(rename = "totalNumberOfVerses")]
    total_number_of_verses: i64,
    #[serde(rename = "languageName")]
    language_name: String,
}

#[derive(serde::Deserialize)]
pub struct TranslationBooksResponseBookItem {
    pub id: String,
    name: String,
    order: i64,
    #[serde(rename = "numberOfChapters")]
    pub number_of_chapters: i64,
    #[serde(rename = "firstChapterNumber")]
    first_chapter_number: i64,
    #[serde(rename = "firstChapterApiLink")]
    first_chapter_api_link: String,
    #[serde(rename = "lastChapterNumber")]
    last_chapter_number: i64,
    #[serde(rename = "lastChapterApiLink")]
    last_chapter_api_link: String,
    #[serde(rename = "totalNumberOfVerses")]
    total_number_of_verses: i64,
}

#[derive(serde::Deserialize)]
pub struct TranslationBooksResponse {
    pub translation: TranslationBooksResponseTranslation,
    pub books: Vec<TranslationBooksResponseBookItem>,
}

pub async fn get_books_for_translation(
    translation: &str,
) -> Result<TranslationBooksResponse, reqwest::Error> {
    reqwest::get(format!(
        "https://bible.helloao.org/api/{}/books.json",
        translation
    ))
    .await?
    .json()
    .await
}

// pub async fn get_book_path(translation: String, book: String) -> String {
//     reqwest::get(format!(
//         "https://bible.helloao.org/api/{}/books.json",
//         translation
//     ))
//     .await?
//     .json()
//     .await
// }
