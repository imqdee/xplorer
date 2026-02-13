mod client;
mod commands;
mod config;
mod error;
mod handlers;

use clap::{Parser, Subcommand};
use error::XplorerError;
use std::env;

#[derive(Parser)]
#[command(name = "xplorer")]
#[command(about = "Etherscan API CLI wrapper", long_about = None)]
struct Cli {
    /// Chain ID for the target network
    #[arg(long, global = true)]
    chain_id: Option<u64>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query contract data from Etherscan
    Contract {
        #[command(subcommand)]
        action: ContractAction,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ContractAction {
    /// Get the ABI of a verified contract
    Getabi {
        /// Contract address
        address: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get the source code of a verified contract
    Getsourcecode {
        /// Contract address
        address: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get contract creation info (creator address and tx hash)
    Getcontractcreation {
        /// Contract addresses (1 to 5)
        #[arg(num_args = 1..=5)]
        addresses: Vec<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set a configuration value
    Set {
        #[command(subcommand)]
        setting: ConfigSetting,
    },
}

#[derive(Subcommand)]
enum ConfigSetting {
    /// Set the Etherscan API key (prompts securely if not provided)
    ApiKey { key: Option<String> },
}

fn resolve_chain_id(flag: Option<u64>) -> Result<u64, XplorerError> {
    if let Some(id) = flag {
        return Ok(id);
    }

    env::var("STARGATE_CHAIN_ID")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .ok_or(XplorerError::MissingChainId)
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), XplorerError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config { action } => match action {
            ConfigAction::Set { setting } => match setting {
                ConfigSetting::ApiKey { key } => commands::config::set_api_key(key),
            },
        },
        Commands::Contract { action } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let chain_id = resolve_chain_id(cli.chain_id)?;
            let client = client::EtherscanClient::new(api_key, chain_id);

            match action {
                ContractAction::Getabi { address, raw } => {
                    commands::contract::get_abi(&client, &address, raw).await
                }
                ContractAction::Getsourcecode { address, raw } => {
                    commands::contract::get_source_code(&client, &address, raw).await
                }
                ContractAction::Getcontractcreation { addresses, raw } => {
                    commands::contract::get_contract_creation(&client, &addresses, raw).await
                }
            }
        }
    }
}
