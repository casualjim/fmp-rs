## ADDED Requirements

### Requirement: CSV streaming client method
`FmpHttpClient` SHALL expose a `get_csv` method that sends an HTTP GET request and returns `FmpResult<impl Stream<Item = FmpResult<T>> + Send>` where T is any `serde::de::DeserializeOwned + Send` type. The outer `FmpResult` SHALL resolve to an error on non-2xx HTTP status or network failure before any streaming begins. The inner stream SHALL yield one deserialized record per CSV row, with per-row deserialization failures surfaced as `FmpResult::Err`.

#### Scenario: Successful streaming response
- **WHEN** the server responds with 200 and a valid CSV body
- **THEN** `get_csv` SHALL return `Ok(stream)` and the stream SHALL yield one `Ok(T)` per CSV record

#### Scenario: HTTP error before streaming
- **WHEN** the server responds with a non-2xx status
- **THEN** `get_csv` SHALL return `Err(FmpError::Api { .. })` and no stream SHALL be returned

#### Scenario: Malformed CSV record in stream
- **WHEN** a CSV row cannot be deserialized into T
- **THEN** the stream SHALL yield `Err(FmpError::CsvParse(..))` for that record and continue with subsequent records

#### Scenario: No query parameters
- **WHEN** params is `()` (unit type)
- **THEN** no query parameters beyond `apikey` SHALL be appended to the URL

### Requirement: CSV parse error variant
`FmpError` SHALL include a `CsvParse` variant wrapping a `csv_async::Error` to represent record-level deserialization failures from bulk CSV responses.

#### Scenario: CsvParse surfaces in stream
- **WHEN** a CSV record fails to deserialize
- **THEN** the yielded error SHALL be `FmpError::CsvParse` (not a generic IO or API error)

### Requirement: Bulk API trait
A `BulkApi` trait SHALL be generated via `define_csv_api_trait!` and implemented for `FmpHttpClient`, covering all 18 FMP bulk endpoints. Each method SHALL return `impl Future<Output = FmpResult<impl Stream<Item = FmpResult<T>> + Send>> + Send`.

#### Scenario: Profile bulk with part parameter
- **WHEN** `profile(PartParams { part: 0 })` is called on `BulkApi`
- **THEN** it SHALL issue a GET to `/profile-bulk?part=0&apikey=...` and return a stream of `CompanyProfile` records

#### Scenario: Income statement bulk with year and period
- **WHEN** `income_statement(YearPeriodParams { year: "2024".into(), period: "FY".into() })` is called on `BulkApi`
- **THEN** it SHALL issue a GET to `/income-statement-bulk?year=2024&period=FY&apikey=...` and return a stream of `IncomeStatement` records

#### Scenario: Key metrics TTM with no parameters
- **WHEN** `key_metrics_ttm(())` is called on `BulkApi`
- **THEN** it SHALL issue a GET to `/key-metrics-ttm-bulk?apikey=...` and return a stream of `KeyMetricsTtm` records

#### Scenario: EOD with date parameter
- **WHEN** `eod(EodParams { date: "2024-10-22".into() })` is called on `BulkApi`
- **THEN** it SHALL issue a GET to `/eod-bulk?date=2024-10-22&apikey=...` and return a stream of `EodData` records

### Requirement: CSV trait macro
A `define_csv_api_trait!` macro SHALL generate a trait and its `FmpHttpClient` implementation where each method delegates to `self.get_csv(path, &params)`. The macro syntax SHALL mirror `define_api_trait!`.

#### Scenario: Macro generates correct delegation
- **WHEN** `define_csv_api_trait!(MyApi, foo -> "/foo" -> MyParams -> MyType,)` is used
- **THEN** `FmpHttpClient::foo(&self, params: MyParams)` SHALL call `self.get_csv("/foo", &params)`

### Requirement: JSON client method uses byte-slice parsing
`FmpHttpClient::get_json` SHALL read the response body as `bytes::Bytes` and parse with `serde_json::from_slice`, avoiding an intermediate `String` allocation on the success path.

#### Scenario: Successful JSON response parsed from bytes
- **WHEN** the server responds with 200 and a valid JSON body
- **THEN** `get_json` SHALL deserialize directly from the byte slice without constructing a UTF-8 String

#### Scenario: Error response still surfaces message
- **WHEN** the server responds with a non-2xx status
- **THEN** `get_json` SHALL still extract and surface the FMP error message from the response body

### Requirement: PartParams uses u8
`PartParams.part` SHALL be typed as `u8`. FMP documents exactly 4 parts (0–3) for part-based bulk endpoints.

#### Scenario: Part serialized as integer string
- **WHEN** `PartParams { part: 2 }` is serialized as query parameters
- **THEN** the URL SHALL contain `part=2`

### Requirement: Rate-limit-aware retry policy
The HTTP client SHALL use a custom retry policy that checks for a `Retry-After` header on 429 responses and waits the indicated duration before retrying. The policy SHALL handle both integer-seconds and HTTP-date forms of `Retry-After`. When no `Retry-After` header is present, the policy SHALL fall back to exponential backoff. Non-429 transient errors (5xx, 408, network failures) SHALL use exponential backoff.

#### Scenario: Retry-After integer seconds respected
- **WHEN** a 429 response includes `Retry-After: 60`
- **THEN** the client SHALL wait at least 60 seconds before retrying

#### Scenario: Retry-After HTTP-date respected
- **WHEN** a 429 response includes `Retry-After: Wed, 21 Oct 2026 07:28:00 GMT`
- **THEN** the client SHALL wait until that time before retrying

#### Scenario: 429 without Retry-After falls back to backoff
- **WHEN** a 429 response has no `Retry-After` header
- **THEN** the client SHALL retry using exponential backoff

#### Scenario: 5xx uses exponential backoff
- **WHEN** the server responds with a 5xx error
- **THEN** the client SHALL retry using exponential backoff (unchanged from current behavior)

### Requirement: CLI bulk commands with --all flag
The CLI SHALL expose bulk subcommands for all 18 bulk endpoints. For the two part-based endpoints (`profile-bulk`, `etf-holder-bulk`), the CLI SHALL support both `--part <0-3>` (single part) and `--all` (all 4 parts sequentially). When `--all` is used, the CLI SHALL pace requests by tracking elapsed time since the previous request started and sleeping `max(0, 60s − elapsed)` between parts.

#### Scenario: Single part fetch
- **WHEN** the user runs `fmp bulk profile --part 1`
- **THEN** the CLI SHALL fetch and output part 1 only

#### Scenario: All parts fetch with pacing
- **WHEN** the user runs `fmp bulk profile --all`
- **THEN** the CLI SHALL fetch parts 0, 1, 2, 3 in sequence, sleeping between each to respect the 60-second rate limit

#### Scenario: EOD by date
- **WHEN** the user runs `fmp bulk eod --date 2024-10-22`
- **THEN** the CLI SHALL fetch and output EOD data for that date

#### Scenario: Earnings surprises by year
- **WHEN** the user runs `fmp bulk earnings-surprises --year 2024`
- **THEN** the CLI SHALL fetch and output earnings surprises for that year

#### Scenario: Income statement by year and period
- **WHEN** the user runs `fmp bulk income-statement --year 2024 --period FY`
- **THEN** the CLI SHALL fetch and output income statements for that year and period

#### Scenario: Balance sheet statement by year and period
- **WHEN** the user runs `fmp bulk balance-sheet-statement --year 2024 --period Q1`
- **THEN** the CLI SHALL fetch and output balance sheet statements for that year and period

#### Scenario: Cash flow statement by year and period
- **WHEN** the user runs `fmp bulk cash-flow-statement --year 2024 --period FY`
- **THEN** the CLI SHALL fetch and output cash flow statements for that year and period

#### Scenario: Income statement growth by year and period
- **WHEN** the user runs `fmp bulk income-statement-growth --year 2024 --period FY`
- **THEN** the CLI SHALL fetch and output income statement growth data for that year and period

#### Scenario: Balance sheet statement growth by year and period
- **WHEN** the user runs `fmp bulk balance-sheet-statement-growth --year 2024 --period FY`
- **THEN** the CLI SHALL fetch and output balance sheet growth data for that year and period

#### Scenario: Cash flow statement growth by year and period
- **WHEN** the user runs `fmp bulk cash-flow-statement-growth --year 2024 --period FY`
- **THEN** the CLI SHALL fetch and output cash flow growth data for that year and period

#### Scenario: Key metrics TTM with no parameters
- **WHEN** the user runs `fmp bulk key-metrics-ttm`
- **THEN** the CLI SHALL fetch and output key metrics TTM data for all companies

#### Scenario: Ratios TTM with no parameters
- **WHEN** the user runs `fmp bulk ratios-ttm`
- **THEN** the CLI SHALL fetch and output ratios TTM data for all companies

#### Scenario: Scores with no parameters
- **WHEN** the user runs `fmp bulk scores`
- **THEN** the CLI SHALL fetch and output financial scores for all companies

#### Scenario: Rating with no parameters
- **WHEN** the user runs `fmp bulk rating`
- **THEN** the CLI SHALL fetch and output stock ratings for all companies

#### Scenario: Upgrades downgrades consensus with no parameters
- **WHEN** the user runs `fmp bulk upgrades-downgrades-consensus`
- **THEN** the CLI SHALL fetch and output analyst consensus data for all companies

#### Scenario: Price target summary with no parameters
- **WHEN** the user runs `fmp bulk price-target-summary`
- **THEN** the CLI SHALL fetch and output price target summaries for all companies

#### Scenario: Peers with no parameters
- **WHEN** the user runs `fmp bulk peers`
- **THEN** the CLI SHALL fetch and output stock peer data for all companies

#### Scenario: DCF with no parameters
- **WHEN** the user runs `fmp bulk dcf`
- **THEN** the CLI SHALL fetch and output DCF valuations for all companies
