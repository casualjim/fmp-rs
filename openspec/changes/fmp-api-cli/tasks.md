## 1. Project Setup

- [x] 1.1 Create new binary crate at `crates/fmp-cli` in workspace
- [x] 1.2 Add CLI dependencies to Cargo.toml (clap with derive+env, dirs, secrecy, tokio, eyre, serde_json, toml, owo-colors)
- [x] 1.3 Add fmp library as workspace dependency
- [x] 1.4 Set up basic binary entry point with tokio main and mimalloc
- [x] 1.5 Configure Cargo workspace to include new crate

## 2. Core Infrastructure

- [x] 2.1 Create config.rs with clap Parser derive and environment variable support
- [x] 2.2 Implement FmpConfig struct with api_key (SecretString), base_url fields
- [x] 2.3 Implement load_config function with layered loading (CLI → env → project file → user file → defaults)
- [x] 2.4 Implement config file save/load with dirs for cross-platform paths
- [x] 2.5 Create API client factory using FmpConfig and secrecy for API key
- [x] 2.6 Implement JSON output formatting to stdout
- [x] 2.7 Create error handling with eyre and context chains
- [x] 2.8 Implement exit code logic (0 success, 1 API error, 2 argument error)
- [x] 2.9 Add owo-colors for colored error messages

## 3. CLI Framework Setup

- [x] 3.1 Define clap command structure with Cli root struct
- [x] 3.2 Implement global flags (--config-file, --api-key, --base-url)
- [x] 3.3 Create Commands enum with Subcommand derive for all namespaces
- [x] 3.4 Implement --version and --help handlers
- [x] 3.5 Set up command dispatch logic based on parsed subcommand
- [x] 3.6 Add mimalloc global allocator for performance

## 4. Quotes Namespace Implementation

- [x] 4.1 Implement `quotes get` command for full quotes (single and multiple symbols)
- [x] 4.2 Implement `quotes short` command for lightweight quotes
- [x] 4.3 Implement `quotes aftermarket-trade` and `aftermarket-quote` commands
- [x] 4.4 Implement `quotes price-change` command
- [x] 4.5 Implement `quotes batch`, `batch-short`, `batch-aftermarket` commands
- [x] 4.6 Implement `quotes exchange` command for exchange-filtered quotes
- [x] 4.7 Implement asset class commands: mutual-fund, etf, commodity, crypto, forex, index

## 5. Chart Namespace Implementation

- [x] 5.1 Implement `chart historical` command with date range support
- [x] 5.2 Add --interval flag support (1min, 5min, 15min, 30min, 1h, 4h, daily)
- [x] 5.3 Implement --from and --to date filters
- [x] 5.4 Implement `chart intraday` command for same-day data
- [x] 5.5 Add technical indicator support (--sma, --ema, --rsi, etc.)

## 6. Company Namespace Implementation

- [x] 6.1 Implement `company profile` command
- [x] 6.2 Implement `company executive` command
- [x] 6.3 Implement `company market-cap` command
- [x] 6.4 Implement `company symbols` command
- [x] 6.5 Implement `company is-the-market-open` status command

## 7. Statements Namespace Implementation

- [x] 7.1 Implement `statements income` command with --period flag (annual, quarterly)
- [x] 7.2 Implement `statements balance-sheet` command with period support
- [x] 7.3 Implement `statements cash-flow` command with period support
- [x] 7.4 Implement `statements reported-financials` for as-reported data
- [x] 7.5 Implement `statements financial-growth` for growth metrics
- [x] 7.6 Implement `statements revenue-segmentation` command
- [x] 7.7 Add --limit and --consolidated flags for all statement commands

## 8. Crypto Namespace Implementation

- [x] 8.1 Implement `crypto price` command for crypto quotes
- [x] 8.2 Implement `crypto historical` with date range and interval support
- [x] 8.3 Implement `crypto list` for available trading pairs
- [ ] 8.4 Implement `crypto news` for crypto-specific news

## 9. Forex Namespace Implementation

- [x] 9.1 Implement `forex rate` command for exchange rates
- [x] 9.2 Implement `forex historical` with date range support
- [x] 9.3 Implement `forex list` for available pairs
- [ ] 9.4 Implement `forex news` for forex market news

## 10. Commodities Namespace Implementation

- [x] 10.1 Implement `commodities price` command for commodity spot prices
- [x] 10.2 Implement `commodities historical` with date range support

## 11. Indexes Namespace Implementation

- [x] 11.1 Implement `indexes quote` command for index values
- [x] 11.2 Implement `indexes historical` with date range support
- [x] 11.3 Implement `indexes constituents` for index member lists
- [x] 11.4 Implement `indexes available` for list of available indexes

## 12. Market Hours Namespace Implementation

- [ ] 12.1 Implement `market-hours status` for current market open/close
- [ ] 12.2 Implement `market-hours holidays` with --year flag
- [ ] 12.3 Implement `market-hours is-open` for quick check

## 13. Market Performance Namespace Implementation

- [ ] 13.1 Implement `market-performance sector` for sector performance
- [ ] 13.2 Implement `market-performance industries` for industry breakdown
- [ ] 13.3 Implement `market-performance gainers` and `losers` commands
- [ ] 13.4 Add --limit flag for top N results

## 14. Search Namespace Implementation

- [ ] 14.1 Implement `search ticker` for symbol search
- [ ] 14.2 Implement `search name` for company name search
- [ ] 14.3 Implement `search isin` for ISIN lookup
- [ ] 14.4 Implement `search cusip` for CUSIP lookup
- [ ] 14.5 Implement `search symbol` for exact symbol match
- [ ] 14.6 Add --limit flag for all search commands

## 15. Directory Namespace Implementation

- [ ] 15.1 Implement `directory companies` for company listings
- [ ] 15.2 Implement `directory etfs` for ETF listings
- [ ] 15.3 Implement `directory trades` for trading data
- [ ] 15.4 Implement `directory index` for index listings
- [ ] 15.5 Implement `directory commodities` for commodity listings
- [ ] 15.6 Implement `directory etf-screener` for ETF screening
- [ ] 15.7 Implement `directory crypto` for crypto listings

## 16. News Namespace Implementation

- [ ] 16.1 Implement `news latest` for general market news
- [ ] 16.2 Implement `news stock` for stock-specific news
- [ ] 16.3 Implement `news crypto` for crypto news
- [ ] 16.4 Implement `news forex` for forex news
- [ ] 16.5 Add --tickers, --from, --to, --limit flags for filtering
- [ ] 16.6 Implement `news search` for keyword search

## 17. Calendar Namespace Implementation

- [ ] 17.1 Implement `calendar earnings` with date range support
- [ ] 17.2 Implement `calendar earnings-confirmed` for confirmed dates
- [ ] 17.3 Implement `calendar ipos` for IPO calendar
- [ ] 17.4 Implement `calendar splits` for stock split calendar

## 18. Economics Namespace Implementation

- [ ] 18.1 Implement `economics indicators` for key economic indicators
- [ ] 18.2 Implement `economics treasury-rates` for yield curve
- [ ] 18.3 Implement `economics federal-fund-rate` historical data
- [ ] 18.4 Add --from and --to flags for historical data

## 19. Analyst Namespace Implementation

- [ ] 19.1 Implement `analyst recommendations` for buy/sell ratings
- [ ] 19.2 Implement `analyst price-target` for consensus targets
- [ ] 19.3 Implement `analyst estimates` for earnings estimates
- [ ] 19.4 Add --period and --limit flags

## 20. SEC Filings Namespace Implementation

- [ ] 20.1 Implement `filings search` for SEC filing search
- [ ] 20.2 Implement `filings by-type` filtered by form type (10-K, 10-Q, 8-K, etc.)
- [ ] 20.3 Implement `filings by-symbol` for company-specific filings
- [ ] 20.4 Implement `filings cik` for CIK number lookup
- [ ] 20.5 Add --from, --to, --limit flags for filtering

## 21. Earnings Transcript Namespace Implementation

- [ ] 21.1 Implement `transcript latest` for most recent transcript
- [ ] 21.2 Implement `transcript by-symbol` for company transcripts
- [ ] 21.3 Implement `transcript search` for keyword search
- [ ] 21.4 Add --quarter and --year flags for specific periods

## 22. ESG Namespace Implementation

- [x] 22.1 Implement `esg scores` for ESG ratings
- [x] 22.2 Implement `esg historical` for historical ESG data
- [x] 22.3 Add --limit flag for multiple results

## 23. DCF Namespace Implementation

- [x] 23.1 Implement `dcf valuation` for DCF fair value
- [x] 23.2 Implement `dcf historical` for historical valuations

## 29. COT Namespace Implementation

- [x] 29.1 Implement `cot report` for Commitment of Traders data
- [x] 29.2 Implement `cot analysis` for COT analysis

## 30. Output Enhancement

- [ ] 30.1 Implement JSON pretty-printing with optional color
- [ ] 30.2 Add summary statistics where appropriate

## 31. Testing

- [ ] 31.1 Write unit tests for configuration management
- [ ] 31.2 Write unit tests for API key resolution logic
- [ ] 31.3 Write unit tests for output formatting
- [ ] 31.4 Write integration tests for key namespaces (quotes, chart, company)
- [ ] 31.5 Test cross-platform configuration paths
- [ ] 31.6 Test output format detection (TTY vs pipe)

## 32. Documentation and Polish

- [ ] 32.1 Write README.md with installation instructions
- [ ] 32.2 Add usage examples for common workflows
- [ ] 32.3 Generate shell completion scripts (bash, zsh, fish)
- [ ] 32.4 Write CHANGELOG.md for version tracking
- [ ] 32.5 Add inline code documentation for public APIs
- [ ] 32.6 Create example config file template

## 33. Release Preparation

- [ ] 33.1 Add CI/CD workflow for automated testing
- [ ] 33.2 Configure release automation for crates.io publishing
- [ ] 33.3 Test installation via cargo install
- [ ] 33.4 Verify binary size and optimize if needed
- [ ] 33.5 Create release notes for v0.1.0
