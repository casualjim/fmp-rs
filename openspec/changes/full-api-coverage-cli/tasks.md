## 1. New CLI module: insider-trades

- [x] 1.1 Read `src/types/insider_trades.rs` to understand all param structs
- [x] 1.2 Create `crates/fmp-cli/src/cli/insider_trades.rs` with `InsiderTradesArgs` enum and all subcommand structs (`Latest`, `Search`, `ByName`, `TransactionTypes`, `Statistics`, `BeneficialOwnership`)
- [x] 1.3 Add `pub mod insider_trades;` and `pub use insider_trades::InsiderTradesArgs;` to `cli/mod.rs`
- [x] 1.4 Add `InsiderTrades(crate::cli::InsiderTradesArgs)` variant to `Commands` enum in `config.rs`
- [x] 1.5 Add match arm for `InsiderTrades` in `cli/mod.rs#dispatch()`

## 2. New CLI module: government-trading

- [x] 2.1 Read `src/types/government_trading.rs` to understand `PaginationParams`, `SymbolParams`, `NameParams`
- [x] 2.2 Create `crates/fmp-cli/src/cli/government_trading.rs` with `GovernmentTradingArgs` enum and all subcommand structs (`SenateLatest`, `HouseLatest`, `SenateBySymbol`, `SenateByName`, `HouseBySymbol`, `HouseByName`)
- [x] 2.3 Add `pub mod government_trading;` and re-export to `cli/mod.rs`
- [x] 2.4 Add `GovernmentTrading(crate::cli::GovernmentTradingArgs)` variant to `Commands` in `config.rs`
- [x] 2.5 Add match arm in `dispatch()`

## 3. New CLI module: form-13f

- [x] 3.1 Read `src/types/form_13f.rs` to understand all param structs
- [x] 3.2 Create `crates/fmp-cli/src/cli/form_13f.rs` with `Form13FArgs` enum and subcommand structs (`Latest`, `Extract`, `Dates`, `HolderAnalytics`, `HolderPerformance`, `HolderIndustry`, `SymbolPositions`, `IndustrySummary`)
- [x] 3.3 Add `pub mod form_13f;` and re-export to `cli/mod.rs`
- [x] 3.4 Add `Form13F(crate::cli::Form13FArgs)` variant to `Commands` in `config.rs`
- [x] 3.5 Add match arm in `dispatch()`

## 4. New CLI module: fund

- [x] 4.1 Read `src/types/fund.rs` to understand all param structs
- [x] 4.2 Create `crates/fmp-cli/src/cli/fund.rs` with `FundArgs` enum and subcommand structs (`EtfHoldings`, `EtfInfo`, `EtfCountries`, `EtfAssets`, `EtfSectors`, `DisclosureHoldersLatest`, `DisclosureHoldersSearch`, `DisclosureDates`, `Disclosure`)
- [x] 4.3 Add `pub mod fund;` and re-export to `cli/mod.rs`
- [x] 4.4 Add `Fund(crate::cli::FundArgs)` variant to `Commands` in `config.rs`
- [x] 4.5 Add match arm in `dispatch()`

## 5. New CLI module: fundraisers

- [x] 5.1 Read `src/types/fundraisers.rs` to understand all param structs
- [x] 5.2 Create `crates/fmp-cli/src/cli/fundraisers.rs` with `FundraisersArgs` enum and subcommand structs (`CrowdfundingLatest`, `CrowdfundingSearch`, `CrowdfundingByCik`, `EquityLatest`, `EquitySearch`, `EquityByCik`)
- [x] 5.3 Add `pub mod fundraisers;` and re-export to `cli/mod.rs`
- [x] 5.4 Add `Fundraisers(crate::cli::FundraisersArgs)` variant to `Commands` in `config.rs`
- [x] 5.5 Add match arm in `dispatch()`

## 6. New CLI module: technical-indicators

- [x] 6.1 Read `src/types/technical_indicators.rs` to understand all param structs
- [x] 6.2 Create `crates/fmp-cli/src/cli/technical_indicators.rs` with `TechnicalIndicatorsArgs` enum and subcommand structs (`Sma`, `Ema`, `Wma`, `Dema`, `Tema`, `Rsi`, `StdDev`, `Williams`, `Adx`)
- [x] 6.3 Add `pub mod technical_indicators;` and re-export to `cli/mod.rs`
- [x] 6.4 Add `TechnicalIndicators(crate::cli::TechnicalIndicatorsArgs)` variant to `Commands` in `config.rs`
- [x] 6.5 Add match arm in `dispatch()`

## 7. Partial gap: calendar

- [x] 7.1 Read `src/types/calendar.rs` to check param types for `dividends`, `ipos_disclosure`, `ipos_prospectus`
- [x] 7.2 Add `Dividends`, `IposDisclosure`, `IposProspectus` variants to `CalendarArgs` in `cli/calendar.rs`
- [x] 7.3 Implement `handle()` for each new variant

## 8. Partial gap: analyst

- [x] 8.1 Read `src/types/analyst.rs` to check param types for missing functions
- [x] 8.2 Add `PriceTargetNews`, `PriceTargetLatestNews`, `GradesNews`, `GradesLatestNews`, `GradesHistorical` variants to `AnalystArgs` in `cli/analyst.rs`
- [x] 8.3 Implement `handle()` for each new variant

## 9. Partial gap: news

- [x] 9.1 Read `src/types/news.rs` to check param types for missing search functions
- [x] 9.2 Add `SearchPressReleases`, `SearchCrypto`, `SearchForex` variants to `NewsArgs` in `cli/news.rs`
- [x] 9.3 Implement `handle()` for each new variant

## 10. Partial gap: sec-filings

- [x] 10.1 Read `src/types/sec_filings.rs` to check param types for missing functions
- [x] 10.2 Add `SecProfile`, `IndustryClassificationList`, `IndustryClassificationSearch`, `IndustryClassificationAll` variants to `FilingsArgs` in `cli/filings.rs`
- [x] 10.3 Implement `handle()` for each new variant

## 11. Partial gap: company

- [x] 11.1 Read `src/types/company.rs` to check param types for missing functions
- [x] 11.2 Add `Notes`, `Delisted`, `ExecutiveCompensationBenchmark` variants to `CompanyArgs` in `cli/company.rs`
- [x] 11.3 Implement `handle()` for each new variant

## 12. Partial gap: crypto and forex 1-hour chart

- [x] 12.1 Verify `historical_chart_1hour` param type in `src/types/crypto.rs` and `src/types/forex.rs`
- [x] 12.2 Add `Chart1hour` variant to `CryptoArgs` in `cli/crypto.rs` and implement `handle()`
- [x] 12.3 Add `Chart1hour` variant to `ForexArgs` in `cli/forex.rs` and implement `handle()`

## 13. Build and verify

- [x] 13.1 Run `mise build:debug` — fix any compile errors
- [x] 13.2 Run `mise test` — ensure all tests pass
- [x] 13.3 Smoke-test `fmp --help` to confirm all new subcommands appear
