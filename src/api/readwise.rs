// This is free and unencumbered software released into the public domain.

use crate::api::types::{BookListResponse, HighlightsResponse};
use anyhow::Result;

use ureq;

#[derive(Debug, Clone)]
pub struct ReadwiseConfig {
    pub base_url: String,
    pub access_token: String,
}

impl ReadwiseConfig {
    pub fn new(access_token: String) -> Self {
        Self {
            base_url: "https://readwise.io/api/v2".to_string(),
            access_token,
        }
    }

    pub fn endpoint_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

pub struct ReadwiseClient {
    config: ReadwiseConfig,
}

impl ReadwiseClient {
    pub fn new(config: ReadwiseConfig) -> Result<Self> {
        Ok(Self { config })
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

    pub fn fetch_highlights(
        &mut self,
        page_size: Option<usize>,
        page: Option<usize>,
    ) -> Result<HighlightsResponse> {
        let url = self.build_url_with_params("/highlights/", page_size, page);

        let mut response = ureq::get(&url)
            .header("Authorization", &self.auth_header())
            .call()
            .map_err(|e| {
                if e.to_string().contains("429") {
                    anyhow::anyhow!("Rate limit exceeded (429). Please wait a minute before trying again. Consider using smaller page sizes to avoid hitting limits.")
                } else {
                    e.into()
                }
            })?;
        let response_body: HighlightsResponse =
            serde_json::from_str(&response.body_mut().read_to_string()?)?;
        Ok(response_body)
    }

    pub fn fetch_booklist(
        &mut self,
        page_size: Option<usize>,
        page: Option<usize>,
    ) -> Result<BookListResponse> {
        let url = self.build_url_with_params("/books/", page_size, page);

        let mut response = ureq::get(&url)
            .header("Authorization", &self.auth_header())
            .call()
            .map_err(|e| {
                if e.to_string().contains("429") {
                    anyhow::anyhow!("Rate limit exceeded (429). Please wait a minute before trying again. Consider using smaller page sizes to avoid hitting limits.")
                } else {
                    e.into()
                }
            })?;
        let response_body: BookListResponse =
            serde_json::from_str(&response.body_mut().read_to_string()?)?;
        Ok(response_body)
    }

    pub fn fetch_highlight_tags(&mut self) -> Result<Vec<serde_json::Value>> {
        let mut all_tags = std::collections::HashMap::new();
        let mut page = 1;
        let page_size = 100;

        loop {
            let highlights = self.fetch_highlights(Some(page_size), Some(page))
            .map_err(|e| {
                if e.to_string().contains("429") {
                    anyhow::anyhow!("Rate limit exceeded while fetching highlights (429). Please wait a minute before trying again.")
                } else {
                    e
                }
            })?;

            if let Some(results) = highlights.results {
                if results.is_empty() {
                    break;
                }

                let has_next = highlights.next.is_some();

                for highlight in results {
                    if let Some(highlight_id) = highlight.id {
                        let tags_url =
                            self.endpoint_url(&format!("/highlights/{}/tags", highlight_id));

                        let mut response = ureq::get(&tags_url)
                            .header("Authorization", &self.auth_header())
                            .call()
                            .map_err(|e| {
                                if e.to_string().contains("429") {
                                    anyhow::anyhow!("Rate limit exceeded while fetching tags (429). Please wait a minute before trying again.")
                                } else {
                                    e.into()
                                }
                            })?;

                        let response_body = response.body_mut().read_to_string()?;

                        let tags_data: serde_json::Value = serde_json::from_str(&response_body)?;

                        if let Some(tag_results) = tags_data["results"].as_array() {
                            for tag in tag_results {
                                if let (Some(name), Some(id)) =
                                    (tag["name"].as_str(), tag["id"].as_u64())
                                {
                                    all_tags.insert(
                                        name.to_string(),
                                        serde_json::json!({
                                            "name": name,
                                            "id": id
                                        }),
                                    );
                                }
                            }
                        }
                    }
                }

                if has_next {
                    page += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(all_tags.values().cloned().collect())
    }
}
