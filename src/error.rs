#[derive(Debug, thiserror::Error)]
pub enum XplorerError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("No API key configured. Run `xplorer config set api-key` first.")]
    MissingApiKey,

    #[error("No chain ID provided. Use --chain-id flag or set STARGATE_CHAIN_ID env var.")]
    MissingChainId,

    #[error("{0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("Validation error: {0}")]
    Validation(String),
}
