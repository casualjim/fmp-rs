## ADDED Requirements

### Requirement: Government trading CLI module
The CLI SHALL expose all `GovernmentTradingApi` functions via a top-level `government-trading` command with subcommands: `senate-latest`, `house-latest`, `senate-by-symbol`, `senate-by-name`, `house-by-symbol`, `house-by-name`.

#### Scenario: Latest Senate trades
- **WHEN** user runs `fmp government-trading senate-latest`
- **THEN** the CLI calls `senate_latest` and prints JSON to stdout

#### Scenario: Latest Senate trades with pagination
- **WHEN** user runs `fmp government-trading senate-latest --page 2`
- **THEN** the CLI passes page=2 to `senate_latest` and prints JSON

#### Scenario: Latest House trades
- **WHEN** user runs `fmp government-trading house-latest`
- **THEN** the CLI calls `house_latest` and prints JSON to stdout

#### Scenario: Senate trades for a symbol
- **WHEN** user runs `fmp government-trading senate-by-symbol --symbol AAPL`
- **THEN** the CLI calls `senate_trades` with symbol=AAPL and prints JSON

#### Scenario: Senate trades by representative name
- **WHEN** user runs `fmp government-trading senate-by-name --name "Nancy Pelosi"`
- **THEN** the CLI calls `senate_trades_by_name` with the given name and prints JSON

#### Scenario: House trades for a symbol
- **WHEN** user runs `fmp government-trading house-by-symbol --symbol AAPL`
- **THEN** the CLI calls `house_trades` with symbol=AAPL and prints JSON

#### Scenario: House trades by representative name
- **WHEN** user runs `fmp government-trading house-by-name --name "Nancy Pelosi"`
- **THEN** the CLI calls `house_trades_by_name` with the given name and prints JSON

### Requirement: Government trading registered in CLI dispatch
The `government-trading` command SHALL be registered in `config.rs` Commands enum and `cli/mod.rs#dispatch()`.

#### Scenario: Command appears in help
- **WHEN** user runs `fmp --help`
- **THEN** `government-trading` appears in the list of subcommands with a one-line description
