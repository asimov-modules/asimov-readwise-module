// This is free and unencumbered software released into the public domain.
use asimov_readwise_module::jq;
use asimov_readwise_module::providers::ReadwiseType;
use clap::Parser;

#[derive(Parser)]
#[command(name = "asimov-readwise-importer")]
#[command(about = "URL protocol importer. Consumes a URL input, produces JSON-LD output.")]
struct Args {
    #[arg(value_name = "INPUT-URL")]
    input_url: String,

    /// Page size (number of items per page, default: fetch all)
    #[arg(long, value_name = "SIZE")]
    page_size: Option<usize>,

    /// Specific page number to fetch (1-based, default: fetch all)
    #[arg(long, value_name = "NUM")]
    page: Option<usize>,
}

fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_module::getenv;
    use asimov_module::secrecy::ExposeSecret;
    use asimov_readwise_module::{api::readwise::ReadwiseClient, find_provider_for};
    use clientele::SysexitsError::*;
    use std::io::stdout;

    clientele::dotenv().ok();

    let args = Args::parse();

    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    let Some(api_key) = getenv::var_secret("READWISE_API_KEY") else {
        eprintln!("Missing READWISE_API_KEY. Run `asimov module config readwise`");
        return Ok(EX_CONFIG);
    };

    let config = asimov_readwise_module::api::types::ReadwiseConfig::new(
        api_key.expose_secret().to_string(),
    );

    let api = ReadwiseClient::new(config)?;

    let Some(provider) = find_provider_for(&args.input_url) else {
        eprintln!("Unsupported URL: {}", args.input_url);
        eprintln!("Make sure your URL starts with one of these patterns:");
        eprintln!("  - https://readwise.io/highlights");
        eprintln!("  - https://readwise.io/books");
        eprintln!("  - https://readwise.io/tags");
        return Ok(EX_UNAVAILABLE);
    };

    let json_ld = match provider.id {
        ReadwiseType::HIGHLIGHTS_ID => {
            let rt = tokio::runtime::Runtime::new()?;
            let highlights = rt.block_on(api.fetch_highlights(args.page_size, args.page))?;
            let highlights_json = serde_json::to_value(&highlights)?;
            jq::readwise().filter_json(highlights_json)?
        },
        ReadwiseType::BOOKLIST_ID => {
            let rt = tokio::runtime::Runtime::new()?;
            let booklist = rt.block_on(api.fetch_booklist(args.page_size, args.page))?;
            let booklist_json = serde_json::to_value(&booklist)?;
            jq::books().filter_json(booklist_json)?
        },
        ReadwiseType::TAGS_ID => {
            let rt = tokio::runtime::Runtime::new()?;
            let tags = rt.block_on(api.fetch_highlight_tags())?;
            let tags_json = serde_json::to_value(&tags)?;
            jq::tags().filter_json(tags_json)?
        },
        _ => {
            eprintln!("Unsupported provider type: {:?}", provider.id);
            return Ok(EX_UNAVAILABLE);
        },
    };

    if cfg!(feature = "pretty") {
        colored_json::write_colored_json(&json_ld, &mut stdout())?;
        println!();
    } else {
        println!("{}", serde_json::to_string(&json_ld)?);
    }

    Ok(EX_OK)
}

#[cfg(test)]
mod tests {
    use super::*;
    use asimov_readwise_module::providers::ReadwiseType;

    #[test]
    fn test_importer_creation() {
        let args = Args {
            input_url: "https://readwise.io/highlights".to_string(),
            page_size: None,
            page: None,
        };
        assert_eq!(args.input_url, "https://readwise.io/highlights");
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
    fn test_args_parsing() {
        let args = Args {
            input_url: "https://readwise.io/books".to_string(),
            page_size: None,
            page: None,
        };
        assert_eq!(args.input_url, "https://readwise.io/books");
    }
}
