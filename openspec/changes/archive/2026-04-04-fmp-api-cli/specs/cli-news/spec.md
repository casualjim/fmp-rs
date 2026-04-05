## ADDED Requirements

### Requirement: FMP Articles
Access FMP-curated financial articles.

#### Scenario: Get FMP articles
- **WHEN** user runs `fmp news fmp-articles --limit 10`
- **THEN** returns latest FMP articles with title, content, date, and tickers mentioned

### Requirement: General Market News
Retrieve general financial news.

#### Scenario: Get general news
- **WHEN** user runs `fmp news general --limit 20`
- **THEN** returns latest general market news articles

### Requirement: Press Releases
Access company press releases.

#### Scenario: Get press releases
- **WHEN** user runs `fmp news press-releases --limit 10`
- **THEN** returns latest press releases from companies

#### Scenario: Search press releases
- **WHEN** user runs `fmp news press-releases-search --tickers AAPL --from 2024-01-01 --limit 10`
- **THEN** returns press releases for specific tickers within date range

### Requirement: Stock News
Retrieve stock-specific news.

#### Scenario: Get stock news
- **WHEN** user runs `fmp news stock --limit 20`
- **THEN** returns latest stock market news

#### Scenario: Search stock news by ticker
- **WHEN** user runs `fmp news stock-search --tickers AAPL,MSFT --limit 10`
- **THEN** returns news articles mentioning the specified tickers

### Requirement: Crypto News
Access cryptocurrency news.

#### Scenario: Get crypto news
- **WHEN** user runs `fmp news crypto --limit 10`
- **THEN** returns latest cryptocurrency news articles

#### Scenario: Search crypto news
- **WHEN** user runs `fmp news crypto-search --tickers BTCUSD --limit 10`
- **THEN** returns news for specific cryptocurrencies

### Requirement: Forex News
Retrieve forex market news.

#### Scenario: Get forex news
- **WHEN** user runs `fmp news forex --limit 10`
- **THEN** returns latest foreign exchange market news

#### Scenario: Search forex news
- **WHEN** user runs `fmp news forex-search --tickers EURUSD --limit 10`
- **THEN** returns news for specific currency pairs
