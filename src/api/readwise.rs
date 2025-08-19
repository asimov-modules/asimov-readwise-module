// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use crate::api::types::types::{
    Book, BookListResponse, Highlight, HighlightRequest, HighlightsResponse, ReadwiseConfig,
    SimpleTag, SimpleTagsResponse, Tag, TagsResponse,
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

    pub fn endpoint_url(&self, path: &str) -> String {
        self.config.endpoint_url(path)
    }

    fn build_url_with_params(
        &self,
        path: &str,
        page_size: Option<usize>,
        page: Option<usize>,
    ) -> String {
        let mut url = self.endpoint_url(path);
        let mut params = vec![];

        if let Some(size) = page_size {
            params.push(format!("page_size={}", size));
        }
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }

    pub async fn fetch_highlights(
        &self,
        page_size: Option<usize>,
        page: Option<usize>,
    ) -> Result<HighlightsResponse> {
        let url = self.build_url_with_params("/highlights/", page_size, page);
        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        let response_text = response.text().await?;
        let highlights: HighlightsResponse = serde_json::from_str(&response_text)?;
        Ok(highlights)
    }

    pub async fn fetch_booklist(
        &self,
        page_size: Option<usize>,
        page: Option<usize>,
    ) -> Result<BookListResponse> {
        let url = self.build_url_with_params("/books/", page_size, page);
        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        let response_text = response.text().await?;
        let booklist: BookListResponse = serde_json::from_str(&response_text)?;
        Ok(booklist)
    }

    pub async fn fetch_highlight_tags(&self) -> Result<Vec<serde_json::Value>> {
        let highlights = self.fetch_highlights(None, None).await?;

        let mut tags = std::collections::HashMap::new();

        if let Some(results) = highlights.results {
            for highlight in results {
                if let Some(highlight_id) = highlight.id {
                    let tags_url = self.endpoint_url(&format!("/highlights/{}/tags", highlight_id));

                    let response = self
                        .client
                        .get(&tags_url)
                        .header("Authorization", self.auth_header())
                        .send()
                        .await?;

                    let tags_response = response.text().await?;

                    if let Ok(tags_data) = serde_json::from_str::<serde_json::Value>(&tags_response)
                    {
                        if let Some(tag_results) =
                            tags_data.get("results").and_then(|r| r.as_array())
                        {
                            for tag in tag_results {
                                if let (Some(name), Some(id)) = (tag.get("name"), tag.get("id")) {
                                    if let (Some(name_str), Some(id_num)) =
                                        (name.as_str(), id.as_u64())
                                    {
                                        tags.insert(
                                            name_str.to_string(),
                                            serde_json::json!({
                                                "name": name_str,
                                                "updated": id_num
                                            }),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(tags.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::types::ReadwiseConfig;

    #[test]
    fn test_readwise_config_new() {
        let config = ReadwiseConfig::new("test_token".to_string());
        assert_eq!(config.access_token, "test_token");
        assert_eq!(config.base_url, "https://readwise.io/api/v2");
    }

    #[test]
    fn test_auth_header() {
        let config = ReadwiseConfig::new("test_token".to_string());
        let client = ReadwiseClient::new(config).unwrap();
        let header = client.auth_header();
        assert_eq!(header, "Token test_token");
    }
}
