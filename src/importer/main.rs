// This is free and unencumbered software released into the public domain.
use asimov_readwise_module::api::types::ReadwiseType;
use asimov_readwise_module::jq;
use clap::Parser;
use clientele::StandardOptions;

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

    #[clap(flatten)]
    flags: StandardOptions,
}

fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_module::getenv;
    use asimov_module::secrecy::ExposeSecret;
    use asimov_readwise_module::{api::readwise::ReadwiseClient, find_provider_for};
    use clientele::SysexitsError::*;
    use std::io::stdout;

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

    if cfg!(feature = "pretty") {
        colored_json::write_colored_json(&json_ld, &mut stdout())?;
        println!();
    } else {
        println!("{}", serde_json::to_string(&json_ld)?);
    }

    Ok(EX_OK)
}
