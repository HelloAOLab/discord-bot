pub fn get_passage_url(book: &str, chapter: &str, translation: &str, lang: &str) -> String {
    format!(
        "https://seed.bible/?book={}&chapter={}&translation={}&lang={}&source=discord_bot",
        book, chapter, translation, lang
    )
}
