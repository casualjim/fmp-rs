## ADDED Requirements

### Requirement: Calendar missing subcommands
The `calendar` CLI module SHALL expose `dividends`, `ipos-disclosure`, and `ipos-prospectus` subcommands in addition to the existing ones.

#### Scenario: Dividend calendar
- **WHEN** user runs `fmp calendar dividends`
- **THEN** the CLI calls `CalendarApi::dividends` and prints JSON to stdout

#### Scenario: Dividend calendar with date range
- **WHEN** user runs `fmp calendar dividends --from 2024-01-01 --to 2024-03-31`
- **THEN** the CLI passes from/to to `CalendarApi::dividends` and prints JSON

#### Scenario: IPO disclosure calendar
- **WHEN** user runs `fmp calendar ipos-disclosure`
- **THEN** the CLI calls `CalendarApi::ipos_disclosure` and prints JSON

#### Scenario: IPO prospectus calendar
- **WHEN** user runs `fmp calendar ipos-prospectus`
- **THEN** the CLI calls `CalendarApi::ipos_prospectus` and prints JSON

### Requirement: Analyst missing subcommands
The `analyst` CLI module SHALL expose `price-target-news`, `price-target-latest-news`, `grades-news`, `grades-latest-news`, and `grades-historical` subcommands.

#### Scenario: Price target news
- **WHEN** user runs `fmp analyst price-target-news --symbol AAPL`
- **THEN** the CLI calls `AnalystApi::price_target_news` and prints JSON

#### Scenario: Price target latest news
- **WHEN** user runs `fmp analyst price-target-latest-news`
- **THEN** the CLI calls `AnalystApi::price_target_latest_news` and prints JSON

#### Scenario: Grades news for a symbol
- **WHEN** user runs `fmp analyst grades-news --symbol AAPL`
- **THEN** the CLI calls `AnalystApi::grades_news` and prints JSON

#### Scenario: Latest grades news
- **WHEN** user runs `fmp analyst grades-latest-news`
- **THEN** the CLI calls `AnalystApi::grades_latest_news` and prints JSON

#### Scenario: Historical grades for a symbol
- **WHEN** user runs `fmp analyst grades-historical --symbol AAPL`
- **THEN** the CLI calls `AnalystApi::grades_historical` and prints JSON

### Requirement: News missing subcommands
The `news` CLI module SHALL expose `search-press-releases`, `search-crypto`, and `search-forex` subcommands.

#### Scenario: Search press releases
- **WHEN** user runs `fmp news search-press-releases --symbol AAPL`
- **THEN** the CLI calls `NewsApi::search_press_releases` and prints JSON

#### Scenario: Search crypto news
- **WHEN** user runs `fmp news search-crypto --symbol BTCUSD`
- **THEN** the CLI calls `NewsApi::search_crypto_news` and prints JSON

#### Scenario: Search forex news
- **WHEN** user runs `fmp news search-forex --symbol EURUSD`
- **THEN** the CLI calls `NewsApi::search_forex_news` and prints JSON

### Requirement: SEC filings missing subcommands
The `filings` CLI module SHALL expose `sec-profile`, `industry-classification-list`, `industry-classification-search`, and `industry-classification-all` subcommands.

#### Scenario: SEC profile for a symbol
- **WHEN** user runs `fmp filings sec-profile --symbol AAPL`
- **THEN** the CLI calls `SecFilingsApi::sec_profile` and prints JSON

#### Scenario: Industry classification list
- **WHEN** user runs `fmp filings industry-classification-list`
- **THEN** the CLI calls `SecFilingsApi::industry_classification_list` and prints JSON

#### Scenario: Industry classification search
- **WHEN** user runs `fmp filings industry-classification-search --query "technology"`
- **THEN** the CLI calls `SecFilingsApi::industry_classification_search` and prints JSON

#### Scenario: All industry classifications
- **WHEN** user runs `fmp filings industry-classification-all`
- **THEN** the CLI calls `SecFilingsApi::all_industry_classification` and prints JSON

### Requirement: Company missing subcommands
The `company` CLI module SHALL expose `notes`, `delisted`, and `executive-compensation-benchmark` subcommands.

#### Scenario: Company notes for a symbol
- **WHEN** user runs `fmp company notes --symbol AAPL`
- **THEN** the CLI calls `CompanyApi::company_notes` and prints JSON

#### Scenario: Delisted companies
- **WHEN** user runs `fmp company delisted`
- **THEN** the CLI calls `CompanyApi::delisted_companies` and prints JSON

#### Scenario: Executive compensation benchmark
- **WHEN** user runs `fmp company executive-compensation-benchmark`
- **THEN** the CLI calls `CompanyApi::executive_compensation_benchmark` and prints JSON

### Requirement: Crypto and Forex 1-hour chart
The `crypto` and `forex` CLI modules SHALL expose `chart-1hour` subcommands.

#### Scenario: Crypto 1-hour chart
- **WHEN** user runs `fmp crypto chart-1hour --symbol BTCUSD`
- **THEN** the CLI calls `CryptoApi::historical_chart_1hour` and prints JSON

#### Scenario: Forex 1-hour chart
- **WHEN** user runs `fmp forex chart-1hour --symbol EURUSD`
- **THEN** the CLI calls `ForexApi::historical_chart_1hour` and prints JSON
