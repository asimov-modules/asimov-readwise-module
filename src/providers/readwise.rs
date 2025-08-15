// This is free and unencumbered software released into the public domain.

use crate::providers::provider::Provider;
use crate::providers::types::ReadwiseType;

pub static READWISE_HIGHLIGHTS: Provider = Provider {
    id: ReadwiseType::HIGHLIGHTS_ID,
    brand: "Readwise",
    url: "https://readwise.io/api/v2/highlights/",
};

pub static URL_PREFIX_TO_PROVIDER: [(&str, &'static Provider); 1] = [
    ("https://readwise.io/api/v2/highlights/", &READWISE_HIGHLIGHTS),
];