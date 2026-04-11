use crate::store::{
    store::Store,
    valid_cache::{is_valid_language, is_valid_translation},
};

const DEFAULT_TRANSLATION: &str = "BSB";
const DEFAULT_LANGUAGE: &str = "en";

pub async fn calc_translation(
    user_choice: Option<&str>,
    user_id: &str,
    guild_id: Option<&str>,
    store: &dyn Store,
) -> String {
    if let Some(choice) = user_choice {
        if is_valid_translation(choice) {
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

pub async fn calc_lang(
    user_choice: Option<&str>,
    user_id: &str,
    guild_id: Option<&str>,
    store: &dyn Store,
) -> String {
    if let Some(choice) = user_choice {
        if is_valid_language(choice) {
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
