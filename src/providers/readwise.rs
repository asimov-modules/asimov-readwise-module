// This is free and unencumbered software released into the public domain.

use crate::providers::provider::Provider;
use crate::providers::types::ReadwiseType;

pub static READWISE_HIGHLIGHTS: Provider = Provider {
    id: ReadwiseType::HIGHLIGHTS_ID,
    brand: "Readwise",
    url: "https://readwise.io/highlights",
};

pub static READWISE_BOOKLIST: Provider = Provider {
    id: ReadwiseType::BOOKLIST_ID,
    brand: "Readwise",
    url: "https://readwise.io/books",
};

pub static READWISE_TAGS: Provider = Provider {
    id: ReadwiseType::TAGS_ID,
    brand: "Readwise",
    url: "https://readwise.io/tags",
};

pub static URL_PREFIX_TO_PROVIDER: [(&str, &'static Provider); 3] = [
    ("https://readwise.io/highlights", &READWISE_HIGHLIGHTS),
    ("https://readwise.io/books", &READWISE_BOOKLIST),
    ("https://readwise.io/tags", &READWISE_TAGS),
];
