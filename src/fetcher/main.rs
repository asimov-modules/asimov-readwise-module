// This is free and unencumbered software released into the public domain.
use asimov_readwise_module::providers::ReadwiseType;

fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_module::getenv;
    use asimov_module::secrecy::ExposeSecret;
    use asimov_readwise_module::{api::readwise::ReadwiseClient, find_engine_for};
    use clientele::SysexitsError::*;
    use std::io::stdout;

    clientele::dotenv().ok();

    let args = clientele::args_os()?;

    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    let urls: Vec<String> = args
        .iter()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into())
        .collect();

    if urls.is_empty() {
        return Ok(EX_OK);
    }

    let Some(api_key) = getenv::var_secret("READWISE_API_KEY") else {
        return Ok(EX_CONFIG);
    };

    let config = asimov_readwise_module::api::readwise::ReadwiseConfig::new(
        api_key.expose_secret().to_string(),
    );
    let api = ReadwiseClient::new(config)?;

    for url in urls {
        let Some(engine) = find_engine_for(&url) else {
            return Ok(EX_UNAVAILABLE);
        };

        let response = match engine.id {
            ReadwiseType::HIGHLIGHTS_ID => {
                let rt = tokio::runtime::Runtime::new()?;
                let highlights = rt.block_on(api.fetch_highlights_from_url(&url))?;
                serde_json::to_string(&highlights)?
            },
            _ => {
                return Ok(EX_UNAVAILABLE);
            },
        };

        if cfg!(feature = "pretty") {
            let response_json: serde_json::Value = serde_json::from_str(&response)?;
            colored_json::write_colored_json(&response_json, &mut stdout())?;
            println!();
        } else {
            println!("{}", response);
        }
    }

    Ok(EX_OK)
}
