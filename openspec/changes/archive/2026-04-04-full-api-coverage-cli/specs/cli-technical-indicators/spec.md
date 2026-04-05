## ADDED Requirements

### Requirement: Technical indicators dedicated CLI module
The CLI SHALL expose all `TechnicalIndicatorsApi` functions via a top-level `technical-indicators` command with subcommands: `sma`, `ema`, `wma`, `dema`, `tema`, `rsi`, `std-dev`, `williams`, `adx`.

#### Scenario: SMA for a symbol
- **WHEN** user runs `fmp technical-indicators sma --symbol AAPL --period 20 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::sma` and prints JSON to stdout

#### Scenario: EMA for a symbol
- **WHEN** user runs `fmp technical-indicators ema --symbol AAPL --period 20 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::ema` and prints JSON

#### Scenario: WMA for a symbol
- **WHEN** user runs `fmp technical-indicators wma --symbol AAPL --period 20 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::wma` and prints JSON

#### Scenario: DEMA for a symbol
- **WHEN** user runs `fmp technical-indicators dema --symbol AAPL --period 20 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::dema` and prints JSON

#### Scenario: TEMA for a symbol
- **WHEN** user runs `fmp technical-indicators tema --symbol AAPL --period 20 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::tema` and prints JSON

#### Scenario: RSI for a symbol
- **WHEN** user runs `fmp technical-indicators rsi --symbol AAPL --period 14 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::rsi` and prints JSON

#### Scenario: Standard deviation for a symbol
- **WHEN** user runs `fmp technical-indicators std-dev --symbol AAPL --period 20 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::standard_deviation` and prints JSON

#### Scenario: Williams %R for a symbol
- **WHEN** user runs `fmp technical-indicators williams --symbol AAPL --period 14 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::williams` and prints JSON

#### Scenario: ADX for a symbol
- **WHEN** user runs `fmp technical-indicators adx --symbol AAPL --period 14 --interval 1day`
- **THEN** the CLI calls `TechnicalIndicatorsApi::adx` and prints JSON

### Requirement: Technical indicators registered in CLI dispatch
The `technical-indicators` command SHALL be registered in `config.rs` Commands enum and `cli/mod.rs#dispatch()`. The existing technical indicator subcommands in `chart` SHALL remain untouched.

#### Scenario: Command appears in help
- **WHEN** user runs `fmp --help`
- **THEN** `technical-indicators` appears in the list of subcommands with a one-line description
