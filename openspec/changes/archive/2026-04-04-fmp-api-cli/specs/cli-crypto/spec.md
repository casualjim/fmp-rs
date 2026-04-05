## ADDED Requirements

### Requirement: Cryptocurrency List
Retrieve list of available cryptocurrencies.

#### Scenario: Get all cryptocurrencies
- **WHEN** user runs `fmp crypto list`
- **THEN** returns list of all supported cryptocurrencies with symbol, name, and market data

### Requirement: Cryptocurrency Quotes
Get current price quotes for cryptocurrencies.

#### Scenario: Full quote
- **WHEN** user runs `fmp crypto quote --symbol BTCUSD`
- **THEN** returns full quote with price, market cap, 24h volume, 24h change percentage

#### Scenario: Short quote
- **WHEN** user runs `fmp crypto quote-short --symbol BTCUSD`
- **THEN** returns simplified quote with symbol, price, and volume

#### Scenario: Batch quotes
- **WHEN** user runs `fmp crypto quote-batch --symbols BTCUSD,ETHUSD,LTCUSD`
- **THEN** returns quotes for multiple cryptocurrencies in a single request

### Requirement: Historical Price Data
Retrieve historical cryptocurrency price data.

#### Scenario: Light historical data
- **WHEN** user runs `fmp crypto eod-light --symbol BTCUSD --from 2024-01-01 --to 2024-12-31`
- **THEN** returns lightweight EOD data with date, open, high, low, close, volume

#### Scenario: Full historical data
- **WHEN** user runs `fmp crypto eod-full --symbol BTCUSD --from 2024-01-01`
- **THEN** returns complete historical data with additional metrics

### Requirement: Intraday Price Data
Access intraday cryptocurrency price movements.

#### Scenario: 1-minute intraday data
- **WHEN** user runs `fmp crypto intraday-1min --symbol BTCUSD --from 2024-01-15`
- **THEN** returns 1-minute candle data for the specified date

#### Scenario: 5-minute intraday data
- **WHEN** user runs `fmp crypto intraday-5min --symbol BTCUSD --from 2024-01-15`
- **THEN** returns 5-minute aggregated candle data

#### Scenario: Hourly intraday data
- **WHEN** user runs `fmp crypto intraday-1hour --symbol BTCUSD --from 2024-01-15`
- **THEN** returns hourly candle data for the specified date
