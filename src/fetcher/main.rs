// This is free and unencumbered software released into the public domain.
use asimov_readwise_module::providers::ReadwiseType;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "asimov-readwise-fetcher")]
#[command(about = "URL protocol fetcher. Consumes a URL input, produces RDF output.")]
struct Args {
    /// The input URL
    #[arg(value_name = "INPUT-URL")]
    input_url: String,

    /// The output format
    #[arg(short, long, value_name = "FORMAT", default_value = "jsonl")]
    output: OutputFormat,
}

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Jsonl,
    Json,
    Turtle,
    Ntriples,
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

    let config = asimov_readwise_module::api::types::types::ReadwiseConfig::new(
        api_key.expose_secret().to_string(),
    );
    let api = ReadwiseClient::new(config)?;

    let Some(provider) = find_provider_for(&args.input_url) else {
        eprintln!("Unsupported URL: {}", args.input_url);
        eprintln!("Make sure your URL starts with one of these patterns.");
        return Ok(EX_UNAVAILABLE);
    };

    let response = match provider.id {
        ReadwiseType::HIGHLIGHTS_ID => {
            let rt = tokio::runtime::Runtime::new()?;
            let highlights = rt.block_on(api.fetch_highlights_from_url(&args.input_url))?;
            serde_json::to_string(&highlights)?
        },
        ReadwiseType::BOOKLIST_ID => {
            let rt = tokio::runtime::Runtime::new()?;
            let booklist = rt.block_on(api.fetch_booklist_from_url(&args.input_url))?;
            serde_json::to_string(&booklist)?
        },
        _ => {
            eprintln!("Unsupported provider type: {:?}", provider.id);
            return Ok(EX_UNAVAILABLE);
        },
    };

    match args.output {
        OutputFormat::Jsonl => {
            let response_json: serde_json::Value = serde_json::from_str(&response)?;
            println!("{}", serde_json::to_string(&response_json)?);
        },
        OutputFormat::Json => {
            if cfg!(feature = "pretty") {
                let response_json: serde_json::Value = serde_json::from_str(&response)?;
                colored_json::write_colored_json(&response_json, &mut stdout())?;
                println!();
            } else {
                println!("{}", response);
            }
        },
        OutputFormat::Turtle | OutputFormat::Ntriples => {
            let response_json: serde_json::Value = serde_json::from_str(&response)?;
            println!("{}", serde_json::to_string(&response_json)?);
        },
    }

    Ok(EX_OK)
}

#[cfg(test)]
mod tests {
    use asimov_readwise_module::find_provider_for;
    use asimov_readwise_module::providers::ReadwiseType;

    #[test]
    fn test_find_provider_for_highlights() {
        let url = "https://readwise.io/api/v2/highlights/?page_size=5";
        let provider = find_provider_for(url);
        assert!(provider.is_some());
        assert_eq!(provider.unwrap().id, ReadwiseType::HIGHLIGHTS_ID);
    }

    #[test]
    fn test_find_provider_for_books() {
        let url = "https://readwise.io/api/v2/books/?page_size=5";
        let provider = find_provider_for(url);
        assert!(provider.is_some());
        assert_eq!(provider.unwrap().id, ReadwiseType::BOOKLIST_ID);
    }

    #[test]
    fn test_find_provider_for_unsupported_url() {
        let url = "https://example.com/api/books";
        let provider = find_provider_for(url);
        assert!(provider.is_none());
    }

    #[test]
    fn test_provider_brand() {
        let url = "https://readwise.io/api/v2/highlights/";
        let provider = find_provider_for(url).unwrap();
        assert_eq!(provider.brand, "Readwise");
    }
}
