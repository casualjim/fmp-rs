## ADDED Requirements

### Requirement: Form 13F CLI module
The CLI SHALL expose all `Form13FApi` functions via a top-level `form-13f` command with subcommands: `latest`, `extract`, `dates`, `holder-analytics`, `holder-performance`, `holder-industry`, `symbol-positions`, `industry-summary`.

#### Scenario: Latest 13F filings
- **WHEN** user runs `fmp form-13f latest`
- **THEN** the CLI calls `institutional_ownership_latest` and prints JSON to stdout

#### Scenario: Latest 13F filings with pagination
- **WHEN** user runs `fmp form-13f latest --page 2`
- **THEN** the CLI passes page=2 to `institutional_ownership_latest` and prints JSON

#### Scenario: Extract 13F filing by CIK and date
- **WHEN** user runs `fmp form-13f extract --cik 0001067983 --date 2024-03-31`
- **THEN** the CLI calls `institutional_ownership_extract` and prints JSON

#### Scenario: List 13F filing dates for a CIK
- **WHEN** user runs `fmp form-13f dates --cik 0001067983`
- **THEN** the CLI calls `institutional_ownership_dates` and prints JSON

#### Scenario: Holder analytics for a CIK
- **WHEN** user runs `fmp form-13f holder-analytics --cik 0001067983`
- **THEN** the CLI calls `institutional_ownership_extract_analytics_holder` and prints JSON

#### Scenario: Holder performance summary
- **WHEN** user runs `fmp form-13f holder-performance --cik 0001067983`
- **THEN** the CLI calls `institutional_ownership_holder_performance_summary` and prints JSON

#### Scenario: Holder industry breakdown
- **WHEN** user runs `fmp form-13f holder-industry --cik 0001067983`
- **THEN** the CLI calls `institutional_ownership_holder_industry_breakdown` and prints JSON

#### Scenario: Symbol positions summary
- **WHEN** user runs `fmp form-13f symbol-positions --symbol AAPL`
- **THEN** the CLI calls `institutional_ownership_symbol_positions_summary` and prints JSON

#### Scenario: Industry summary
- **WHEN** user runs `fmp form-13f industry-summary`
- **THEN** the CLI calls `institutional_ownership_industry_summary` and prints JSON

### Requirement: Form 13F registered in CLI dispatch
The `form-13f` command SHALL be registered in `config.rs` Commands enum and `cli/mod.rs#dispatch()`.

#### Scenario: Command appears in help
- **WHEN** user runs `fmp --help`
- **THEN** `form-13f` appears in the list of subcommands with a one-line description
