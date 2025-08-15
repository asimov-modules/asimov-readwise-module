// This is free and unencumbered software released into the public domain.

#![forbid(unsafe_code)]

pub mod api;
pub mod jq;
pub mod providers;

pub use providers::Provider;

pub fn find_engine_for(url: impl AsRef<str>) -> Option<&'static Provider> {
    let url = url.as_ref();
    for (url_pattern, engine) in providers::readwise::URL_PREFIX_TO_PROVIDER.iter().rev() {
        if url.starts_with(url_pattern) {
            return Some(engine);
        }
    }
    None // not found
}