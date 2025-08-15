// This is free and unencumbered software released into the public domain.
pub use ::jq::*;

#[cfg(feature = "std")]
pub fn readwise() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/readwise.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn readwise() -> JsonFilter {
    include_str!("jq/readwise.jq").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readwise_jq_compilation() {
        let _filter = readwise();
    }
}
