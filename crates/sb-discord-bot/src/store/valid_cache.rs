use std::{
    collections::{HashMap, HashSet},
    sync::{Mutex, OnceLock},
};

use crate::store::bibleapi::{get_available_translations, get_books_for_translation};

static VALID_TRANSLATIONS: OnceLock<HashSet<String>> = OnceLock::new();
static VALID_LANGUAGES: OnceLock<HashSet<String>> = OnceLock::new();
// Maps translation id -> (book 3c id -> chapter count), populated on demand.
static BOOK_CHAPTERS: OnceLock<Mutex<HashMap<String, HashMap<String, i64>>>> = OnceLock::new();

fn book_chapters_cache() -> &'static Mutex<HashMap<String, HashMap<String, i64>>> {
    BOOK_CHAPTERS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub async fn init_valid_translations() -> Result<(), reqwest::Error> {
    if let Ok(resp) = get_available_translations().await {
        let mut translations = HashSet::new();
        let mut languages = HashSet::new();
        for t in resp.translations {
            translations.insert(t.id);
            languages.insert(t.language);
        }
        VALID_TRANSLATIONS.set(translations).ok();
        VALID_LANGUAGES.set(languages).ok();
    }
    Ok(()) // TODO: Possibly retry or error handle.
}

pub fn get_valid_translations() -> Option<&'static HashSet<String>> {
    VALID_TRANSLATIONS.get()
}

pub fn is_valid_translation(translation: &str) -> bool {
    VALID_TRANSLATIONS
        .get()
        .map(|set| set.contains(translation))
        .unwrap_or(false)
}

pub fn get_valid_languages() -> Option<&'static HashSet<String>> {
    VALID_LANGUAGES.get()
}

pub fn is_valid_language(language: &str) -> bool {
    VALID_LANGUAGES
        .get()
        .map(|set| set.contains(language))
        .unwrap_or(false)
}

/// Fetches the book list for `translation` from the API and inserts it into the
/// cache. Does nothing if it is already cached. Returns `false` on fetch failure.
async fn ensure_books_cached(translation: &str) -> bool {
    {
        if book_chapters_cache()
            .lock()
            .unwrap()
            .contains_key(translation)
        {
            return true;
        }
    }
    let Ok(resp) = get_books_for_translation(translation).await else {
        return false;
    };
    let books: HashMap<String, i64> = resp
        .books
        .into_iter()
        .map(|b| (b.id, b.number_of_chapters))
        .collect();
    book_chapters_cache()
        .lock()
        .unwrap()
        .insert(translation.to_string(), books);
    true
}

/// Returns the chapter count for `book_3c_id` within `translation`,
/// fetching and caching the translation's book list on first access.
pub async fn get_chapter_count(translation: &str, book_3c_id: &str) -> Option<i64> {
    ensure_books_cached(translation).await;
    book_chapters_cache()
        .lock()
        .unwrap()
        .get(translation)
        .and_then(|books| books.get(book_3c_id).copied())
}

/// Returns all (book 3c id → chapter count) entries for `translation`,
/// fetching and caching on first access.
pub async fn get_all_book_chapters(translation: &str) -> HashMap<String, i64> {
    ensure_books_cached(translation).await;
    book_chapters_cache()
        .lock()
        .unwrap()
        .get(translation)
        .cloned()
        .unwrap_or_default()
}
