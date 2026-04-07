use std::{collections::HashSet, sync::OnceLock};

static VALID_TRANSLATIONS: OnceLock<HashSet<String>> = OnceLock::new();

#[derive(serde::Deserialize)]
struct Translation {
    id: String,
    #[serde(rename = "shortName")]
    short_name: String,
    language: String,
    #[serde(rename = "languageName")]
    language_name: String,
}

#[derive(serde::Deserialize)]
struct AvailableTranslationsResponse {
    translations: Vec<Translation>,
}

pub async fn init_valid_translations() -> Result<(), reqwest::Error> {
    let resp: AvailableTranslationsResponse =
        reqwest::get("https://bible.helloao.org/api/available_translations.json")
            .await?
            .json()
            .await?;
    let set: HashSet<String> = resp.translations.into_iter().map(|t| t.id).collect();
    VALID_TRANSLATIONS.set(set).ok();
    Ok(())
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

