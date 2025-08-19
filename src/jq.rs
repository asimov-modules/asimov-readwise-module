// This is free and unencumbered software released into the public domain.
pub use ::jq::*;

pub fn readwise() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/readwise.jq").parse().unwrap())
}

pub fn books() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/books.jq").parse().unwrap())
}

pub fn tags() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/tags.jq").parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_readwise_jq_compilation() {
        let _filter = readwise();
    }

    #[test]
    fn test_books_jq_compilation() {
        let _filter = books();
    }

    #[test]
    fn test_tags_jq_compilation() {
        let _filter = tags();
    }

    #[test]
    fn test_readwise_jq_with_sample_data() {
        let filter = readwise();
        let sample_data = json!({
            "count": 2,
            "results": [
                {
                    "id": 123,
                    "text": "Sample highlight text",
                    "note": "Sample note",
                    "location": 42,
                    "location_type": "page",
                    "highlighted_at": "2024-01-01T00:00:00Z",
                    "updated": "2024-01-01T00:00:00Z"
                }
            ]
        });

        let result = filter.filter_json(sample_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_books_jq_with_sample_data() {
        let filter = books();
        let sample_data = json!({
            "count": 1,
            "results": [
                {
                    "id": 456,
                    "title": "Sample Book",
                    "author": "Sample Author"
                }
            ]
        });

        let result = filter.filter_json(sample_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tags_jq_with_sample_data() {
        let filter = tags();
        let sample_data = json!([
            {
                "id": 789,
                "name": "Sample Tag",
                "updated": 1234567890
            }
        ]);

        let result = filter.filter_json(sample_data);
        assert!(result.is_ok());
    }
}
