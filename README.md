# xplorer

[Etherscan](https://etherscan.io/) API CLI wrapper. Query blockchain data from the terminal using the [Etherscan V2 API](https://docs.etherscan.io/etherscan-v2), with multichain support via a single API key.

See the official [Etherscan API documentation](https://docs.etherscan.io/introduction) for the full API reference.

## Installation

```bash
cargo install xplorer
```

## Configuration

Set your [Etherscan](https://etherscan.io/apis) API key:

```bash
# Interactive mode (recommended - input is hidden)
xplorer config set api-key

# Or pass directly (visible in shell history)
xplorer config set api-key YOUR_ETHERSCAN_KEY
```

Config is stored at `~/.xplorer/config.toml`.

## Usage

### Chain ID

Every query requires a chain ID, provided via the `--chain-id` flag:

```bash
xplorer --chain-id 1 account balance 0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe
```

xplorer is also compatible with [Stargate](https://github.com/imqdee/stargate), a blockchain network switcher CLI for Foundry. When you switch networks with Stargate, it exports a `STARGATE_CHAIN_ID` environment variable that xplorer picks up automatically, so you don't need to pass `--chain-id` on every call:

```bash
sg switch mainnet
xplorer account balance 0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe
```

If both are provided, the `--chain-id` flag takes precedence.

### Raw output

Every formatted command supports a `--raw` flag that outputs the JSON `result` field directly, useful for scripting and piping:

```bash
xplorer --chain-id 1 account balance 0xde0B2...9BAe --raw | jq .
```

## Supported Endpoints

xplorer covers 45 read-only endpoints across 9 modules. Endpoints marked **[Pro]** require an [Etherscan Pro](https://docs.etherscan.io/getting-started/endpoint-urls#pro-api) API key.

### Account

| Command | Description |
|---------|-------------|
| `account balance <address>` | Get native balance for an address |
| `account balancehistory <address> --blockno <n>` | Get native balance at a historical block **[Pro]** |
| `account txlist <address>` | Get normal transactions |
| `account txlistinternal <address>` | Get internal transactions |
| `account tokentx <address>` | Get ERC-20 token transfer events |
| `account tokennfttx <address>` | Get ERC-721 (NFT) transfer events |
| `account token1155tx <address>` | Get ERC-1155 transfer events |
| `account tokenbalance <address> --contractaddress <ca>` | Get ERC-20 token balance for a specific contract |
| `account tokenbalancehistory <address> --contractaddress <ca> --blockno <n>` | Get ERC-20 token balance at a historical block **[Pro]** |
| `account addresstokenbalance <address>` | Get all ERC-20 token holdings **[Pro]** |
| `account addresstokennftbalance <address>` | Get all ERC-721 (NFT) holdings **[Pro]** |
| `account addresstokennftinventory <address>` | Get ERC-721 token IDs for an address and contract **[Pro]** |
| `account getminedblocks <address>` | Get blocks mined by an address |
| `account getdeposittxs <address>` | Get L2 deposit transactions |
| `account getwithdrawaltxs <address>` | Get L2 withdrawal transactions |
| `account txsbeaconwithdrawal <address>` | Get beacon chain withdrawals |
| `account txnbridge <address>` | Get bridge transactions |
| `account fundedby <address>` | Get the address that funded an account |

Transaction list commands (`txlist`, `txlistinternal`, `tokentx`, `tokennfttx`, `token1155tx`, `getdeposittxs`, `getwithdrawaltxs`, `txsbeaconwithdrawal`, `txnbridge`) support `--startblock`, `--endblock`, `--page`, `--offset`, and `--sort` options.

### Contract

| Command | Description |
|---------|-------------|
| `contract getabi <address>` | Get the ABI of a verified contract |
| `contract getsourcecode <address>` | Get source code, compiler settings, and metadata |
| `contract getcontractcreation <address>...` | Get contract creator address and deployment tx (up to 5) |
| `contract checkverifystatus <guid>` | Check source code verification status |
| `contract checkproxyverification <guid>` | Check proxy contract verification status |

### Token

| Command | Description |
|---------|-------------|
| `token tokeninfo <contractaddress>` | Get token name, symbol, type, supply, and social links |
| `token tokenholdercount <contractaddress>` | Get total number of token holders **[Pro]** |
| `token tokenholderlist <contractaddress>` | Get paginated list of token holders **[Pro]** |
| `token topholders <contractaddress>` | Get top token holders **[Pro]** |

### Block

| Command | Description |
|---------|-------------|
| `block getblockreward <blockno>` | Get block reward and uncle details |
| `block getblockcountdown <blockno>` | Get estimated countdown to a future block |
| `block getblocknobytime <timestamp>` | Find the block closest to a given unix timestamp |

### Stats

All stats endpoints are **[Pro]** and accept `--startdate` and `--enddate` in `yyyy-MM-dd` format, and `--sort` (asc/desc).

| Command | Description |
|---------|-------------|
| `stats dailyavgblocksize --startdate <d> --enddate <d>` | Get daily average block size |
| `stats dailyblkcount --startdate <d> --enddate <d>` | Get daily block count and rewards |
| `stats dailyblockrewards --startdate <d> --enddate <d>` | Get daily block rewards |
| `stats dailyavgblocktime --startdate <d> --enddate <d>` | Get daily average block time |
| `stats dailyuncleblkcount --startdate <d> --enddate <d>` | Get daily uncle block count and rewards |
| `stats dailyavggaslimit --startdate <d> --enddate <d>` | Get daily average gas limit |
| `stats dailyavggasprice --startdate <d> --enddate <d>` | Get daily average gas price |
| `stats dailygasused --startdate <d> --enddate <d>` | Get daily total gas used |

### Gas Tracker

| Command | Description |
|---------|-------------|
| `gas gasoracle` | Get current safe, standard, and fast gas prices |
| `gas gasestimate <gasprice>` | Get estimated confirmation time for a gas price (wei) |

### Logs

| Command | Description |
|---------|-------------|
| `logs getlogs --from-block <n> --to-block <n>` | Get event logs by address and/or topics |

Supports filtering by `--address`, `--topic0` through `--topic3`, and topic operators (`--topic0-1-opr`, etc.).

### Transaction

| Command | Description |
|---------|-------------|
| `transaction getstatus <txhash>` | Check execution status of a transaction |
| `transaction gettxreceiptstatus <txhash>` | Check transaction receipt status (success/fail) |

### Utilities

| Command | Description |
|---------|-------------|
| `apilimit` | Check API credit usage and rate limits |
| `chainlist` | List all 60+ chains supported by the Etherscan V2 API |

### Raw API Access

The `raw` command gives you direct access to the entire Etherscan API surface, including endpoints not yet covered by dedicated commands:

```bash
xplorer raw <module> <action> [--param key=value]... [--compact]
```

Output defaults to pretty-printed JSON. Use `--compact` for single-line output, useful for piping.

```bash
# Get token supply
xplorer --chain-id 1 raw stats tokensupply \
  --param contractaddress=0xdAC17F958D2ee523a2206206994597C13D831ec7 --compact

# Get transaction list and pipe to jq
xplorer --chain-id 1 raw account txlist \
  --param address=0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe \
  --param startblock=0 --compact | jq '.[0]'
```

## Supported Chains

xplorer uses the Etherscan V2 API, which supports 60+ chains through a single endpoint. Pass any valid chain ID:

| Chain    | Chain ID |
| -------- | -------- |
| Ethereum | 1        |
| Arbitrum | 42161    |
| Base     | 8453     |
| Optimism | 10       |
| Polygon  | 137      |
| BSC      | 56       |
| Linea    | 59144    |
| Scroll   | 534352   |
| zkSync   | 324      |

Run `xplorer chainlist` for the full list, or see the [Etherscan V2 docs](https://docs.etherscan.io/etherscan-v2).

## Developers

### Building from Source

```bash
git clone https://github.com/imqdee/xplorer.git
cd xplorer
cargo build --release
```

The binary will be at `target/release/xplorer`.

### Local Installation

**Option 1: Install globally** (replaces any existing installation)

```bash
cargo install --path .
```

**Option 2: Test without installing**

```bash
cargo build --release
./target/release/xplorer --help
./target/release/xplorer --chain-id 1 account balance 0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe
```

### Running Tests

```bash
cargo test
```

### Git Hooks

This project uses [lefthook](https://github.com/evilmartians/lefthook) for git hooks.

```bash
# Install lefthook (macOS)
brew install lefthook

# Install hooks
lefthook install
```

Hooks run automatically on commit (fmt, clippy) and push (test, build).
