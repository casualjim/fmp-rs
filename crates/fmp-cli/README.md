# fmp-cli

Command-line interface for the [Financial Modeling Prep](https://financialmodelingprep.com/) API.

## Installation

```bash
cargo install fmp-cli
```

Or build from source:

```bash
git clone https://git.wagyu.icu/lbyl/fmp-rs
cd fmp-rs
cargo build --release -p fmp-cli
```

## Configuration

Set your API key via environment variable or pass it directly:

```bash
export FMP_API_KEY=your-api-key-here
# or
fmp --api-key your-api-key-here quotes get AAPL
```

Override the base URL if needed:

```bash
export FMP_BASE_URL=https://financialmodelingprep.com/stable/
```

## Usage

```
fmp [OPTIONS] <COMMAND>

Options:
  --api-key <API_KEY>    FMP API key [env: FMP_API_KEY]
  --base-url <BASE_URL>  API base URL [env: FMP_BASE_URL]
  -h, --help             Print help
  -V, --version          Print version
```

All commands output JSON to stdout, making them composable with tools like `jq`.

## Commands

### Quotes

```bash
# Full quote for one or more symbols
fmp quotes get --symbols AAPL,MSFT,GOOGL

# Lightweight quote
fmp quotes short --symbols AAPL

# After-market quotes
fmp quotes aftermarket-quote --symbol AAPL

# Price change (1D, 5D, 1M, etc.)
fmp quotes price-change --symbol AAPL

# Exchange-filtered quotes
fmp quotes exchange --exchange NASDAQ
```

### Charts

```bash
# Historical price data
fmp chart eod --symbol AAPL --from 2024-01-01 --to 2024-12-31

# Intraday (5-minute bars)
fmp chart intraday-5min --symbol AAPL --from 2024-01-15

# With SMA indicator
fmp chart eod --symbol AAPL --sma 20
```

### Company

```bash
fmp company profile --symbol AAPL
fmp company executive --symbol AAPL
fmp company market-cap --symbol AAPL
fmp company is-the-market-open --exchange NYSE
```

### Financial Statements

```bash
fmp statements income --symbol AAPL --period annual --limit 5
fmp statements balance-sheet --symbol AAPL --period quarterly
fmp statements cash-flow --symbol AAPL
fmp statements financial-growth --symbol AAPL
```

### News

```bash
# Latest general market news
fmp news latest --limit 10

# Stock-specific news
fmp news stock --limit 20

# Search by ticker
fmp news search --symbols AAPL,MSFT --limit 5

# Crypto and forex news
fmp news crypto
fmp news forex
```

### Calendar

```bash
fmp calendar earnings --from 2024-01-01 --to 2024-01-31
fmp calendar ipos --from 2024-01-01
fmp calendar splits
fmp calendar dividends
```

### Economics

```bash
fmp economics treasury-rates --from 2024-01-01
fmp economics federal-fund-rate --from 2020-01-01
fmp economics indicators --name GDP
fmp economics calendar-events --from 2024-01-01 --to 2024-01-31
```

### Analyst

```bash
fmp analyst ratings --symbol AAPL
fmp analyst price-target --symbol AAPL
fmp analyst estimates --symbol AAPL --period annual
fmp analyst grades --symbol AAPL --limit 10
```

### SEC Filings

```bash
fmp filings by-symbol --symbol AAPL --limit 10
fmp filings by-type --form-type 10-K --limit 5
fmp filings by-cik --cik 0000320193
fmp filings latest-8k --limit 20
```

### Earnings Transcripts

```bash
fmp transcript latest --limit 5
fmp transcript get --symbol AAPL --year 2024 --quarter 1
fmp transcript dates --symbol AAPL
```

### Crypto

```bash
fmp crypto quote --symbol BTCUSD
fmp crypto eod-full --symbol BTCUSD --from 2024-01-01
fmp crypto news --limit 10
fmp crypto list
```

### Forex

```bash
fmp forex quote --symbol EURUSD
fmp forex eod-light --symbol EURUSD --from 2024-01-01
fmp forex news
fmp forex list
```

### Market Data

```bash
# Market hours
fmp market-hours exchange --exchange NYSE
fmp market-hours holidays --from 2024-01-01 --to 2024-12-31

# Market performance
fmp market-performance biggest-gainers
fmp market-performance biggest-losers
fmp market-performance sector-performance-snapshot

# Indexes
fmp indexes quote --symbol SPX
fmp indexes constituents --symbol SPX
```

### Search & Directory

```bash
fmp search symbol --query "apple" --limit 10
fmp search screener --market-cap-more-than 1000000000 --limit 20

fmp directory stock-list
fmp directory etf-list
```

## Piping with jq

```bash
# Get closing prices
fmp quotes get --symbols AAPL | jq '.[] | .price'

# Filter earnings by surprise
fmp calendar earnings --from 2024-01-01 | jq '.[] | select(.epsActual > .epsEstimated)'

# Pretty-print with color
fmp company profile --symbol AAPL | jq .
```

## Shell Completions

Generate shell completions:

```bash
# Bash
fmp completions --shell bash > ~/.local/share/bash-completion/completions/fmp

# Zsh
fmp completions --shell zsh > ~/.zsh/completions/_fmp

# Fish
fmp completions --shell fish > ~/.config/fish/completions/fmp.fish
```
