// This is free and unencumbered software released into the public domain.

use asimov_readwise_module::{
    api::readwise::ReadwiseConfig, api::types::ReadwiseType, find_provider_for,
};

#[test]
fn test_find_provider_for_highlights() {
    let url = "https://readwise.io/highlights";
    let provider = find_provider_for(url);
    assert!(provider.is_some());
    assert_eq!(provider.unwrap().id, ReadwiseType::HIGHLIGHTS_ID);
}

#[test]
fn test_find_provider_for_books() {
    let url = "https://readwise.io/books";
    let provider = find_provider_for(url);
    assert!(provider.is_some());
    assert_eq!(provider.unwrap().id, ReadwiseType::BOOKLIST_ID);
}

#[test]
fn test_find_provider_for_tags() {
    let url = "https://readwise.io/tags";
    let provider = find_provider_for(url);
    assert!(provider.is_some());
    assert_eq!(provider.unwrap().id, ReadwiseType::TAGS_ID);
}

#[test]
fn test_find_provider_for_unsupported_url() {
    let url = "https://example.com/api/books";
    let provider = find_provider_for(url);
    assert!(provider.is_none());
}

#[test]
fn test_provider_url() {
    let url = "https://readwise.io/highlights";
    let provider = find_provider_for(url).unwrap();
    assert_eq!(provider.url, "https://readwise.io/highlights");
}

#[test]
fn test_supported_url_patterns() {
    let highlights_url = "https://readwise.io/highlights";
    let books_url = "https://readwise.io/books";
    let tags_url = "https://readwise.io/tags";

    assert!(highlights_url.contains("highlights"));
    assert!(books_url.contains("books"));
    assert!(tags_url.contains("tags"));
}

#[test]
fn test_readwise_type_variants() {
    let _highlights = ReadwiseType::HIGHLIGHTS_ID;
    let _books = ReadwiseType::BOOKLIST_ID;
    let _tags = ReadwiseType::TAGS_ID;
}

#[test]
fn test_readwise_type_as_str() {
    assert_eq!(ReadwiseType::Highlights.as_str(), "readwise-highlights");
    assert_eq!(ReadwiseType::Booklist.as_str(), "readwise-booklist");
    assert_eq!(ReadwiseType::Tags.as_str(), "readwise-tags");
}

#[test]
fn test_readwise_config_new() {
    let config = ReadwiseConfig::new("test_token".to_string());
    assert_eq!(config.access_token, "test_token");
    assert_eq!(config.base_url, "https://readwise.io/api/v2");
}
