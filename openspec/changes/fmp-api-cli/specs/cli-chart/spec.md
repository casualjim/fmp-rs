## ADDED Requirements

### Requirement: Historical EOD Price Data
Retrieve end-of-day historical price data with various adjustment options.

#### Scenario: Light chart data retrieval
- **WHEN** user runs `fmp chart eod-light --symbol AAPL --from 2024-01-01 --to 2024-12-31`
- **THEN** returns lightweight EOD price data with date, open, high, low, close, volume

#### Scenario: Full chart data retrieval
- **WHEN** user runs `fmp chart eod-full --symbol AAPL --from 2024-01-01 --to 2024-12-31`
- **THEN** returns complete EOD price data including adjClose, unadjustedVolume, change, changePercent

#### Scenario: Non-split adjusted data
- **WHEN** user runs `fmp chart eod-non-split --symbol AAPL --from 2024-01-01`
- **THEN** returns price data not adjusted for stock splits

### Requirement: Intraday Chart Intervals
Retrieve intraday price data at various time intervals.

#### Scenario: 1-minute interval data
- **WHEN** user runs `fmp chart intraday --symbol AAPL --interval 1min --from 2024-01-15`
- **THEN** returns 1-minute candle data with timestamp, open, high, low, close, volume

#### Scenario: 1-hour interval data
- **WHEN** user runs `fmp chart intraday --symbol AAPL --interval 1hour --from 2024-01-15`
- **THEN** returns hourly candle data for the specified date

#### Scenario: 4-hour interval data
- **WHEN** user runs `fmp chart intraday --symbol AAPL --interval 4hour`
- **THEN** returns 4-hour aggregated candle data

### Requirement: Simple Moving Average (SMA)
Calculate and retrieve SMA technical indicator data.

#### Scenario: SMA with default period
- **WHEN** user runs `fmp chart sma --symbol AAPL --period 20`
- **THEN** returns SMA values with date, SMA value, and period

#### Scenario: SMA with date range
- **WHEN** user runs `fmp chart sma --symbol AAPL --period 50 --from 2024-01-01 --to 2024-06-30`
- **THEN** returns SMA values for the specified date range

### Requirement: Exponential Moving Average (EMA)
Calculate and retrieve EMA technical indicator data.

#### Scenario: EMA calculation
- **WHEN** user runs `fmp chart ema --symbol AAPL --period 20`
- **THEN** returns EMA values for the specified period

#### Scenario: Multiple EMA periods
- **WHEN** user runs `fmp chart ema --symbol AAPL --period 12` and `fmp chart ema --symbol AAPL --period 26`
- **THEN** returns respective EMA values for MACD analysis

### Requirement: Relative Strength Index (RSI)
Calculate and retrieve RSI technical indicator.

#### Scenario: Standard RSI period
- **WHEN** user runs `fmp chart rsi --symbol AAPL --period 14`
- **THEN** returns RSI values between 0-100 for overbought/oversold analysis

#### Scenario: Custom RSI period
- **WHEN** user runs `fmp chart rsi --symbol AAPL --period 7`
- **THEN** returns RSI with more responsive short-term calculation

### Requirement: Additional Technical Indicators
Access various other technical indicators.

#### Scenario: Williams %R indicator
- **WHEN** user runs `fmp chart williams --symbol AAPL --period 14`
- **THEN** returns Williams %R oscillator values

#### Scenario: ADX trend strength
- **WHEN** user runs `fmp chart adx --symbol AAPL --period 14`
- **THEN** returns ADX values measuring trend strength

#### Scenario: Standard deviation
- **WHEN** user runs `fmp chart stddev --symbol AAPL --period 20`
- **THEN** returns standard deviation values for volatility analysis
