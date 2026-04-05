## ADDED Requirements

### Requirement: Latest Insider Trades
Track recent insider trading activity.

#### Scenario: Get latest insider trades
- **WHEN** user runs `fmp insider latest --limit 50`
- **THEN** returns recent insider trades with symbol, insider name, transaction type, shares, and date

#### Scenario: Get latest for symbol
- **WHEN** user runs `fmp insider latest --symbol AAPL`
- **THEN** returns recent insider trades for the specified company

### Requirement: Search Insider Trades
Search historical insider trading data.

#### Scenario: Search by symbol
- **WHEN** user runs `fmp insider search --symbol AAPL --from 2024-01-01`
- **THEN** returns insider trades for the company within the date range

#### Scenario: Search by insider name
- **WHEN** user runs `fmp insider search-reporting-name --name "Tim Cook"`
- **THEN** returns trades by the specified insider across all companies

### Requirement: Insider Transaction Types
Analyze types of insider transactions.

#### Scenario: Get transaction types
- **WHEN** user runs `fmp insider transaction-types --symbol AAPL`
- **THEN** returns breakdown of transaction types (buy, sell, option exercise, etc.)

### Requirement: Insider Trading Statistics
Get aggregated insider trading statistics.

#### Scenario: Get insider statistics
- **WHEN** user runs `fmp insider statistics --symbol AAPL`
- **THEN** returns aggregated statistics including total buys, sells, and net activity

### Requirement: Beneficial Ownership Acquisitions
Track significant ownership changes.

#### Scenario: Get acquisition notices
- **WHEN** user runs `fmp insider acquisition --from 2024-01-01 --limit 20`
- **THEN** returns Form 4 acquisition of beneficial ownership filings
