## ADDED Requirements

### Requirement: Index List
Retrieve list of available market indexes.

#### Scenario: Get all indexes
- **WHEN** user runs `fmp index list`
- **THEN** returns list of all supported market indexes with symbol and name

### Requirement: Index Quotes
Get current index values.

#### Scenario: Full quote
- **WHEN** user runs `fmp index quote --symbol ^GSPC`
- **THEN** returns full quote with price, changes, and percentage moves

#### Scenario: Short quote
- **WHEN** user runs `fmp index quote-short --symbol ^GSPC`
- **THEN** returns simplified quote with symbol and current price

#### Scenario: Batch quotes
- **WHEN** user runs `fmp index quote-batch --symbols ^GSPC,^DJI,^IXIC`
- **THEN** returns quotes for multiple indexes in a single request

### Requirement: Historical Index Data
Retrieve historical index values.

#### Scenario: Light historical data
- **WHEN** user runs `fmp index eod-light --symbol ^GSPC --from 2024-01-01 --to 2024-12-31`
- **THEN** returns lightweight EOD data with date, open, high, low, close, volume

#### Scenario: Full historical data
- **WHEN** user runs `fmp index eod-full --symbol ^GSPC --from 2024-01-01`
- **THEN** returns complete historical data with additional metrics

### Requirement: Intraday Index Data
Access intraday index movements.

#### Scenario: 1-minute intraday data
- **WHEN** user runs `fmp index intraday-1min --symbol ^GSPC --from 2024-01-15`
- **THEN** returns 1-minute candle data for the specified date

#### Scenario: 5-minute intraday data
- **WHEN** user runs `fmp index intraday-5min --symbol ^GSPC --from 2024-01-15`
- **THEN** returns 5-minute aggregated candle data

#### Scenario: Hourly intraday data
- **WHEN** user runs `fmp index intraday-1hour --symbol ^GSPC --from 2024-01-15`
- **THEN** returns hourly candle data for the specified date
