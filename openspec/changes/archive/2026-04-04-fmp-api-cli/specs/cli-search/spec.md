## ADDED Requirements

### Requirement: Symbol Search
Search for stocks by ticker symbol.

#### Scenario: Search by partial symbol
- **WHEN** user runs `fmp search symbol --query AAPL`
- **THEN** returns matching stocks with symbol, name, currency, exchange, and stock exchange

#### Scenario: Search with limit
- **WHEN** user runs `fmp search symbol --query AA --limit 10`
- **THEN** returns up to 10 matching symbols

### Requirement: Name Search
Search for stocks by company name.

#### Scenario: Search by company name
- **WHEN** user runs `fmp search name --query Apple`
- **THEN** returns companies with matching names including symbol and full name

#### Scenario: Partial name match
- **WHEN** user runs `fmp search name --query Micro --limit 20`
- **THEN** returns companies with names containing the search term

### Requirement: CIK Search
Search for companies by SEC CIK number.

#### Scenario: Search by CIK
- **WHEN** user runs `fmp search cik --query 0000320193`
- **THEN** returns company information matching the CIK number

### Requirement: CUSIP Search
Search for securities by CUSIP identifier.

#### Scenario: Search by CUSIP
- **WHEN** user runs `fmp search cusip --query 037833100`
- **THEN** returns securities matching the CUSIP identifier

### Requirement: ISIN Search
Search for securities by ISIN identifier.

#### Scenario: Search by ISIN
- **WHEN** user runs `fmp search isin --query US0378331005`
- **THEN** returns securities matching the ISIN identifier

### Requirement: Stock Screener
Screen stocks based on multiple criteria.

#### Scenario: Screen by market cap
- **WHEN** user runs `fmp search screener --market-cap-more-than 1000000000`
- **THEN** returns stocks with market cap greater than $1B

#### Scenario: Screen by sector
- **WHEN** user runs `fmp search screener --sector Technology --limit 50`
- **THEN** returns technology sector stocks

#### Scenario: Complex screener query
- **WHEN** user runs `fmp search screener --market-cap-more-than 1000000000 --dividend-more-than 2 --beta-less-than 1.5`
- **THEN** returns stocks matching all specified criteria
