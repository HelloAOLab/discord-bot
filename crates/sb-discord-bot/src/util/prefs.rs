use crate::store::{
    store::Store,
    valid_cache::{is_valid_language, is_valid_translation},
};

const DEFAULT_TRANSLATION: &str = "BSB";
const DEFAULT_LANGUAGE: &str = "en";

/// Resolves the effective translation from the priority chain: explicit
/// choice (if valid per `is_valid`) → user pref → server pref → default.
/// `is_valid` is dependency-injected so this stays testable without the
/// global translation cache in `valid_cache` being populated.
pub async fn calc_translation_with(
    user_choice: Option<&str>,
    user_id: &str,
    guild_id: Option<&str>,
    store: &dyn Store,
    is_valid: impl Fn(&str) -> bool,
) -> String {
    if let Some(choice) = user_choice {
        if is_valid(choice) {
            return choice.to_string();
        }
    }
    if let Some(t) = store.get_user_translation(user_id.to_string()).await {
        return t;
    }
    if let Some(gid) = guild_id {
        if let Some(t) = store.get_server_translation(gid.to_string()).await {
            return t;
        }
    }
    DEFAULT_TRANSLATION.to_string()
}

pub async fn calc_translation(
    user_choice: Option<&str>,
    user_id: &str,
    guild_id: Option<&str>,
    store: &dyn Store,
) -> String {
    calc_translation_with(user_choice, user_id, guild_id, store, is_valid_translation).await
}

/// Resolves the effective language from the priority chain: explicit choice
/// (if valid per `is_valid`) → user pref → server pref → default.
/// `is_valid` is dependency-injected so this stays testable without the
/// global language cache in `valid_cache` being populated.
pub async fn calc_lang_with(
    user_choice: Option<&str>,
    user_id: &str,
    guild_id: Option<&str>,
    store: &dyn Store,
    is_valid: impl Fn(&str) -> bool,
) -> String {
    if let Some(choice) = user_choice {
        if is_valid(choice) {
            return choice.to_string();
        }
    }
    if let Some(l) = store.get_user_language(user_id.to_string()).await {
        return l;
    }
    if let Some(gid) = guild_id {
        if let Some(l) = store.get_server_language(gid.to_string()).await {
            return l;
        }
    }
    DEFAULT_LANGUAGE.to_string()
}

pub async fn calc_lang(
    user_choice: Option<&str>,
    user_id: &str,
    guild_id: Option<&str>,
    store: &dyn Store,
) -> String {
    calc_lang_with(user_choice, user_id, guild_id, store, is_valid_language).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::fake::FakeStore;

    fn always_valid(_: &str) -> bool {
        true
    }

    fn never_valid(_: &str) -> bool {
        false
    }

    #[tokio::test]
    async fn explicit_valid_choice_wins() {
        let store = FakeStore::new()
            .with_user_translation("u1", "KJV")
            .with_server_translation("g1", "NIV");
        let result =
            calc_translation_with(Some("BSB"), "u1", Some("g1"), &store, always_valid).await;
        assert_eq!(result, "BSB");
    }

    #[tokio::test]
    async fn invalid_explicit_choice_falls_back_to_user_pref() {
        let store = FakeStore::new()
            .with_user_translation("u1", "KJV")
            .with_server_translation("g1", "NIV");
        let result =
            calc_translation_with(Some("bogus"), "u1", Some("g1"), &store, never_valid).await;
        assert_eq!(result, "KJV");
    }

    #[tokio::test]
    async fn falls_back_to_server_pref_when_no_user_pref() {
        let store = FakeStore::new().with_server_translation("g1", "NIV");
        let result = calc_translation_with(None, "u1", Some("g1"), &store, never_valid).await;
        assert_eq!(result, "NIV");
    }

    #[tokio::test]
    async fn falls_back_to_default_when_nothing_set() {
        let store = FakeStore::new();
        let result = calc_translation_with(None, "u1", None, &store, never_valid).await;
        assert_eq!(result, DEFAULT_TRANSLATION);
    }

    #[tokio::test]
    async fn falls_back_to_default_when_no_guild() {
        let store = FakeStore::new();
        let result = calc_translation_with(None, "u1", None, &store, never_valid).await;
        assert_eq!(result, DEFAULT_TRANSLATION);
    }

    #[tokio::test]
    async fn lang_explicit_valid_choice_wins() {
        let store = FakeStore::new().with_user_language("u1", "es");
        let result = calc_lang_with(Some("fr"), "u1", None, &store, always_valid).await;
        assert_eq!(result, "fr");
    }

    #[tokio::test]
    async fn lang_falls_back_to_user_pref() {
        let store = FakeStore::new().with_user_language("u1", "es");
        let result = calc_lang_with(None, "u1", None, &store, never_valid).await;
        assert_eq!(result, "es");
    }

    #[tokio::test]
    async fn lang_falls_back_to_server_pref() {
        let store = FakeStore::new().with_server_language("g1", "de");
        let result = calc_lang_with(None, "u1", Some("g1"), &store, never_valid).await;
        assert_eq!(result, "de");
    }

    #[tokio::test]
    async fn lang_falls_back_to_default() {
        let store = FakeStore::new();
        let result = calc_lang_with(None, "u1", None, &store, never_valid).await;
        assert_eq!(result, DEFAULT_LANGUAGE);
    }
}
