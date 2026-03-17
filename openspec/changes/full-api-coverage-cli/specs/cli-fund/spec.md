## ADDED Requirements

### Requirement: Fund CLI module
The CLI SHALL expose all `FundApi` functions via a top-level `fund` command with subcommands: `etf-holdings`, `etf-info`, `etf-countries`, `etf-assets`, `etf-sectors`, `disclosure-holders-latest`, `disclosure-holders-search`, `disclosure-dates`, `disclosure`.

#### Scenario: ETF holdings
- **WHEN** user runs `fmp fund etf-holdings --symbol SPY`
- **THEN** the CLI calls `etf_holdings` with symbol=SPY and prints JSON to stdout

#### Scenario: ETF info
- **WHEN** user runs `fmp fund etf-info --symbol SPY`
- **THEN** the CLI calls `etf_info` with symbol=SPY and prints JSON

#### Scenario: ETF country weightings
- **WHEN** user runs `fmp fund etf-countries --symbol EFA`
- **THEN** the CLI calls `etf_country_weightings` with symbol=EFA and prints JSON

#### Scenario: ETF asset exposure
- **WHEN** user runs `fmp fund etf-assets --symbol SPY`
- **THEN** the CLI calls `etf_asset_exposure` with symbol=SPY and prints JSON

#### Scenario: ETF sector weightings
- **WHEN** user runs `fmp fund etf-sectors --symbol SPY`
- **THEN** the CLI calls `etf_sector_weightings` with symbol=SPY and prints JSON

#### Scenario: Latest disclosure holders for a fund
- **WHEN** user runs `fmp fund disclosure-holders-latest --symbol ARKK`
- **THEN** the CLI calls `funds_disclosure_holders_latest` with symbol=ARKK and prints JSON

#### Scenario: Search fund disclosure holders
- **WHEN** user runs `fmp fund disclosure-holders-search --symbol ARKK`
- **THEN** the CLI calls `funds_disclosure_holders_search` and prints JSON

#### Scenario: Fund disclosure dates
- **WHEN** user runs `fmp fund disclosure-dates --symbol ARKK`
- **THEN** the CLI calls `funds_disclosure_dates` and prints JSON

#### Scenario: Fund disclosure for a date
- **WHEN** user runs `fmp fund disclosure --symbol ARKK --date 2024-03-31`
- **THEN** the CLI calls `funds_disclosure` and prints JSON

### Requirement: Fund registered in CLI dispatch
The `fund` command SHALL be registered in `config.rs` Commands enum and `cli/mod.rs#dispatch()`.

#### Scenario: Command appears in help
- **WHEN** user runs `fmp --help`
- **THEN** `fund` appears in the list of subcommands with a one-line description
