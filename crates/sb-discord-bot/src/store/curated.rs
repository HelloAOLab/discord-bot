use rand::seq::SliceRandom;

use crate::store::contract::BibleBooks;

pub struct VerseRef {
    pub book: BibleBooks,
    pub chapter: i64,
    pub verse: i64,
}

impl VerseRef {
    const fn new(book: BibleBooks, chapter: i64, verse: i64) -> Self {
        Self { book, chapter, verse }
    }
}

const CURATED_VERSES: &[VerseRef] = &[
    VerseRef::new(BibleBooks::John, 3, 16),
    VerseRef::new(BibleBooks::John, 3, 17),
    VerseRef::new(BibleBooks::Romans, 8, 28),
    VerseRef::new(BibleBooks::Romans, 8, 38),
    VerseRef::new(BibleBooks::Romans, 8, 39),
    VerseRef::new(BibleBooks::Psalms, 23, 1),
    VerseRef::new(BibleBooks::Psalms, 23, 4),
    VerseRef::new(BibleBooks::Psalms, 46, 1),
    VerseRef::new(BibleBooks::Psalms, 91, 1),
    VerseRef::new(BibleBooks::Psalms, 119, 105),
    VerseRef::new(BibleBooks::Proverbs, 3, 5),
    VerseRef::new(BibleBooks::Proverbs, 3, 6),
    VerseRef::new(BibleBooks::Isaiah, 40, 31),
    VerseRef::new(BibleBooks::Isaiah, 41, 10),
    VerseRef::new(BibleBooks::Jeremiah, 29, 11),
    VerseRef::new(BibleBooks::Matthew, 6, 33),
    VerseRef::new(BibleBooks::Matthew, 11, 28),
    VerseRef::new(BibleBooks::Matthew, 28, 19),
    VerseRef::new(BibleBooks::Matthew, 28, 20),
    VerseRef::new(BibleBooks::Philippians, 4, 6),
    VerseRef::new(BibleBooks::Philippians, 4, 7),
    VerseRef::new(BibleBooks::Philippians, 4, 13),
    VerseRef::new(BibleBooks::Galatians, 5, 22),
    VerseRef::new(BibleBooks::Galatians, 5, 23),
    VerseRef::new(BibleBooks::Ephesians, 2, 8),
    VerseRef::new(BibleBooks::Ephesians, 2, 9),
    VerseRef::new(BibleBooks::Ephesians, 6, 10),
    VerseRef::new(BibleBooks::Romans, 3, 23),
    VerseRef::new(BibleBooks::Romans, 6, 23),
    VerseRef::new(BibleBooks::Romans, 10, 9),
    VerseRef::new(BibleBooks::Hebrews, 11, 1),
    VerseRef::new(BibleBooks::Hebrews, 12, 1),
    VerseRef::new(BibleBooks::James, 1, 2),
    VerseRef::new(BibleBooks::James, 1, 3),
    VerseRef::new(BibleBooks::Peter1, 5, 7),
    VerseRef::new(BibleBooks::John1, 1, 9),
    VerseRef::new(BibleBooks::John1, 4, 8),
    VerseRef::new(BibleBooks::Revelation, 21, 4),
    VerseRef::new(BibleBooks::Genesis, 1, 1),
    VerseRef::new(BibleBooks::Joshua, 1, 9),
];

pub fn random_curated_verse() -> &'static VerseRef {
    CURATED_VERSES
        .choose(&mut rand::thread_rng())
        .expect("curated pool is non-empty")
}
