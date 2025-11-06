// This is free and unencumbered software released into the public domain.

#![forbid(unsafe_code)]

pub mod api;
pub mod jq;
pub mod output;
pub mod providers;

pub use providers::Provider;

pub fn find_provider_for(url: impl AsRef<str>) -> Option<&'static Provider> {
    let url = url.as_ref();
    // Iterate in reverse to ensure that more specific URL patterns are matched first.
    // This relies on URL_PREFIX_TO_PROVIDER being ordered from least to most specific.
    for (url_pattern, provider) in providers::readwise::URL_PREFIX_TO_PROVIDER.iter().rev() {
        if url.starts_with(url_pattern) {
            return Some(provider);
        }
    }
    None
}
