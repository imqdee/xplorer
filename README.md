# xplorer

[Etherscan](https://etherscan.io/) API CLI wrapper. Query blockchain data from the terminal using the [Etherscan V2 API](https://docs.etherscan.io/etherscan-v2), with multichain support via a single API key.

The goal is to cover all modules and routes exposed by the Etherscan API. This is a work in progress. Currently supported:

- **contract** module
  - `getabi` - Get the ABI of a verified contract
  - `getsourcecode` - Get the source code of a verified contract
  - `getcontractcreation` - Get contract creation info (creator address + deployment tx hash)
- **raw** - Direct access to any Etherscan API endpoint (module + action + arbitrary params)

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
xplorer --chain-id 1 contract getabi 0x...
```

xplorer is also compatible with [Stargate](https://github.com/imqdee/stargate), a blockchain network switcher CLI for Foundry. When you switch networks with Stargate, it exports a `STARGATE_CHAIN_ID` environment variable that xplorer picks up automatically, so you don't need to pass `--chain-id` on every call:

```bash
sg switch mainnet
xplorer contract getabi 0x...   # uses chain ID 1 from Stargate
```

If both are provided, the `--chain-id` flag takes precedence.

### Contract Module

```bash
# Get the ABI of a verified contract (pretty-printed JSON)
xplorer --chain-id 1 contract getabi 0xdAC17F958D2ee523a2206206994597C13D831ec7

# Get full source code with metadata (compiler, optimization, license, proxy info)
xplorer --chain-id 1 contract getsourcecode 0xdAC17F958D2ee523a2206206994597C13D831ec7

# Get contract creation info (creator address + deployment tx hash)
xplorer --chain-id 1 contract getcontractcreation 0xdAC17F958D2ee523a2206206994597C13D831ec7

# Query multiple contracts at once (up to 5)
xplorer --chain-id 1 contract getcontractcreation 0xdAC1...1ec7 0xA0b8...eB48
```

### Raw API Access

The `raw` command gives you direct access to the entire Etherscan API surface. Pass any module, action, and key=value parameters:

```bash
xplorer raw <module> <action> [--param key=value]... [--compact]
```

Output defaults to pretty-printed JSON (the `result` field from the Etherscan response). Use `--compact` for single-line JSON, useful for piping.

```bash
# Get account balance
xplorer --chain-id 1 raw account balance --param address=0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe --param tag=latest

# Get token supply (compact output for piping)
xplorer --chain-id 1 raw stats tokensupply --param contractaddress=0xdAC17F958D2ee523a2206206994597C13D831ec7 --compact

# Get transaction list and pipe to jq
xplorer --chain-id 1 raw account txlist --param address=0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe --param startblock=0 --param endblock=99999999 --compact | jq '.[0]'

# Get contract ABI (equivalent to `contract getabi --raw`)
xplorer --chain-id 1 raw contract getabi --param address=0xdAC17F958D2ee523a2206206994597C13D831ec7
```

See all available modules and actions at [docs.etherscan.io](https://docs.etherscan.io/etherscan-v2).

### Supported Chains

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

See the full list at [docs.etherscan.io](https://docs.etherscan.io/etherscan-v2).

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
./target/release/xplorer --chain-id 1 contract getabi 0xdAC17F958D2ee523a2206206994597C13D831ec7
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
