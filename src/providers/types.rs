// This is free and unencumbered software released into the public domain.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReadwiseType {
    Highlights,
    Booklist,
    Tags,
}

impl ReadwiseType {
    pub const HIGHLIGHTS_ID: &'static str = "readwise-highlights";
    pub const BOOKLIST_ID: &'static str = "readwise-booklist";
    pub const TAGS_ID: &'static str = "readwise-tags";

    pub fn as_str(&self) -> &'static str {
        match self {
            ReadwiseType::Highlights => Self::HIGHLIGHTS_ID,
            ReadwiseType::Booklist => Self::BOOKLIST_ID,
            ReadwiseType::Tags => Self::TAGS_ID,
        }
    }
}
