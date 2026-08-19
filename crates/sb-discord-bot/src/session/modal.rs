/// The "session builder" modal shown by `/launchsession`. Passage/schedule
/// only — translation and language are always resolved per-subscriber at
/// link-generation time, so they aren't asked for here.
#[derive(Debug, poise::Modal)]
#[name = "Launch a Session"]
pub struct LaunchSessionModal {
    #[name = "Book"]
    #[placeholder = "e.g. John"]
    pub book: String,
    #[name = "Chapter"]
    #[placeholder = "e.g. 3"]
    pub chapter: String,
    #[name = "When"]
    #[placeholder = "\"now\" or \"YYYY-MM-DD HH:MM\" (UTC)"]
    pub when: String,
    #[name = "Remind subscribers this many minutes before"]
    #[placeholder = "optional, e.g. 10"]
    pub reminder_minutes: Option<String>,
}
