## Why

FMP's bulk endpoints deliver full-market datasets (profiles, financials, ratings, EOD prices, etc.) as CSV files — typically 10–20 MB per response. The client currently has no support for these endpoints, and its retry middleware retries 429 responses using exponential backoff without reading `Retry-After` headers, meaning retries can fire too early. Adding streaming bulk support and correct rate-limit retry handling enables use cases like database population, backtesting, and screening that require whole-market data.

## What Changes

- Add `csv-async` and `tokio-util` as dependencies for async CSV streaming
- Add `get_csv` method to `FmpHttpClient` returning `FmpResult<impl Stream<Item = FmpResult<T>>>`
- Add `FmpError::CsvParse` variant for CSV deserialization errors
- Fix `get_json` to use `bytes()` + `serde_json::from_slice` instead of `text()` + `from_str`
- Add `define_csv_api_trait!` macro generating stream-returning trait methods
- Add `src/endpoints/bulk.rs` with `BulkApi` trait covering all 18 FMP bulk endpoints
- Change `PartParams.part` from `String` to `u8` (valid values: 0–3)
- Add custom `RetryPolicy` implementation that reads `Retry-After` headers on 429 responses
- Add CLI bulk commands with `--all` flag for part-based endpoints (profile, ETF holder) that sequences parts 0–3 with rate-limit-aware pacing
- Re-export `BulkApi` from `lib.rs`

## Capabilities

### New Capabilities

- `bulk-csv-streaming`: Streaming CSV ingestion via `get_csv`, the `define_csv_api_trait!` macro, and the `BulkApi` trait covering all 18 FMP bulk endpoints
- `rate-limit-retry`: Custom retry policy that respects `Retry-After` response headers on 429s, replacing naive exponential backoff for rate-limited responses

### Modified Capabilities

- (none — `get_json` byte-slice fix and `PartParams` type change are internal details)

## Impact

- **`src/client.rs`**: new `get_csv` method; `get_json` byte-slice fix; wire custom retry policy
- **`src/errors.rs`**: new `CsvParse` error variant
- **`src/macros.rs`**: new `define_csv_api_trait!` macro
- **`src/retry.rs`**: new file — `RetryAfterPolicy` custom retry policy
- **`src/endpoints/bulk.rs`**: new file, 18 endpoints
- **`src/endpoints/mod.rs`**: add `pub mod bulk`
- **`src/lib.rs`**: re-export `BulkApi`
- **`src/types/bulk.rs`**: change `PartParams.part: String` → `u8`
- **`Cargo.toml`**: add `csv-async`, `tokio-util` (with `io-util` feature)
- **`crates/fmp-cli`**: add bulk subcommands with `--all` flag for part-based endpoints
