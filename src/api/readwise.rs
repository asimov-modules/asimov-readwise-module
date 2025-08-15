// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadwiseConfig {
    pub base_url: String,
    pub access_token: String,
    pub timeout: u64,
    pub rate_limit: u32,
}

impl Default for ReadwiseConfig {
    fn default() -> Self {
        Self {
            base_url: "https://readwise.io/api/v2".to_string(),
            access_token: String::new(),
            timeout: 30,
            rate_limit: 20,
        }
    }
}

impl ReadwiseConfig {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            ..Default::default()
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn endpoint_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightRequest {
    pub text: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub source_url: Option<String>,
    pub source_type: Option<String>,
    pub category: Option<String>,
    pub note: Option<String>,
    pub location: Option<i32>,
    pub location_type: Option<String>,
    pub highlighted_at: Option<String>,
    pub highlight_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightResponse {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub source: Option<String>,
    pub num_highlights: Option<u32>,
    pub last_highlight_at: Option<String>,
    pub updated: Option<String>,
    pub cover_image_url: Option<String>,
    pub highlights_url: Option<String>,
    pub source_url: Option<String>,
    pub modified_highlights: Option<Vec<u64>>,
    pub text: Option<String>,
    pub source_type: Option<String>,
    pub note: Option<String>,
    pub location: Option<i32>,
    pub location_type: Option<String>,
    pub highlighted_at: Option<String>,
    pub highlight_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightsResponse {
    pub count: Option<u32>,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Option<Vec<HighlightResponse>>,
}
