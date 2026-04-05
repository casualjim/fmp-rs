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
- [x] 8.4 Implement `crypto news` for crypto-specific news

## 9. Forex Namespace Implementation

- [x] 9.1 Implement `forex rate` command for exchange rates
- [x] 9.2 Implement `forex historical` with date range support
- [x] 9.3 Implement `forex list` for available pairs
- [x] 9.4 Implement `forex news` for forex market news

## 10. Commodities Namespace Implementation

- [x] 10.1 Implement `commodities price` command for commodity spot prices
- [x] 10.2 Implement `commodities historical` with date range support

## 11. Indexes Namespace Implementation

- [x] 11.1 Implement `indexes quote` command for index values
- [x] 11.2 Implement `indexes historical` with date range support
- [x] 11.3 Implement `indexes constituents` for index member lists
- [x] 11.4 Implement `indexes available` for list of available indexes

## 12. Market Hours Namespace Implementation

- [x] 12.1 Implement `market-hours exchange` for market hours by exchange
- [x] 12.2 Implement `market-hours holidays` with --from and --to flags
- [x] 12.3 Implement `market-hours all` for all exchanges market hours

## 13. Market Performance Namespace Implementation

- [x] 13.1 Implement `market-performance sector-performance-snapshot` for sector performance
- [x] 13.2 Implement `market-performance industry-performance-snapshot` for industry breakdown
- [x] 13.3 Implement `market-performance biggest-gainers` and `biggest-losers` commands
- [x] 13.4 Implement `market-performance most-actives` command
- [x] 13.5 Implement historical sector/industry performance commands
- [x] 13.6 Implement sector/industry PE snapshot commands

## 14. Search Namespace Implementation

- [x] 14.1 Implement `search symbol` for symbol search
- [x] 14.2 Implement `search name` for company name search
- [x] 14.3 Implement `search cik` for CIK lookup
- [x] 14.4 Implement `search cusip` for CUSIP lookup
- [x] 14.5 Implement `search isin` for ISIN lookup
- [x] 14.6 Implement `search exchange-variants` for exchange variant search
- [x] 14.7 Implement `search screener` for stock screening with filters
- [x] 14.8 Add --limit flag for all search commands

## 15. Directory Namespace Implementation

- [x] 15.1 Implement `directory stock-list` for company listings
- [x] 15.2 Implement `directory etf-list` for ETF listings
- [x] 15.3 Implement `directory financial-statement-symbols` for financial statement symbols
- [x] 15.4 Implement `directory cik-list` for CIK listings
- [x] 15.5 Implement `directory symbol-change` for symbol changes
- [x] 15.6 Implement `directory actively-trading-list` for actively trading stocks
- [x] 15.7 Implement `directory earnings-transcript-list` for earnings transcript list
- [x] 15.8 Implement `directory available-exchanges` for available exchanges
- [x] 15.9 Implement `directory available-sectors` for sector listings
- [x] 15.10 Implement `directory available-industries` for industry listings
- [x] 15.11 Implement `directory available-countries` for country listings

## 16. News Namespace Implementation

- [x] 16.1 Implement `news latest` for general market news
- [x] 16.2 Implement `news stock` for stock-specific news
- [x] 16.3 Implement `news crypto` for crypto news
- [x] 16.4 Implement `news forex` for forex news
- [x] 16.5 Add --tickers, --from, --to, --limit flags for filtering
- [x] 16.6 Implement `news search` for keyword search

## 17. Calendar Namespace Implementation

- [x] 17.1 Implement `calendar earnings` with date range support
- [x] 17.2 Implement `calendar earnings-confirmed` for confirmed dates
- [x] 17.3 Implement `calendar ipos` for IPO calendar
- [x] 17.4 Implement `calendar splits` for stock split calendar

## 18. Economics Namespace Implementation

- [x] 18.1 Implement `economics indicators` for key economic indicators
- [x] 18.2 Implement `economics treasury-rates` for yield curve
- [x] 18.3 Implement `economics federal-fund-rate` historical data
- [x] 18.4 Add --from and --to flags for historical data

## 19. Analyst Namespace Implementation

- [x] 19.1 Implement `analyst recommendations` for buy/sell ratings
- [x] 19.2 Implement `analyst price-target` for consensus targets
- [x] 19.3 Implement `analyst estimates` for earnings estimates
- [x] 19.4 Add --period and --limit flags

## 20. SEC Filings Namespace Implementation

- [x] 20.1 Implement `filings search` for SEC filing search
- [x] 20.2 Implement `filings by-type` filtered by form type (10-K, 10-Q, 8-K, etc.)
- [x] 20.3 Implement `filings by-symbol` for company-specific filings
- [x] 20.4 Implement `filings cik` for CIK number lookup
- [x] 20.5 Add --from, --to, --limit flags for filtering

## 21. Earnings Transcript Namespace Implementation

- [x] 21.1 Implement `transcript latest` for most recent transcript
- [x] 21.2 Implement `transcript by-symbol` for company transcripts
- [x] 21.3 Implement `transcript search` for keyword search
- [x] 21.4 Add --quarter and --year flags for specific periods

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

- [x] 30.1 Implement JSON pretty-printing with optional color
- [ ] 30.2 Add summary statistics where appropriate

## 31. Testing

- [ ] 31.1 Write unit tests for configuration management
- [ ] 31.2 Write unit tests for API key resolution logic
- [ ] 31.3 Write unit tests for output formatting
- [ ] 31.4 Write integration tests for key namespaces (quotes, chart, company)
- [ ] 31.5 Test cross-platform configuration paths
- [ ] 31.6 Test output format detection (TTY vs pipe)

## 32. Documentation and Polish

- [x] 32.1 Write README.md with installation instructions
- [x] 32.2 Add usage examples for common workflows
- [x] 32.3 Generate shell completion scripts (bash, zsh, fish)
- [ ] 32.4 Write CHANGELOG.md for version tracking
- [ ] 32.5 Add inline code documentation for public APIs
- [x] 32.6 Create example config file template

## 33. Release Preparation

- [ ] 33.1 Add CI/CD workflow for automated testing
- [ ] 33.2 Configure release automation for crates.io publishing
- [ ] 33.3 Test installation via cargo install
- [ ] 33.4 Verify binary size and optimize if needed
- [ ] 33.5 Create release notes for v0.1.0
