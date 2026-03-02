## ADDED Requirements

### Requirement: Get real-time quotes
The CLI SHALL fetch real-time stock quotes for one or more symbols.

#### Scenario: Single symbol quote
- **WHEN** user runs `fmp quotes get AAPL`
- **THEN** system displays full quote with price, change, volume, market cap, PE ratio

#### Scenario: Multiple symbols quote
- **WHEN** user runs `fmp quotes get AAPL MSFT GOOGL`
- **THEN** system displays quotes for all symbols in JSON format

#### Scenario: Quote output includes key fields
- **WHEN** quote data is displayed
- **THEN** output includes symbol, price, change, change percent, volume, avg volume, market cap

### Requirement: Get short-form quotes
The CLI SHALL fetch lightweight quote data for performance.

#### Scenario: Short quote
- **WHEN** user runs `fmp quotes short AAPL MSFT`
- **THEN** system displays symbol, price, and volume only

### Requirement: Get aftermarket data
The CLI SHALL fetch pre-market and after-hours trading data.

#### Scenario: Aftermarket trades
- **WHEN** user runs `fmp quotes aftermarket-trade AAPL`
- **THEN** system displays after-hours trade data with time, price, volume

#### Scenario: Aftermarket quotes
- **WHEN** user runs `fmp quotes aftermarket-quote AAPL`
- **THEN** system displays after-hours quote with bid/ask spread

### Requirement: Get price changes
The CLI SHALL fetch stock price change statistics.

#### Scenario: Price change data
- **WHEN** user runs `fmp quotes price-change AAPL`
- **THEN** system displays price changes over multiple periods (1D, 5D, 1M, 3M, 6M, 1Y, YTD)

### Requirement: Batch quote operations
The CLI SHALL support batch operations for efficient data retrieval.

#### Scenario: Batch quotes
- **WHEN** user runs `fmp quotes batch AAPL,MSFT,GOOGL`
- **THEN** system fetches full quotes for all symbols in single request

#### Scenario: Batch short quotes
- **WHEN** user runs `fmp quotes batch-short AAPL,MSFT,GOOGL`
- **THEN** system fetches short quotes for all symbols

#### Scenario: Batch aftermarket data
- **WHEN** user runs `fmp quotes batch-aftermarket AAPL,MSFT`
- **THEN** system fetches aftermarket quotes for all symbols

### Requirement: Exchange-specific quotes
The CLI SHALL fetch quotes filtered by exchange.

#### Scenario: Exchange quotes
- **WHEN** user runs `fmp quotes exchange NASDAQ`
- **THEN** system displays quotes for all NASDAQ-listed stocks

### Requirement: Asset class quotes
The CLI SHALL fetch quotes for different asset classes.

#### Scenario: Mutual fund quotes
- **WHEN** user runs `fmp quotes mutual-fund VFIAX`
- **THEN** system displays mutual fund NAV and performance

#### Scenario: ETF quotes
- **WHEN** user runs `fmp quotes etf SPY,QQQ`
- **THEN** system displays ETF prices and volumes

#### Scenario: Commodity quotes
- **WHEN** user runs `fmp quotes commodity GCUSD,SIUSD`
- **THEN** system displays commodity spot prices

#### Scenario: Crypto quotes
- **WHEN** user runs `fmp quotes crypto BTCUSD,ETHUSD`
- **THEN** system displays cryptocurrency prices

#### Scenario: Forex quotes
- **WHEN** user runs `fmp quotes forex EURUSD,GBPUSD`
- **THEN** system displays forex rates

#### Scenario: Index quotes
- **WHEN** user runs `fmp quotes index SPX,DJI`
- **THEN** system displays index values
