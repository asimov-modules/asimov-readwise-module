// This is free and unencumbered software released into the public domain.

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Jsonl,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Json
    }
}

pub fn write_json_output<T: Serialize>(data: &T) -> Result<(), Box<dyn std::error::Error>> {
    let response = serde_json::to_string(data)?;
    println!("{}", response);
    Ok(())
}

pub fn write_jsonl_from_results<T: Serialize>(
    results: Option<&Vec<T>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(results) = results {
        for item in results {
            let line = serde_json::to_string(item)?;
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn write_jsonl_from_jsonld(
    json_ld: &serde_json::Value,
    provider_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::api::types::ReadwiseType;

    let items = match provider_id {
        ReadwiseType::HIGHLIGHTS_ID => json_ld
            .get("highlights")
            .and_then(|h| h.get("items"))
            .and_then(|i| i.as_array()),
        ReadwiseType::BOOKLIST_ID => json_ld
            .get("books")
            .and_then(|b| b.get("items"))
            .and_then(|i| i.as_array()),
        ReadwiseType::TAGS_ID => json_ld
            .get("tags")
            .and_then(|t| t.get("items"))
            .and_then(|i| i.as_array()),
        _ => None,
    };

    if let Some(items_array) = items {
        for item in items_array {
            let line = serde_json::to_string(item)?;
            println!("{}", line);
        }
    } else {
        eprintln!(
            "Warning: Expected items array not found in JSON-LD structure for JSONL output. No data will be produced."
        );
    }

    Ok(())
}
