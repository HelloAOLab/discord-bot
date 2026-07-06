use std::sync::OnceLock;

use regex::Regex;

use crate::store::contract::BibleBooks;

/// A Bible reference detected in free-form text.
/// `verses` is `None` when the reference names a whole chapter with no verse part.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedReference {
    pub book: BibleBooks,
    pub chapter: i64,
    pub verses: Option<Vec<i64>>,
}

/// Caps how many references a single message can trigger, so one message can't
/// fan out into a large burst of API calls and replies.
const MAX_REFERENCES_PER_MESSAGE: usize = 5;

/// Lowercase alias -> book. Includes each book's full name plus common abbreviations.
static ALIASES: &[(&str, BibleBooks)] = &[
    ("genesis", BibleBooks::Genesis),
    ("gen", BibleBooks::Genesis),
    ("ge", BibleBooks::Genesis),
    ("gn", BibleBooks::Genesis),
    ("exodus", BibleBooks::Exodus),
    ("exod", BibleBooks::Exodus),
    ("exo", BibleBooks::Exodus),
    ("ex", BibleBooks::Exodus),
    ("leviticus", BibleBooks::Leviticus),
    ("lev", BibleBooks::Leviticus),
    ("le", BibleBooks::Leviticus),
    ("lv", BibleBooks::Leviticus),
    ("numbers", BibleBooks::Numbers),
    ("num", BibleBooks::Numbers),
    ("nu", BibleBooks::Numbers),
    ("nm", BibleBooks::Numbers),
    ("deuteronomy", BibleBooks::Deuteronomy),
    ("deut", BibleBooks::Deuteronomy),
    ("deu", BibleBooks::Deuteronomy),
    ("dt", BibleBooks::Deuteronomy),
    ("joshua", BibleBooks::Joshua),
    ("josh", BibleBooks::Joshua),
    ("jos", BibleBooks::Joshua),
    ("judges", BibleBooks::Judges),
    ("judg", BibleBooks::Judges),
    ("jdg", BibleBooks::Judges),
    ("jgs", BibleBooks::Judges),
    ("ruth", BibleBooks::Ruth),
    ("rth", BibleBooks::Ruth),
    ("ru", BibleBooks::Ruth),
    ("1 samuel", BibleBooks::Samuel1),
    ("1st samuel", BibleBooks::Samuel1),
    ("1 sam", BibleBooks::Samuel1),
    ("1sa", BibleBooks::Samuel1),
    ("1sam", BibleBooks::Samuel1),
    ("2 samuel", BibleBooks::Samuel2),
    ("2nd samuel", BibleBooks::Samuel2),
    ("2 sam", BibleBooks::Samuel2),
    ("2sa", BibleBooks::Samuel2),
    ("2sam", BibleBooks::Samuel2),
    ("1 kings", BibleBooks::Kings1),
    ("1st kings", BibleBooks::Kings1),
    ("1 kgs", BibleBooks::Kings1),
    ("1ki", BibleBooks::Kings1),
    ("1kgs", BibleBooks::Kings1),
    ("2 kings", BibleBooks::Kings2),
    ("2nd kings", BibleBooks::Kings2),
    ("2 kgs", BibleBooks::Kings2),
    ("2ki", BibleBooks::Kings2),
    ("2kgs", BibleBooks::Kings2),
    ("1 chronicles", BibleBooks::Chronicles1),
    ("1st chronicles", BibleBooks::Chronicles1),
    ("1 chron", BibleBooks::Chronicles1),
    ("1 chr", BibleBooks::Chronicles1),
    ("1ch", BibleBooks::Chronicles1),
    ("2 chronicles", BibleBooks::Chronicles2),
    ("2nd chronicles", BibleBooks::Chronicles2),
    ("2 chron", BibleBooks::Chronicles2),
    ("2 chr", BibleBooks::Chronicles2),
    ("2ch", BibleBooks::Chronicles2),
    ("ezra", BibleBooks::Ezra),
    ("ezr", BibleBooks::Ezra),
    ("nehemiah", BibleBooks::Nehemiah),
    ("neh", BibleBooks::Nehemiah),
    ("esther", BibleBooks::Esther),
    ("esth", BibleBooks::Esther),
    ("est", BibleBooks::Esther),
    ("job", BibleBooks::Job),
    ("jb", BibleBooks::Job),
    ("psalms", BibleBooks::Psalms),
    ("psalm", BibleBooks::Psalms),
    ("pslm", BibleBooks::Psalms),
    ("psa", BibleBooks::Psalms),
    ("psm", BibleBooks::Psalms),
    ("ps", BibleBooks::Psalms),
    ("proverbs", BibleBooks::Proverbs),
    ("prov", BibleBooks::Proverbs),
    ("pro", BibleBooks::Proverbs),
    ("prv", BibleBooks::Proverbs),
    ("ecclesiastes", BibleBooks::Ecclesiastes),
    ("eccles", BibleBooks::Ecclesiastes),
    ("eccl", BibleBooks::Ecclesiastes),
    ("ecc", BibleBooks::Ecclesiastes),
    ("song of solomon", BibleBooks::SongOfSolomon),
    ("song of songs", BibleBooks::SongOfSolomon),
    ("song", BibleBooks::SongOfSolomon),
    ("sos", BibleBooks::SongOfSolomon),
    ("isaiah", BibleBooks::Isaiah),
    ("isa", BibleBooks::Isaiah),
    ("jeremiah", BibleBooks::Jeremiah),
    ("jer", BibleBooks::Jeremiah),
    ("lamentations", BibleBooks::Lamentations),
    ("lam", BibleBooks::Lamentations),
    ("ezekiel", BibleBooks::Ezekiel),
    ("ezek", BibleBooks::Ezekiel),
    ("eze", BibleBooks::Ezekiel),
    ("ezk", BibleBooks::Ezekiel),
    ("daniel", BibleBooks::Daniel),
    ("dan", BibleBooks::Daniel),
    ("hosea", BibleBooks::Hosea),
    ("hos", BibleBooks::Hosea),
    ("joel", BibleBooks::Joel),
    ("jl", BibleBooks::Joel),
    ("amos", BibleBooks::Amos),
    ("am", BibleBooks::Amos),
    ("obadiah", BibleBooks::Obadiah),
    ("obad", BibleBooks::Obadiah),
    ("oba", BibleBooks::Obadiah),
    ("ob", BibleBooks::Obadiah),
    ("jonah", BibleBooks::Jonah),
    ("jnh", BibleBooks::Jonah),
    ("jon", BibleBooks::Jonah),
    ("micah", BibleBooks::Micah),
    ("mic", BibleBooks::Micah),
    ("nahum", BibleBooks::Nahum),
    ("nah", BibleBooks::Nahum),
    ("nam", BibleBooks::Nahum),
    ("habakkuk", BibleBooks::Habakkuk),
    ("hab", BibleBooks::Habakkuk),
    ("zephaniah", BibleBooks::Zephaniah),
    ("zeph", BibleBooks::Zephaniah),
    ("zep", BibleBooks::Zephaniah),
    ("haggai", BibleBooks::Haggai),
    ("hag", BibleBooks::Haggai),
    ("zechariah", BibleBooks::Zechariah),
    ("zech", BibleBooks::Zechariah),
    ("zec", BibleBooks::Zechariah),
    ("malachi", BibleBooks::Malachi),
    ("mal", BibleBooks::Malachi),
    ("matthew", BibleBooks::Matthew),
    ("matt", BibleBooks::Matthew),
    ("mt", BibleBooks::Matthew),
    ("mark", BibleBooks::Mark),
    ("mrk", BibleBooks::Mark),
    ("mk", BibleBooks::Mark),
    ("luke", BibleBooks::Luke),
    ("lk", BibleBooks::Luke),
    ("john", BibleBooks::John),
    ("jhn", BibleBooks::John),
    ("jn", BibleBooks::John),
    ("acts", BibleBooks::Acts),
    ("act", BibleBooks::Acts),
    ("romans", BibleBooks::Romans),
    ("rom", BibleBooks::Romans),
    ("1 corinthians", BibleBooks::Corinthians1),
    ("1st corinthians", BibleBooks::Corinthians1),
    ("1 cor", BibleBooks::Corinthians1),
    ("1co", BibleBooks::Corinthians1),
    ("2 corinthians", BibleBooks::Corinthians2),
    ("2nd corinthians", BibleBooks::Corinthians2),
    ("2 cor", BibleBooks::Corinthians2),
    ("2co", BibleBooks::Corinthians2),
    ("galatians", BibleBooks::Galatians),
    ("gal", BibleBooks::Galatians),
    ("ephesians", BibleBooks::Ephesians),
    ("eph", BibleBooks::Ephesians),
    ("philippians", BibleBooks::Philippians),
    ("phil", BibleBooks::Philippians),
    ("php", BibleBooks::Philippians),
    ("colossians", BibleBooks::Colossians),
    ("col", BibleBooks::Colossians),
    ("1 thessalonians", BibleBooks::Thessalonians1),
    ("1st thessalonians", BibleBooks::Thessalonians1),
    ("1 thess", BibleBooks::Thessalonians1),
    ("1 thes", BibleBooks::Thessalonians1),
    ("1th", BibleBooks::Thessalonians1),
    ("2 thessalonians", BibleBooks::Thessalonians2),
    ("2nd thessalonians", BibleBooks::Thessalonians2),
    ("2 thess", BibleBooks::Thessalonians2),
    ("2 thes", BibleBooks::Thessalonians2),
    ("2th", BibleBooks::Thessalonians2),
    ("1 timothy", BibleBooks::Timothy1),
    ("1st timothy", BibleBooks::Timothy1),
    ("1 tim", BibleBooks::Timothy1),
    ("1ti", BibleBooks::Timothy1),
    ("2 timothy", BibleBooks::Timothy2),
    ("2nd timothy", BibleBooks::Timothy2),
    ("2 tim", BibleBooks::Timothy2),
    ("2ti", BibleBooks::Timothy2),
    ("titus", BibleBooks::Titus),
    ("tit", BibleBooks::Titus),
    ("philemon", BibleBooks::Philemon),
    ("phlm", BibleBooks::Philemon),
    ("phm", BibleBooks::Philemon),
    ("hebrews", BibleBooks::Hebrews),
    ("heb", BibleBooks::Hebrews),
    ("james", BibleBooks::James),
    ("jas", BibleBooks::James),
    ("1 peter", BibleBooks::Peter1),
    ("1st peter", BibleBooks::Peter1),
    ("1 pet", BibleBooks::Peter1),
    ("1pe", BibleBooks::Peter1),
    ("2 peter", BibleBooks::Peter2),
    ("2nd peter", BibleBooks::Peter2),
    ("2 pet", BibleBooks::Peter2),
    ("2pe", BibleBooks::Peter2),
    ("1 john", BibleBooks::John1),
    ("1st john", BibleBooks::John1),
    ("1 jn", BibleBooks::John1),
    ("1jo", BibleBooks::John1),
    ("2 john", BibleBooks::John2),
    ("2nd john", BibleBooks::John2),
    ("2 jn", BibleBooks::John2),
    ("2jo", BibleBooks::John2),
    ("3 john", BibleBooks::John3),
    ("3rd john", BibleBooks::John3),
    ("3 jn", BibleBooks::John3),
    ("3jo", BibleBooks::John3),
    ("jude", BibleBooks::Jude),
    ("jde", BibleBooks::Jude),
    ("revelation", BibleBooks::Revelation),
    ("rev", BibleBooks::Revelation),
];

fn build_pattern() -> String {
    let mut aliases: Vec<&'static str> = ALIASES.iter().map(|(alias, _)| *alias).collect();
    // Longest alias first, so e.g. "song of solomon" wins over any shorter overlapping alias.
    aliases.sort_by_key(|a| std::cmp::Reverse(a.len()));
    let alternation = aliases
        .iter()
        .map(|a| regex::escape(a))
        .collect::<Vec<_>>()
        .join("|");
    format!(
        r"(?i)\b(?P<book>{})\.?\s+(?P<chapter>\d{{1,3}})(?:\s*:\s*(?P<verses>\d{{1,3}}(?:\s*-\s*\d{{1,3}})?(?:\s*,\s*\d{{1,3}}(?:\s*-\s*\d{{1,3}})?)*))?\b",
        alternation
    )
}

fn reference_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(&build_pattern()).expect("reference regex is valid"))
}

fn lookup_book(raw: &str) -> Option<BibleBooks> {
    let normalized = raw.to_lowercase();
    ALIASES
        .iter()
        .find(|(alias, _)| *alias == normalized)
        .map(|(_, book)| book.clone())
}

/// Expands a verse spec like "1-3, 5" into `[1, 2, 3, 5]`. Ignores malformed pieces.
fn parse_verse_spec(spec: &str) -> Vec<i64> {
    let mut verses = Vec::new();
    for part in spec.split(',') {
        let part = part.trim();
        if let Some((start, end)) = part.split_once('-') {
            if let (Ok(s), Ok(e)) = (start.trim().parse::<i64>(), end.trim().parse::<i64>())
                && s <= e
            {
                verses.extend(s..=e);
            }
        } else if let Ok(n) = part.parse::<i64>() {
            verses.push(n);
        }
    }
    verses
}

/// Finds recognisable Bible references in free-form text, e.g. "John 3:16",
/// "Romans 8:28-30", "Ps 23", "Genesis 1", or "John 1:1-3, 5". Returns at most
/// `MAX_REFERENCES_PER_MESSAGE` references, in the order they appear.
pub fn find_references(text: &str) -> Vec<ParsedReference> {
    let mut results = Vec::new();
    for caps in reference_regex().captures_iter(text) {
        if results.len() >= MAX_REFERENCES_PER_MESSAGE {
            break;
        }
        let Some(book) = caps.name("book").and_then(|m| lookup_book(m.as_str())) else {
            continue;
        };
        let Some(chapter) = caps
            .name("chapter")
            .and_then(|m| m.as_str().parse::<i64>().ok())
        else {
            continue;
        };
        let verses = caps
            .name("verses")
            .map(|m| parse_verse_spec(m.as_str()))
            .filter(|v| !v.is_empty());
        results.push(ParsedReference {
            book,
            chapter,
            verses,
        });
    }
    results
}

/// Renders a verse list back into compact notation, e.g. `[1, 2, 3, 5]` -> `"1-3, 5"`.
pub fn format_verse_numbers(verses: &[i64]) -> String {
    let mut parts = Vec::new();
    let mut i = 0;
    while i < verses.len() {
        let start = verses[i];
        let mut end = start;
        while i + 1 < verses.len() && verses[i + 1] == end + 1 {
            end = verses[i + 1];
            i += 1;
        }
        if start == end {
            parts.push(start.to_string());
        } else {
            parts.push(format!("{}-{}", start, end));
        }
        i += 1;
    }
    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_verse() {
        let refs = find_references("John 3:16");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].book, BibleBooks::John);
        assert_eq!(refs[0].chapter, 3);
        assert_eq!(refs[0].verses, Some(vec![16]));
    }

    #[test]
    fn parses_verse_range() {
        let refs = find_references("Romans 8:28-30");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].book, BibleBooks::Romans);
        assert_eq!(refs[0].chapter, 8);
        assert_eq!(refs[0].verses, Some(vec![28, 29, 30]));
    }

    #[test]
    fn parses_full_chapter_with_abbreviation() {
        let refs = find_references("Ps 23");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].book, BibleBooks::Psalms);
        assert_eq!(refs[0].chapter, 23);
        assert_eq!(refs[0].verses, None);
    }

    #[test]
    fn parses_full_chapter_with_full_name() {
        let refs = find_references("Genesis 1");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].book, BibleBooks::Genesis);
        assert_eq!(refs[0].chapter, 1);
        assert_eq!(refs[0].verses, None);
    }

    #[test]
    fn parses_non_contiguous_verses() {
        let refs = find_references("John 1:1-3, 5");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].book, BibleBooks::John);
        assert_eq!(refs[0].chapter, 1);
        assert_eq!(refs[0].verses, Some(vec![1, 2, 3, 5]));
    }

    #[test]
    fn ignores_plain_sentences() {
        assert!(find_references("just chatting about the weather today").is_empty());
    }

    #[test]
    fn formats_verse_numbers_back_to_compact_notation() {
        assert_eq!(format_verse_numbers(&[16]), "16");
        assert_eq!(format_verse_numbers(&[28, 29, 30]), "28-30");
        assert_eq!(format_verse_numbers(&[1, 2, 3, 5]), "1-3, 5");
    }
}
