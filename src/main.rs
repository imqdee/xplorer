mod client;
mod commands;
mod config;
mod error;
mod handlers;

use clap::{Parser, Subcommand};
use commands::account::PaginationParams;
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
    /// Query account data from Etherscan
    Account {
        #[command(subcommand)]
        action: Box<AccountAction>,
    },
    /// Query contract data from Etherscan
    Contract {
        #[command(subcommand)]
        action: ContractAction,
    },
    /// Query block data from Etherscan
    Block {
        #[command(subcommand)]
        action: BlockAction,
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
        action: Box<LogsAction>,
    },
    /// Query token data from Etherscan
    Token {
        #[command(subcommand)]
        action: TokenAction,
    },
    /// Query transaction status from Etherscan
    Transaction {
        #[command(subcommand)]
        action: TransactionAction,
    },
    /// Check API credit usage and limits
    Apilimit {
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// List all chains supported by Etherscan
    Chainlist {
        /// Output raw JSON response
        #[arg(long)]
        raw: bool,
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
enum AccountAction {
    /// Get ETH balance for an address
    Balance {
        /// Account address
        address: String,
        /// Block tag (default: latest)
        #[arg(long, default_value = "latest")]
        tag: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ETH balance at a historical block [Pro]
    Balancehistory {
        /// Account address
        address: String,
        /// Block number to check balance at
        #[arg(long)]
        blockno: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get normal transactions for an address
    Txlist {
        /// Account address
        address: String,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get internal transactions for an address
    Txlistinternal {
        /// Account address
        address: String,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ERC-20 token transfer events for an address
    Tokentx {
        /// Account address
        address: String,
        /// Filter by token contract address
        #[arg(long)]
        contractaddress: Option<String>,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ERC-721 (NFT) token transfer events for an address
    Tokennfttx {
        /// Account address
        address: String,
        /// Filter by token contract address
        #[arg(long)]
        contractaddress: Option<String>,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ERC-1155 token transfer events for an address
    Token1155tx {
        /// Account address
        address: String,
        /// Filter by token contract address
        #[arg(long)]
        contractaddress: Option<String>,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ERC-20 token balance for an address and contract
    Tokenbalance {
        /// Account address
        address: String,
        /// Token contract address
        #[arg(long)]
        contractaddress: String,
        /// Block tag (default: latest)
        #[arg(long, default_value = "latest")]
        tag: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ERC-20 token balance at a historical block [Pro]
    Tokenbalancehistory {
        /// Account address
        address: String,
        /// Token contract address
        #[arg(long)]
        contractaddress: String,
        /// Block number to check balance at
        #[arg(long)]
        blockno: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ERC-20 token holdings for an address [Pro]
    Addresstokenbalance {
        /// Account address
        address: String,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ERC-721 (NFT) token holdings for an address [Pro]
    Addresstokennftbalance {
        /// Account address
        address: String,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get ERC-721 inventory (token IDs) for an address and contract [Pro]
    Addresstokennftinventory {
        /// Account address
        address: String,
        /// NFT contract address
        #[arg(long)]
        contractaddress: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get blocks mined by an address
    Getminedblocks {
        /// Miner address
        address: String,
        /// Block type: blocks or uncles (default: blocks)
        #[arg(long, default_value = "blocks")]
        blocktype: String,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get L2 deposit transactions for an address
    Getdeposittxs {
        /// Account address
        address: String,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get L2 withdrawal transactions for an address
    Getwithdrawaltxs {
        /// Account address
        address: String,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get beacon chain withdrawal transactions for an address
    #[command(name = "txsbeaconwithdrawal")]
    TxsBeaconWithdrawal {
        /// Account address
        address: String,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get bridge transactions for an address
    Txnbridge {
        /// Account address
        address: String,
        /// Starting block number
        #[arg(long)]
        startblock: Option<String>,
        /// Ending block number
        #[arg(long)]
        endblock: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Sort order (asc/desc)
        #[arg(long)]
        sort: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get the address that funded an account
    Fundedby {
        /// Account address
        address: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
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
enum BlockAction {
    /// Get block and uncle reward by block number
    Getblockreward {
        /// Block number
        blockno: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get estimated block countdown time by block number
    Getblockcountdown {
        /// Target block number
        blockno: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get block number by timestamp
    Getblocknobytime {
        /// Unix timestamp
        timestamp: String,
        /// Closest block: before or after (default: before)
        #[arg(long, default_value = "before")]
        closest: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
}

#[derive(Subcommand)]
enum TokenAction {
    /// Get token info by contract address
    Tokeninfo {
        /// Token contract address
        contractaddress: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get total number of token holders [Pro]
    Tokenholdercount {
        /// Token contract address
        contractaddress: String,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get list of token holders [Pro]
    Tokenholderlist {
        /// Token contract address
        contractaddress: String,
        /// Page number
        #[arg(long)]
        page: Option<String>,
        /// Number of results per page
        #[arg(long)]
        offset: Option<String>,
        /// Output raw JSON result field
        #[arg(long)]
        raw: bool,
    },
    /// Get top token holders [Pro]
    Topholders {
        /// Token contract address
        contractaddress: String,
        /// Page number
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
        Commands::Account { action } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let chain_id = resolve_chain_id(cli.chain_id)?;
            let client = client::EtherscanClient::new(api_key, Some(chain_id));

            match *action {
                AccountAction::Balance { address, tag, raw } => {
                    commands::account::balance(&client, &address, &tag, raw).await
                }
                AccountAction::Balancehistory {
                    address,
                    blockno,
                    raw,
                } => commands::account::balancehistory(&client, &address, &blockno, raw).await,
                AccountAction::Txlist {
                    address,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::txlist(&client, &address, &pagination, raw).await
                }
                AccountAction::Txlistinternal {
                    address,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::txlistinternal(&client, &address, &pagination, raw).await
                }
                AccountAction::Tokentx {
                    address,
                    contractaddress,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::tokentx(
                        &client,
                        &address,
                        contractaddress.as_deref(),
                        &pagination,
                        raw,
                    )
                    .await
                }
                AccountAction::Tokennfttx {
                    address,
                    contractaddress,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::tokennfttx(
                        &client,
                        &address,
                        contractaddress.as_deref(),
                        &pagination,
                        raw,
                    )
                    .await
                }
                AccountAction::Token1155tx {
                    address,
                    contractaddress,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::token1155tx(
                        &client,
                        &address,
                        contractaddress.as_deref(),
                        &pagination,
                        raw,
                    )
                    .await
                }
                AccountAction::Tokenbalance {
                    address,
                    contractaddress,
                    tag,
                    raw,
                } => {
                    commands::account::tokenbalance(&client, &address, &contractaddress, &tag, raw)
                        .await
                }
                AccountAction::Tokenbalancehistory {
                    address,
                    contractaddress,
                    blockno,
                    raw,
                } => {
                    commands::account::tokenbalancehistory(
                        &client,
                        &address,
                        &contractaddress,
                        &blockno,
                        raw,
                    )
                    .await
                }
                AccountAction::Addresstokenbalance {
                    address,
                    page,
                    offset,
                    raw,
                } => {
                    commands::account::addresstokenbalance(
                        &client,
                        &address,
                        page.as_deref(),
                        offset.as_deref(),
                        raw,
                    )
                    .await
                }
                AccountAction::Addresstokennftbalance {
                    address,
                    page,
                    offset,
                    raw,
                } => {
                    commands::account::addresstokennftbalance(
                        &client,
                        &address,
                        page.as_deref(),
                        offset.as_deref(),
                        raw,
                    )
                    .await
                }
                AccountAction::Addresstokennftinventory {
                    address,
                    contractaddress,
                    page,
                    offset,
                    raw,
                } => {
                    commands::account::addresstokennftinventory(
                        &client,
                        &address,
                        contractaddress.as_deref(),
                        page.as_deref(),
                        offset.as_deref(),
                        raw,
                    )
                    .await
                }
                AccountAction::Getminedblocks {
                    address,
                    blocktype,
                    page,
                    offset,
                    raw,
                } => {
                    commands::account::getminedblocks(
                        &client,
                        &address,
                        &blocktype,
                        page.as_deref(),
                        offset.as_deref(),
                        raw,
                    )
                    .await
                }
                AccountAction::Getdeposittxs {
                    address,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::getdeposittxs(&client, &address, &pagination, raw).await
                }
                AccountAction::Getwithdrawaltxs {
                    address,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::getwithdrawaltxs(&client, &address, &pagination, raw).await
                }
                AccountAction::TxsBeaconWithdrawal {
                    address,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::txsbeaconwithdrawal(&client, &address, &pagination, raw)
                        .await
                }
                AccountAction::Txnbridge {
                    address,
                    startblock,
                    endblock,
                    page,
                    offset,
                    sort,
                    raw,
                } => {
                    let pagination = PaginationParams {
                        startblock: startblock.as_deref(),
                        endblock: endblock.as_deref(),
                        page: page.as_deref(),
                        offset: offset.as_deref(),
                        sort: sort.as_deref(),
                    };
                    commands::account::txnbridge(&client, &address, &pagination, raw).await
                }
                AccountAction::Fundedby { address, raw } => {
                    commands::account::fundedby(&client, &address, raw).await
                }
            }
        }
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

            match *action {
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
        Commands::Block { action } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let chain_id = resolve_chain_id(cli.chain_id)?;
            let client = client::EtherscanClient::new(api_key, Some(chain_id));

            match action {
                BlockAction::Getblockreward { blockno, raw } => {
                    commands::block::getblockreward(&client, &blockno, raw).await
                }
                BlockAction::Getblockcountdown { blockno, raw } => {
                    commands::block::getblockcountdown(&client, &blockno, raw).await
                }
                BlockAction::Getblocknobytime {
                    timestamp,
                    closest,
                    raw,
                } => commands::block::getblocknobytime(&client, &timestamp, &closest, raw).await,
            }
        }
        Commands::Token { action } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let chain_id = resolve_chain_id(cli.chain_id)?;
            let client = client::EtherscanClient::new(api_key, Some(chain_id));

            match action {
                TokenAction::Tokeninfo {
                    contractaddress,
                    raw,
                } => commands::token::tokeninfo(&client, &contractaddress, raw).await,
                TokenAction::Tokenholdercount {
                    contractaddress,
                    raw,
                } => commands::token::tokenholdercount(&client, &contractaddress, raw).await,
                TokenAction::Tokenholderlist {
                    contractaddress,
                    page,
                    offset,
                    raw,
                } => {
                    commands::token::tokenholderlist(
                        &client,
                        &contractaddress,
                        page.as_deref(),
                        offset.as_deref(),
                        raw,
                    )
                    .await
                }
                TokenAction::Topholders {
                    contractaddress,
                    page,
                    offset,
                    raw,
                } => {
                    commands::token::topholders(
                        &client,
                        &contractaddress,
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
        Commands::Apilimit { raw } => {
            let cfg = config::Config::load();
            let api_key = cfg.require_api_key()?.to_string();
            let client = client::EtherscanClient::new(api_key, None);

            commands::apilimit::apilimit(&client, raw).await
        }
        Commands::Chainlist { raw } => {
            let client = client::EtherscanClient::new_minimal();
            commands::chainlist::chainlist(&client, raw).await
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
