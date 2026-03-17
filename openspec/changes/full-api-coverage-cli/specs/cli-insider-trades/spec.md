## ADDED Requirements

### Requirement: Insider trades CLI module
The CLI SHALL expose all `InsiderTradesApi` functions via a top-level `insider-trades` command with subcommands: `latest`, `search`, `by-name`, `transaction-types`, `statistics`, `beneficial-ownership`.

#### Scenario: Latest insider trades
- **WHEN** user runs `fmp insider-trades latest`
- **THEN** the CLI calls `insider_trading_latest` and prints JSON to stdout

#### Scenario: Latest insider trades with page parameter
- **WHEN** user runs `fmp insider-trades latest --page 2`
- **THEN** the CLI passes `page=2` to the API and prints JSON to stdout

#### Scenario: Search insider trades by symbol
- **WHEN** user runs `fmp insider-trades search --symbol AAPL`
- **THEN** the CLI calls `insider_trading_search` with symbol=AAPL and prints JSON to stdout

#### Scenario: Search insider trades with pagination
- **WHEN** user runs `fmp insider-trades search --symbol AAPL --page 1`
- **THEN** the CLI passes both symbol and page to the API

#### Scenario: Insider trades by reporting name
- **WHEN** user runs `fmp insider-trades by-name --name "Elon Musk"`
- **THEN** the CLI calls `insider_trading_reporting_name` with the given name and prints JSON

#### Scenario: Insider transaction types for a symbol
- **WHEN** user runs `fmp insider-trades transaction-types --symbol TSLA`
- **THEN** the CLI calls `insider_trading_transaction_type` with symbol=TSLA and prints JSON

#### Scenario: Insider trading statistics for a symbol
- **WHEN** user runs `fmp insider-trades statistics --symbol TSLA`
- **THEN** the CLI calls `insider_trading_statistics` with symbol=TSLA and prints JSON

#### Scenario: Acquisition of beneficial ownership
- **WHEN** user runs `fmp insider-trades beneficial-ownership --symbol TSLA`
- **THEN** the CLI calls `acquisition_of_beneficial_ownership` and prints JSON

### Requirement: Insider trades registered in CLI dispatch
The `insider-trades` command SHALL be registered in `config.rs` Commands enum and in `cli/mod.rs#dispatch()`.

#### Scenario: Command appears in help
- **WHEN** user runs `fmp --help`
- **THEN** `insider-trades` appears in the list of subcommands with a one-line description
