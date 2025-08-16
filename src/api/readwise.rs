// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use crate::api::types::types::{
    Book, BookListResponse, Highlight, HighlightRequest, HighlightsResponse, ReadwiseConfig,
};
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

pub struct ReadwiseClient {
    config: ReadwiseConfig,
    client: Client,
}

impl ReadwiseClient {
    pub fn new(config: ReadwiseConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()?;

        Ok(Self { config, client })
    }

    fn auth_header(&self) -> String {
        format!("Token {}", self.config.access_token)
    }

    pub async fn fetch_highlights(
        &self,
        query_params: Option<HashMap<String, String>>,
    ) -> Result<HighlightsResponse> {
        let url = self.config.endpoint_url("/highlights/");

        let params: Vec<(String, String)> = query_params.unwrap_or_default().into_iter().collect();

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .query(&params)
            .send()
            .await?;

        let response_text = response.text().await?;
        let highlights: HighlightsResponse = serde_json::from_str(&response_text)?;
        Ok(highlights)
    }

    pub async fn fetch_highlights_from_url(&self, url: &str) -> Result<HighlightsResponse> {
        let response = self
            .client
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        let response_text = response.text().await?;
        let highlights: HighlightsResponse = serde_json::from_str(&response_text)?;
        Ok(highlights)
    }

    pub async fn fetch_booklist_from_url(&self, url: &str) -> Result<BookListResponse> {
        let response = self
            .client
            .get(url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        let response_text = response.text().await?;
        let booklist: BookListResponse = serde_json::from_str(&response_text)?;
        Ok(booklist)
    }
}
