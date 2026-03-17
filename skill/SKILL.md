---
name: fmp
description: >
  Use the fmp CLI tool to fetch financial data. Trigger when the user asks for stock
  prices, quotes, historical price charts, financial statements (income/balance sheet/
  cash flow), analyst ratings, earnings transcripts, SEC filings, crypto prices,
  forex rates, commodity data, market indexes, economic indicators, treasury rates,
  ESG scores, insider trades, government/congressional trading disclosures, COT reports,
  IPO calendars, dividend calendars, Form 13F institutional ownership filings, ETF and
  fund data, crowdfunding or equity fundraising filings, technical indicators (SMA, EMA,
  RSI, ADX, etc.), or any other financial market data. Also trigger when the user wants
  to screen stocks, search for companies by name or CIK, look up market hours, or get
  sector/industry performance. The fmp binary reads FMP_API_KEY from the environment.
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
- **Analyst data**: ratings, price targets, EPS estimates, grade changes, analyst news
- **News**: market news, stock/crypto/forex news, press releases (including by-symbol search)
- **Calendars**: earnings dates, IPOs, IPO disclosures/prospectus, stock splits, dividends
- **SEC filings**: 10-K, 10-Q, 8-K by symbol, CIK, or form type; SEC company profile; SIC industry classification
- **Transcripts**: earnings call transcripts
- **Macro**: economic indicators, treasury rates, Fed funds rate
- **Screening**: filter stocks by market cap, sector, price, volume, etc.
- **Market structure**: sector/industry performance, movers, market hours
- **Insider trading**: latest transactions, search by symbol/CIK/type, beneficial ownership (5%+ holders), statistics
- **Government trading**: Senate and House financial disclosure trades by symbol or name
- **Institutional ownership**: Form 13F filings, holdings extraction, holder analytics/performance/industry, symbol positions
- **Funds / ETFs**: ETF holdings, info, sector/country weightings, fund portfolio disclosures
- **Fundraising**: crowdfunding offerings (Reg CF), equity fundraising (Reg D/Reg A) filings
- **Technical indicators**: SMA, EMA, WMA, DEMA, TEMA, RSI, StdDev, Williams %R, ADX — standalone with configurable period/timeframe

## Command Map

| Namespace | What it does | Key subcommands |
|-----------|-------------|-----------------|
| `quotes` | Real-time quotes | `get AAPL MSFT`, `short`, `batch`, `exchange NYSE`, `price-change` |
| `chart` | Historical OHLCV + indicators | `eod-full`, `eod-light`, `intraday`, `sma`, `ema`, `rsi`, `adx` |
| `company` | Profiles & corporate data | `profile`, `executives`, `market-cap`, `employees`, `peers`, `float` |
| `statements` | Financial statements | `income`, `balance-sheet`, `cash-flow`, `key-metrics`, `ratios`, `scores` |
| `analyst` | Analyst coverage | `ratings`, `price-target`, `estimates`, `grades`, `price-target-news`, `grades-news` |
| `news` | Financial news | `latest`, `stock`, `crypto`, `forex`, `search --symbols`, `search-press-releases`, `search-crypto`, `search-forex` |
| `calendar` | Event calendars | `earnings`, `ipos`, `ipos-disclosure`, `ipos-prospectus`, `splits`, `dividends` |
| `filings` | SEC filings | `by-symbol`, `by-type`, `latest-8k`, `by-cik`, `sec-profile`, `industry-classification-list` |
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
| `insider-trades` | Insider trading | `latest`, `search`, `by-name`, `transaction-types`, `statistics`, `beneficial-ownership` |
| `government-trading` | Congressional trades | `senate-latest`, `house-latest`, `senate-by-symbol`, `house-by-symbol`, `senate-by-name`, `house-by-name` |
| `form13-f` | Form 13F institutional filings | `latest`, `extract`, `dates`, `holder-analytics`, `holder-performance`, `holder-industry`, `symbol-positions`, `industry-summary` |
| `fund` | ETF / fund data | `etf-holdings`, `etf-info`, `etf-sectors`, `etf-countries`, `etf-assets`, `disclosure`, `disclosure-holders-latest` |
| `fundraisers` | Crowdfunding & equity raises | `crowdfunding-latest`, `crowdfunding-search`, `crowdfunding-by-cik`, `equity-latest`, `equity-search`, `equity-by-cik` |
| `technical-indicators` | Standalone technical indicators | `sma`, `ema`, `wma`, `dema`, `tema`, `rsi`, `std-dev`, `williams`, `adx` |

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

# Insider trades for a company (recent)
fmp insider-trades search --symbol AAPL

# Beneficial owners (5%+ holders) for a company
fmp insider-trades beneficial-ownership --symbol AAPL

# Latest Senate disclosure trades
fmp government-trading senate-latest --limit 20

# Congressional trades for a stock
fmp government-trading senate-by-symbol --symbol NVDA

# Latest 13F institutional filings
fmp form13-f latest --limit 10

# Extract 13F holdings for a specific institution
fmp form13-f extract --cik 0001166559 --year 2024 --quarter 4

# Who holds AAPL in 13F filings
fmp form13-f holder-analytics --symbol AAPL --year 2024 --quarter 4

# ETF holdings breakdown
fmp fund etf-holdings --symbol SPY

# ETF sector weightings
fmp fund etf-sectors --symbol QQQ

# Latest crowdfunding filings
fmp fundraisers crowdfunding-latest --limit 20

# 20-period SMA (daily)
fmp technical-indicators sma --symbol AAPL --period 20

# 14-period RSI on hourly bars
fmp technical-indicators rsi --symbol AAPL --period 14 --timeframe 1hour
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
