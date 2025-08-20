// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug, Default)]
pub struct Provider {
    pub id: &'static str,
    pub url: &'static str,
}
