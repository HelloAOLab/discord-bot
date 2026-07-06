use poise::CreateReply;
use serde_json::Value;

pub fn embed_json(reply: &CreateReply, index: usize) -> Value {
    serde_json::to_value(&reply.embeds[index]).unwrap()
}

pub fn embed_title(reply: &CreateReply, index: usize) -> String {
    embed_json(reply, index)["title"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

pub fn embed_description(reply: &CreateReply, index: usize) -> String {
    embed_json(reply, index)["description"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

pub fn embed_footer_text(reply: &CreateReply, index: usize) -> String {
    embed_json(reply, index)["footer"]["text"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}
