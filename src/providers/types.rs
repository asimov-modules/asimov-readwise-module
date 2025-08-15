// This is free and unencumbered software released into the public domain.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReadwiseType {
    Highlights,
}

impl ReadwiseType {
    pub const HIGHLIGHTS_ID: &'static str = "readwise-highlights";

    pub fn as_str(&self) -> &'static str {
        match self {
            ReadwiseType::Highlights => "readwise-highlights",
        }
    }
}
