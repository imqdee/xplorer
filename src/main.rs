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
    /// Query gas tracker data from Etherscan
    Gas {
        #[command(subcommand)]
        action: GasAction,
    },
    /// Query event logs from Etherscan
    Logs {
        #[command(subcommand)]
        action: LogsAction,
    },
    /// Query transaction status from Etherscan
    Transaction {
        #[command(subcommand)]
        action: TransactionAction,
    },
    /// Send a raw request to any Etherscan API endpoint
    Raw {
        /// Etherscan API module (e.g. account, transaction, block, stats)
        module: String,
        /// Etherscan API action (e.g. balance, txlist, tokensupply)
        action: String,
        /// Additional query parameters in key=value format
        #[arg(long = "param")]
        params: Vec<String>,
        /// Output single-line JSON instead of pretty-printed
        #[arg(long)]
        compact: bool,
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
    /// Check source code verification submission status
    Checkverifystatus {
        /// GUID from verification submission
        guid: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Check proxy contract verification submission status
    Checkproxyverification {
        /// GUID from proxy verification submission
        guid: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
}

#[derive(Subcommand)]
enum GasAction {
    /// Get current safe, standard, and fast gas prices
    Gasoracle {
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get estimated confirmation time for a given gas price
    Gasestimate {
        /// Gas price in wei
        gasprice: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
}

#[derive(Subcommand)]
enum LogsAction {
    /// Get event logs by address and/or topics within a block range
    Getlogs {
        /// Starting block number
        #[arg(long)]
        from_block: String,
        /// Ending block number
        #[arg(long)]
        to_block: String,
        /// Contract address to filter logs
        #[arg(long)]
        address: Option<String>,
        /// Topic 0 (event signature hash)
        #[arg(long)]
        topic0: Option<String>,
        /// Topic 1
        #[arg(long)]
        topic1: Option<String>,
        /// Topic 2
        #[arg(long)]
        topic2: Option<String>,
        /// Topic 3
        #[arg(long)]
        topic3: Option<String>,
        /// Operator between topic0 and topic1 (and/or)
        #[arg(long)]
        topic0_1_opr: Option<String>,
        /// Operator between topic0 and topic2 (and/or)
        #[arg(long)]
        topic0_2_opr: Option<String>,
        /// Operator between topic0 and topic3 (and/or)
        #[arg(long)]
        topic0_3_opr: Option<String>,
        /// Operator between topic1 and topic2 (and/or)
        #[arg(long)]
        topic1_2_opr: Option<String>,
        /// Operator between topic1 and topic3 (and/or)
        #[arg(long)]
        topic1_3_opr: Option<String>,
        /// Operator between topic2 and topic3 (and/or)
        #[arg(long)]
        topic2_3_opr: Option<String>,
        /// Page number for pagination
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
}

#[derive(Subcommand)]
enum TransactionAction {
    /// Check execution status of a transaction (post-Byzantium: isError field)
    Getstatus {
        /// Transaction hash
        txhash: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Check transaction receipt status (1 = success, 0 = failed)
    Gettxreceiptstatus {
        /// Transaction hash
        txhash: String,
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
            let client = client::EtherscanClient::new(api_key, Some(chain_id));

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
                ContractAction::Checkverifystatus { guid, raw } => {
                    commands::contract::check_verify_status(&client, &guid, raw).await
                }
                ContractAction::Checkproxyverification { guid, raw } => {
                    commands::contract::check_proxy_verification(&client, &guid, raw).await
                }
            }
        }
        Commands::Gas { action } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let chain_id = resolve_chain_id(cli.chain_id)?;
            let client = client::EtherscanClient::new(api_key, Some(chain_id));

            match action {
                GasAction::Gasoracle { raw } => commands::gas::gas_oracle(&client, raw).await,
                GasAction::Gasestimate { gasprice, raw } => {
                    commands::gas::gas_estimate(&client, &gasprice, raw).await
                }
            }
        }
        Commands::Logs { action } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let chain_id = resolve_chain_id(cli.chain_id)?;
            let client = client::EtherscanClient::new(api_key, Some(chain_id));

            match action {
                LogsAction::Getlogs {
                    from_block,
                    to_block,
                    address,
                    topic0,
                    topic1,
                    topic2,
                    topic3,
                    topic0_1_opr,
                    topic0_2_opr,
                    topic0_3_opr,
                    topic1_2_opr,
                    topic1_3_opr,
                    topic2_3_opr,
                    page,
                    offset,
                    raw,
                } => {
                    commands::logs::get_logs(
                        &client,
                        &from_block,
                        &to_block,
                        address.as_deref(),
                        topic0.as_deref(),
                        topic1.as_deref(),
                        topic2.as_deref(),
                        topic3.as_deref(),
                        topic0_1_opr.as_deref(),
                        topic0_2_opr.as_deref(),
                        topic0_3_opr.as_deref(),
                        topic1_2_opr.as_deref(),
                        topic1_3_opr.as_deref(),
                        topic2_3_opr.as_deref(),
                        page.as_deref(),
                        offset.as_deref(),
                        raw,
                    )
                    .await
                }
            }
        }
        Commands::Transaction { action } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let chain_id = resolve_chain_id(cli.chain_id)?;
            let client = client::EtherscanClient::new(api_key, Some(chain_id));

            match action {
                TransactionAction::Getstatus { txhash, raw } => {
                    commands::transaction::get_status(&client, &txhash, raw).await
                }
                TransactionAction::Gettxreceiptstatus { txhash, raw } => {
                    commands::transaction::get_tx_receipt_status(&client, &txhash, raw).await
                }
            }
        }
        Commands::Raw {
            module,
            action,
            params,
            compact,
        } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let chain_id = resolve_chain_id(cli.chain_id)?;
            let client = client::EtherscanClient::new(api_key, Some(chain_id));

            commands::raw::execute(&client, &module, &action, &params, compact).await
        }
    }
}
