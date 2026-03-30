use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter};

use crate::util::format::getPassageURL;

pub struct Data {
    pub store: std::sync::Arc<dyn crate::store::store::Store>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    slash_command,
    prefix_command,
    category = "General",
    description_localized("en-US", "Get a list of available commands")
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::default()
                .title("Help")
                .description("Seed Bible Bot Commands:\n\n`/open` - Open a passage in Seed Bible\n`/verse` - Get a specific verse\n`/chapter` — Get a full chapter\n`/random` — Random verse from curated pool\n`/truerandom` — Any verse in the Bible\n`/votd` — Verse of the day\n`/translations` — List available translations\n`/settranslation` — Set your personal default translation")
                .color(Colour::from_rgb(178, 255, 237))
                .footer(CreateEmbedFooter::new("(admin) /setservertranslation, /setseedbiblelinks, /setdailychannel, /setdailyverserole"))
        ],
        ..Default::default()
    })
    .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Open a passage at the given location ")
)]
pub async fn open(
    ctx: Context<'_>,
    #[description = "Book"] book: String,
    #[description = "Chapter"] chapter: String,
    #[description = "Translation"] translation: String,
    #[description = "Language"] langauge: String,
) -> Result<(), Error> {
    ctx.send(CreateReply {
        embeds: vec![CreateEmbed::default().title("Open").url(getPassageURL(
            &book,
            &chapter,
            &translation,
            &langauge,
        ))],
        ..Default::default()
    })
    .await?;
    Ok(())
}

pub fn all_commands() -> Vec<poise::Command<Data, Error>> {
    vec![help(), open()]
}
