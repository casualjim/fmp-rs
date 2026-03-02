## Why

The fmp-rs library provides comprehensive Rust bindings for the Financial Modeling Prep API, but lacks a command-line interface for quick access to financial data. A CLI would enable rapid data exploration, scripting, and testing without writing code, making the API accessible to analysts, developers, and power users who prefer command-line workflows.

## What Changes

- Create a new CLI binary crate (`fmp-cli`) that wraps the existing fmp library
- Implement namespaced command structure mirroring the API categories (e.g., `fmp quotes get AAPL`, `fmp crypto price BTCUSD`)
- Add configuration management for API key storage and default settings
- Support JSON output format for programmatic processing
- Provide interactive and non-interactive modes for different use cases

## Capabilities

### New Capabilities

- `cli-core`: CLI infrastructure using clap, API key management with secrecy, output formatting
- `cli-quotes`: Commands for real-time and batch quotes (QuotesApi)
- `cli-chart`: Commands for historical chart data with intervals and technical indicators (ChartApi, TechnicalIndicatorsApi)
- `cli-company`: Commands for company profiles, executives, and fundamentals (CompanyApi)
- `cli-statements`: Commands for financial statements - income, balance sheet, cash flow (StatementsApi)
- `cli-crypto`: Commands for cryptocurrency prices and market data (CryptoApi)
- `cli-forex`: Commands for foreign exchange rates and conversion (ForexApi)
- `cli-commodities`: Commands for commodity prices (CommodityApi)
- `cli-indexes`: Commands for market index data and constituents (IndexesApi)
- `cli-market-hours`: Commands for trading hours and market schedules (MarketHoursApi)
- `cli-market-performance`: Commands for market performance and movers (MarketPerformanceApi)
- `cli-search`: Commands for symbol and company search (SearchApi)
- `cli-directory`: Commands for symbol directories and listings (DirectoryApi)
- `cli-news`: Commands for market and company news (NewsApi)
- `cli-calendar`: Commands for earnings and economic calendars (CalendarApi)
- `cli-economics`: Commands for economic indicators and treasury rates (EconomicsApi)
- `cli-analyst`: Commands for analyst ratings and price targets (AnalystApi)
- `cli-sec-filings`: Commands for SEC filing access (SecFilingsApi)
- `cli-earnings-transcript`: Commands for earnings call transcripts (EarningsTranscriptApi)
- `cli-esg`: Commands for ESG scores and ratings (EsgApi)
- `cli-dcf`: Commands for discounted cash flow valuations (DcfApi)
- `cli-insider-trades`: Commands for insider trading data (InsiderTradesApi)
- `cli-form-13f`: Commands for institutional holdings (Form13FApi)
- `cli-fund`: Commands for ETF and mutual fund data (FundApi)
- `cli-fundraisers`: Commands for IPO and fundraising data (FundraisersApi)
- `cli-government-trading`: Commands for political trading activity (GovernmentTradingApi)
- `cli-cot`: Commands for Commitment of Traders reports (CotApi)

### Modified Capabilities

None - this is a new addition that consumes the existing library API without modifying its requirements.

## Impact

**Code**: New `crates/fmp-cli` binary crate with dependencies on `fmp` library, `clap` for argument parsing, and table rendering libraries

**APIs**: No changes to existing library APIs - CLI is a consumer

**Dependencies**: 
- `clap` with derive + env features (CLI framework)
- `dirs` (cross-platform directory paths)
- `secrecy` (secure API key handling)
- `tokio` with full features (async runtime)
- `eyre` (error handling)
- `serde_json` (JSON output)
- `toml` (config file format)
- `owo-colors` (colored output)

**Systems**: 
- Configuration in `~/.config/fmp-cli/config.toml` (Linux), `~/Library/Application Support/fmp-cli/config.toml` (macOS), `%APPDATA%\fmp-cli\config.toml` (Windows)
- API key via `FMP_API_KEY` env var, `--api-key` flag, or config file
- Token cache in local data directory
- Output to stdout/stderr following Unix conventions
