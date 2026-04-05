## ADDED Requirements

### Requirement: Forex Pair List
Retrieve list of available forex pairs.

#### Scenario: Get all forex pairs
- **WHEN** user runs `fmp forex list`
- **THEN** returns list of all supported forex currency pairs

### Requirement: Forex Quotes
Get current exchange rate quotes.

#### Scenario: Full quote
- **WHEN** user runs `fmp forex quote --symbol EURUSD`
- **THEN** returns full quote with bid, ask, open, close, high, low prices

#### Scenario: Short quote
- **WHEN** user runs `fmp forex quote-short --symbol EURUSD`
- **THEN** returns simplified quote with symbol and current price

#### Scenario: Batch quotes
- **WHEN** user runs `fmp forex quote-batch --symbols EURUSD,GBPUSD,USDJPY`
- **THEN** returns quotes for multiple currency pairs in a single request

### Requirement: Historical Forex Data
Retrieve historical exchange rate data.

#### Scenario: Light historical data
- **WHEN** user runs `fmp forex eod-light --symbol EURUSD --from 2024-01-01 --to 2024-12-31`
- **THEN** returns lightweight EOD data with date, open, high, low, close

#### Scenario: Full historical data
- **WHEN** user runs `fmp forex eod-full --symbol EURUSD --from 2024-01-01`
- **THEN** returns complete historical data with additional metrics

### Requirement: Intraday Forex Data
Access intraday exchange rate movements.

#### Scenario: 1-minute intraday data
- **WHEN** user runs `fmp forex intraday-1min --symbol EURUSD --from 2024-01-15`
- **THEN** returns 1-minute candle data for the specified date

#### Scenario: 5-minute intraday data
- **WHEN** user runs `fmp forex intraday-5min --symbol EURUSD --from 2024-01-15`
- **THEN** returns 5-minute aggregated candle data

#### Scenario: Hourly intraday data
- **WHEN** user runs `fmp forex intraday-1hour --symbol EURUSD --from 2024-01-15`
- **THEN** returns hourly candle data for the specified date
