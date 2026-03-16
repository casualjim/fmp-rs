---
name: fmp
description: >
  Use the fmp CLI tool to fetch financial data. Trigger when the user asks for stock
  prices, quotes, historical price charts, financial statements (income/balance sheet/
  cash flow), analyst ratings, earnings transcripts, SEC filings, crypto prices,
  forex rates, commodity data, market indexes, economic indicators, treasury rates,
  ESG scores, insider trades, COT reports, IPO calendars, dividend calendars, or
  any other financial market data. Also trigger when the user wants to screen stocks,
  search for companies by name or CIK, look up market hours, or get sector/industry
  performance. The fmp binary reads FMP_API_KEY from the environment.
metadata:
  version: "1.0.0"
  binary: fmp
  source: code.lotsa.ai/lbyl/fmp-rs
---

# fmp CLI

Command-line interface for the [Financial Modeling Prep API](https://financialmodelingprep.com/). Covers real-time quotes, historical OHLCV, financial statements, analyst data, SEC filings, crypto, forex, economics, and more.

## Setup

```bash
export FMP_API_KEY=your_key_here   # or pass --api-key KEY to every command
fmp --help                          # list all namespaces
fmp <namespace> --help              # list subcommands
fmp <namespace> <subcommand> --help # show args for a specific command
```

## When to Apply

Use `fmp` when the user wants:
- **Prices / quotes**: current or historical stock, crypto, forex, commodity, index prices
- **Financials**: income statements, balance sheets, cash flow, ratios, key metrics
- **Analyst data**: ratings, price targets, EPS estimates, grade changes
- **News**: market news, stock/crypto/forex news, press releases
- **Calendars**: earnings dates, IPOs, stock splits, dividends
- **SEC filings**: 10-K, 10-Q, 8-K by symbol, CIK, or form type
- **Transcripts**: earnings call transcripts
- **Macro**: economic indicators, treasury rates, Fed funds rate
- **Screening**: filter stocks by market cap, sector, price, volume, etc.
- **Market structure**: sector/industry performance, movers, market hours

## Command Map

| Namespace | What it does | Key subcommands |
|-----------|-------------|-----------------|
| `quotes` | Real-time quotes | `get AAPL MSFT`, `short`, `batch`, `exchange NYSE`, `price-change` |
| `chart` | Historical OHLCV + indicators | `eod-full`, `eod-light`, `intraday`, `sma`, `ema`, `rsi`, `adx` |
| `company` | Profiles & corporate data | `profile`, `executives`, `market-cap`, `employees`, `peers`, `float` |
| `statements` | Financial statements | `income`, `balance-sheet`, `cash-flow`, `key-metrics`, `ratios`, `scores` |
| `analyst` | Analyst coverage | `ratings`, `price-target`, `estimates`, `grades` |
| `news` | Financial news | `latest`, `stock`, `crypto`, `forex`, `search --symbols` |
| `calendar` | Event calendars | `earnings`, `ipos`, `splits`, `dividends` |
| `filings` | SEC filings | `by-symbol`, `by-type`, `latest-8k`, `by-cik` |
| `transcript` | Earnings call transcripts | `get --symbol --year --quarter`, `dates`, `latest` |
| `search` | Symbol/company lookup | `symbol --query`, `name --query`, `cik`, `cusip`, `isin`, `screener` |
| `economics` | Macro indicators | `indicators --name GDP`, `treasury-rates`, `federal-fund-rate`, `calendar-events` |
| `crypto` | Crypto prices | `quote --symbol BTCUSD`, `eod-full`, `intraday-5min`, `news` |
| `forex` | Currency rates | `quote --symbol EURUSD`, `eod-full`, `intraday-1hour` |
| `commodities` | Commodity prices | `list` |
| `indexes` | Market indexes | `quote --symbol SPX`, `eod-full`, `list` |
| `market-performance` | Sector/industry perf | `biggest-gainers`, `biggest-losers`, `most-actives`, `sector-performance-snapshot --date` |
| `market-hours` | Exchange hours | `exchange --exchange NYSE`, `holidays`, `all` |
| `directory` | Symbol directories | `stock-list`, `available-sectors`, `available-exchanges`, `etf-list` |
| `dcf` | Valuation models | `valuation --symbol`, `levered-valuation`, `custom` |
| `esg` | ESG scores | `disclosure --symbol`, `ratings --symbol`, `benchmark` |
| `cot` | COT reports | `report`, `analysis` |

## Common Examples

```bash
# Current stock price
fmp quotes get AAPL

# Multiple symbols at once
fmp quotes get AAPL MSFT GOOGL META NVDA

# Full year of daily price history
fmp chart eod-full --symbol AAPL --from 2024-01-01 --to 2024-12-31

# 5-minute intraday bars (today)
fmp chart intraday --symbol AAPL --interval 5min

# RSI (14-period)
fmp chart rsi --symbol AAPL --period 14

# Company profile
fmp company profile --symbol AAPL

# Last 5 annual income statements
fmp statements income --symbol AAPL --period annual --limit 5

# Quarterly balance sheet
fmp statements balance-sheet --symbol AAPL --period quarterly --limit 8

# Key financial metrics (TTM)
fmp statements key-metrics-ttm --symbol AAPL

# Analyst price targets
fmp analyst price-target --symbol AAPL

# Annual EPS estimates
fmp analyst estimates --symbol AAPL --period annual

# Earnings calendar (next quarter)
fmp calendar earnings --from 2026-01-01 --to 2026-03-31

# Stock-specific news (last 20)
fmp news search --symbols AAPL --limit 20

# Latest 10-K filings
fmp filings by-type --form-type 10-K --limit 10

# All 10-K filings for a company
fmp filings by-symbol --symbol AAPL

# Earnings call transcript
fmp transcript get --symbol AAPL --year 2024 --quarter 4

# Bitcoin daily prices
fmp crypto eod-full --symbol BTCUSD --from 2024-01-01

# EUR/USD rate history
fmp forex eod-full --symbol EURUSD --from 2024-01-01

# S&P 500 price history
fmp indexes eod-full --symbol SPX --from 2024-01-01

# US Treasury yield curve
fmp economics treasury-rates --from 2024-01-01

# GDP time series
fmp economics indicators --name GDP

# Stock screener: large-cap tech stocks
fmp search screener --market-cap-more-than 10000000000 --sector Technology --exchange NASDAQ --limit 50

# Today's biggest gainers
fmp market-performance biggest-gainers

# Sector performance on a date
fmp market-performance sector-performance-snapshot --date 2024-03-15

# DCF valuation
fmp dcf valuation --symbol AAPL
```

## Argument Conventions

- **Date strings**: `YYYY-MM-DD` (e.g., `2024-01-01`)
- **Period**: `annual` or `quarterly`
- **Intervals**: `1min`, `5min`, `15min`, `30min`, `1hour`, `4hour`
- **Timeframe** (indicators): `daily`, `weekly`, `monthly`
- **Multiple symbols** (positional): space-separated — `fmp quotes get AAPL MSFT`
- **Multiple symbols** (--flag): comma-separated — `fmp news search --symbols "AAPL,MSFT"`
- **Limit**: number of records to return (plan-dependent maximum)
- **Page**: 0-indexed pagination

## Full Command Reference

See [references/commands.md](references/commands.md) for the complete listing of every subcommand and all its arguments.
