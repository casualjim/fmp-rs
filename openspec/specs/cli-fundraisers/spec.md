## ADDED Requirements

### Requirement: Fundraisers CLI module
The CLI SHALL expose all `FundraisersApi` functions via a top-level `fundraisers` command with subcommands: `crowdfunding-latest`, `crowdfunding-search`, `crowdfunding-by-cik`, `equity-latest`, `equity-search`, `equity-by-cik`.

#### Scenario: Latest crowdfunding offerings
- **WHEN** user runs `fmp fundraisers crowdfunding-latest`
- **THEN** the CLI calls `crowdfunding_offerings_latest` and prints JSON to stdout

#### Scenario: Latest crowdfunding offerings with pagination
- **WHEN** user runs `fmp fundraisers crowdfunding-latest --page 2`
- **THEN** the CLI passes page=2 to `crowdfunding_offerings_latest` and prints JSON

#### Scenario: Search crowdfunding offerings by name
- **WHEN** user runs `fmp fundraisers crowdfunding-search --name "Acme"`
- **THEN** the CLI calls `crowdfunding_offerings_search` with the given name and prints JSON

#### Scenario: Crowdfunding offerings by CIK
- **WHEN** user runs `fmp fundraisers crowdfunding-by-cik --cik 0001234567`
- **THEN** the CLI calls `crowdfunding_offerings` with the given CIK and prints JSON

#### Scenario: Latest equity fundraising offerings
- **WHEN** user runs `fmp fundraisers equity-latest`
- **THEN** the CLI calls `fundraising_latest` and prints JSON to stdout

#### Scenario: Latest equity fundraising with pagination
- **WHEN** user runs `fmp fundraisers equity-latest --page 2`
- **THEN** the CLI passes page=2 to `fundraising_latest` and prints JSON

#### Scenario: Search equity fundraising by name
- **WHEN** user runs `fmp fundraisers equity-search --name "Acme"`
- **THEN** the CLI calls `fundraising_search` with the given name and prints JSON

#### Scenario: Equity fundraising by CIK
- **WHEN** user runs `fmp fundraisers equity-by-cik --cik 0001234567`
- **THEN** the CLI calls `fundraising` with the given CIK and prints JSON

### Requirement: Fundraisers registered in CLI dispatch
The `fundraisers` command SHALL be registered in `config.rs` Commands enum and `cli/mod.rs#dispatch()`.

#### Scenario: Command appears in help
- **WHEN** user runs `fmp --help`
- **THEN** `fundraisers` appears in the list of subcommands with a one-line description
