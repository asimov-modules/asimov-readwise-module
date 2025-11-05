// This is free and unencumbered software released into the public domain.
use asimov_readwise_module::api::types::ReadwiseType;
use asimov_readwise_module::jq;
use clap::{Parser, ValueEnum};
use clientele::StandardOptions;

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Json,
    Jsonl,
}

#[derive(Parser)]
#[command(name = "asimov-readwise-importer")]
#[command(about = "URL protocol importer. Consumes a URL input, produces JSON-LD output.")]
struct Options {
    #[arg(value_name = "INPUT-URL")]
    input_url: Option<String>,

    #[arg(long, value_name = "SIZE")]
    page_size: Option<usize>,

    #[arg(long, value_name = "NUM")]
    page: Option<usize>,

    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<OutputFormat>,

    #[clap(flatten)]
    flags: StandardOptions,
}

fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_module::getenv;
    use asimov_module::secrecy::ExposeSecret;
    use asimov_readwise_module::{api::readwise::ReadwiseClient, find_provider_for};
    use clientele::SysexitsError::*;

    clientele::dotenv().ok();

    let options: Options = Options::parse();

    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    let Some(input_url) = options.input_url else {
        eprintln!("Missing INPUT-URL argument. Use --help for usage info");
        return Ok(EX_USAGE);
    };

    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    let Some(api_key) = getenv::var_secret("READWISE_API_KEY") else {
        eprintln!("Missing READWISE_API_KEY. Run `asimov module config readwise`");
        return Ok(EX_CONFIG);
    };

    let config = asimov_readwise_module::api::readwise::ReadwiseConfig::new(
        api_key.expose_secret().to_string(),
    );

    let mut api = ReadwiseClient::new(config)?;

    let Some(provider) = find_provider_for(&input_url) else {
        eprintln!(
            "Unsupported URL: {}. Supported: highlights, books, tags",
            input_url
        );
        return Ok(EX_UNAVAILABLE);
    };

    let json_ld = match provider.id {
        ReadwiseType::HIGHLIGHTS_ID => {
            let highlights = api.fetch_highlights(options.page_size, options.page)?;
            let highlights_json = serde_json::to_value(&highlights)?;
            jq::readwise().filter_json(highlights_json)?
        },
        ReadwiseType::BOOKLIST_ID => {
            let booklist = api.fetch_booklist(options.page_size, options.page)?;
            let booklist_json = serde_json::to_value(&booklist)?;
            jq::books().filter_json(booklist_json)?
        },
        ReadwiseType::TAGS_ID => {
            let tags = api.fetch_highlight_tags()?;
            let tags_json = serde_json::to_value(&tags)?;
            jq::tags().filter_json(tags_json)?
        },
        _ => {
            eprintln!("Unsupported provider type: {:?}", provider.id);
            return Ok(EX_UNAVAILABLE);
        },
    };

    let output_format = options.output.unwrap_or(OutputFormat::Json);

    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(&json_ld)?);
        },
        OutputFormat::Jsonl => {
            let items = match provider.id {
                ReadwiseType::HIGHLIGHTS_ID => {
                    json_ld.get("highlights")
                        .and_then(|h| h.get("items"))
                        .and_then(|i| i.as_array())
                },
                ReadwiseType::BOOKLIST_ID => {
                    json_ld.get("books")
                        .and_then(|b| b.get("items"))
                        .and_then(|i| i.as_array())
                },
                ReadwiseType::TAGS_ID => {
                    json_ld.get("tags")
                        .and_then(|t| t.get("items"))
                        .and_then(|i| i.as_array())
                },
                _ => None,
            };

            if let Some(items_array) = items {
                for item in items_array {
                    let line = serde_json::to_string(item)?;
                    println!("{}", line);
                }
            }
        },
    }

    Ok(EX_OK)
}
