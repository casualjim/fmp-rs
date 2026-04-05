## 1. Dependencies

- [ ] 1.1 Add `csv-async` (with `tokio` feature) to `[dependencies]` in `Cargo.toml`
- [ ] 1.2 Add `tokio-util` (with `io-util` feature) to `[dependencies]` in `Cargo.toml` if not already a transitive dep (check `cargo tree`)
- [ ] 1.3 Run `cargo check` to confirm dependency resolution

## 2. Error Type

- [ ] 2.1 Add `CsvParse(#[from] csv_async::Error)` variant to `FmpError` in `src/errors.rs`
- [ ] 2.2 Ensure `#[error]` message is descriptive (e.g., `"csv parse error: {0}"`)

## 3. Client — Refactor `make_reqwest_client` and `get_json`

- [ ] 3.1 Remove `Content-Type: application/json` from default headers in `make_reqwest_client` in `src/lib.rs`
- [ ] 3.2 Add `Accept: application/json` header to each request in `get_json` (`.header(ACCEPT, "application/json")`)
- [ ] 3.3 Change `resp.text().await?` to `resp.bytes().await?` in `get_json`
- [ ] 3.4 Update `map_api_error` signature to accept `&[u8]` instead of `&str`, using `String::from_utf8_lossy` internally
- [ ] 3.5 Change `serde_json::from_str(&text)` to `serde_json::from_slice(&bytes)`
- [ ] 3.6 Run `cargo test` to confirm no regressions

## 4. Rate-Limit-Aware Retry Policy

- [ ] 4.1 Create `src/retry.rs` with `RetryAfterPolicy` struct implementing `reqwest_retry::RetryPolicy`
- [ ] 4.2 On 429: parse `Retry-After` header — handle integer-seconds form and HTTP-date form
- [ ] 4.3 On 429 without `Retry-After`: fall back to exponential backoff delay
- [ ] 4.4 On 5xx, 408, network errors: delegate to exponential backoff (existing behavior)
- [ ] 4.5 Replace `ExponentialBackoff` with `RetryAfterPolicy` in `make_reqwest_client` in `src/lib.rs`
- [ ] 4.6 Add `mod retry` to `src/lib.rs` (private module)

## 5. Types — `PartParams`

- [ ] 5.1 Change `PartParams.part` from `String` to `u8` in `src/types/bulk.rs`
- [ ] 5.2 Confirm `serde_urlencoded` serializes `u8` as expected (it does — just verify with `cargo test`)

## 6. Client — Add `get_csv`

- [ ] 6.1 Add `get_csv` method to `FmpHttpClient` with signature `pub async fn get_csv<T, P>(&self, path: &str, params: &P) -> FmpResult<impl Stream<Item = FmpResult<T>> + Send>`
- [ ] 6.2 Reuse `url_with_api_key` and query-building logic from `get_json`
- [ ] 6.3 Add `Accept: application/octet-stream` header to the request in `get_csv`
- [ ] 6.4 Send the request and check status — return `Err` on non-2xx before creating the stream
- [ ] 6.5 Wrap `resp.bytes_stream()` in `tokio_util::io::StreamReader`
- [ ] 6.6 Create `csv_async::AsyncDeserializer::from_reader(reader)` and call `.into_deserialize::<T>()`
- [ ] 6.7 Map each record error to `FmpError::CsvParse` via `.map(|r| r.map_err(FmpError::from))`
- [ ] 6.8 Add required `use` imports (`tokio_util::io::StreamReader`, `futures::StreamExt`, etc.)

## 7. Macro — `define_csv_api_trait!`

- [ ] 7.1 Add `define_csv_api_trait!` to `src/macros.rs` mirroring `define_api_trait!` syntax
- [ ] 7.2 Generate trait methods returning `impl Future<Output = FmpResult<impl Stream<Item = FmpResult<$return_ty>> + Send>> + Send`
- [ ] 7.3 Generate impl body delegating to `self.get_csv($path, &params)`
- [ ] 7.4 Export with `pub(crate) use define_csv_api_trait`

## 8. Bulk Endpoints

- [ ] 8.1 Create `src/endpoints/bulk.rs` and add `pub mod bulk` to `src/endpoints/mod.rs`
- [ ] 8.2 Implement `BulkApi` trait using `define_csv_api_trait!` with all 18 endpoints:
  - `profile -> "/profile-bulk" -> PartParams -> CompanyProfile`
  - `etf_holder -> "/etf-holder-bulk" -> PartParams -> EtfHolder`
  - `eod -> "/eod-bulk" -> EodParams -> EodData`
  - `earnings_surprises -> "/earnings-surprises-bulk" -> EarningsSurpriseParams -> EarningsSurprise`
  - `income_statement -> "/income-statement-bulk" -> YearPeriodParams -> IncomeStatement`
  - `balance_sheet_statement -> "/balance-sheet-statement-bulk" -> YearPeriodParams -> BalanceSheetStatement`
  - `cash_flow_statement -> "/cash-flow-statement-bulk" -> YearPeriodParams -> CashFlowStatement`
  - `income_statement_growth -> "/income-statement-growth-bulk" -> YearPeriodParams -> IncomeStatementGrowth`
  - `balance_sheet_statement_growth -> "/balance-sheet-statement-growth-bulk" -> YearPeriodParams -> BalanceSheetGrowth`
  - `cash_flow_statement_growth -> "/cash-flow-statement-growth-bulk" -> YearPeriodParams -> CashFlowGrowth`
  - `key_metrics_ttm -> "/key-metrics-ttm-bulk" -> () -> KeyMetricsTtm`
  - `ratios_ttm -> "/ratios-ttm-bulk" -> () -> RatiosTtm`
  - `scores -> "/scores-bulk" -> () -> FinancialScore`
  - `rating -> "/rating-bulk" -> () -> StockRating`
  - `upgrades_downgrades_consensus -> "/upgrades-downgrades-consensus-bulk" -> () -> UpgradesDowngradesConsensus`
  - `price_target_summary -> "/price-target-summary-bulk" -> () -> PriceTargetSummary`
  - `peers -> "/peers-bulk" -> () -> StockPeer`
  - `dcf -> "/dcf-bulk" -> () -> DcfValuation`
- [ ] 8.3 Add all required `use` imports from `crate::types::bulk`

## 9. Wire Up Library

- [ ] 9.1 Re-export `BulkApi` from `src/lib.rs` alongside existing API traits
- [ ] 9.2 Run `cargo build` — resolve any compiler errors

## 10. CLI Bulk Commands

- [ ] 10.1 Explore existing CLI structure in `crates/fmp-cli` to understand command pattern
- [ ] 10.2 Add `bulk` subcommand group to the CLI
- [ ] 10.3 Add `profile` subcommand with `--part <u8>` and `--all` flags
- [ ] 10.4 Add `etf-holder` subcommand with `--part <u8>` and `--all` flags
- [ ] 10.5 Implement `--all` pacing logic: track start time per part, sleep `max(0, 60s − elapsed)` between parts
- [ ] 10.6 Add `eod` subcommand with `--date <YYYY-MM-DD>` flag
- [ ] 10.7 Add `earnings-surprises` subcommand with `--year <year>` flag
- [ ] 10.8 Add `income-statement` subcommand with `--year <year>` and `--period <period>` flags
- [ ] 10.9 Add `balance-sheet-statement` subcommand with `--year <year>` and `--period <period>` flags
- [ ] 10.10 Add `cash-flow-statement` subcommand with `--year <year>` and `--period <period>` flags
- [ ] 10.11 Add `income-statement-growth` subcommand with `--year <year>` and `--period <period>` flags
- [ ] 10.12 Add `balance-sheet-statement-growth` subcommand with `--year <year>` and `--period <period>` flags
- [ ] 10.13 Add `cash-flow-statement-growth` subcommand with `--year <year>` and `--period <period>` flags
- [ ] 10.14 Add `key-metrics-ttm` subcommand (no params)
- [ ] 10.15 Add `ratios-ttm` subcommand (no params)
- [ ] 10.16 Add `scores` subcommand (no params)
- [ ] 10.17 Add `rating` subcommand (no params)
- [ ] 10.18 Add `upgrades-downgrades-consensus` subcommand (no params)
- [ ] 10.19 Add `price-target-summary` subcommand (no params)
- [ ] 10.20 Add `peers` subcommand (no params)
- [ ] 10.21 Add `dcf` subcommand (no params)
- [ ] 10.22 Wire CLI output — print CSV records as JSON lines (or follow existing CLI output pattern)

## 11. Verification

- [ ] 11.1 Run `cargo clippy -- -D warnings` and fix any issues
- [ ] 11.2 Verify CSV header casing against a live FMP response; adjust `#[serde(rename)]` in `bulk.rs` if headers are not camelCase
- [ ] 11.3 Run `cargo test`
