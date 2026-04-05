## ADDED Requirements

### Requirement: Stock List
Retrieve complete list of available stocks.

#### Scenario: Get all stocks
- **WHEN** user runs `fmp directory stocks`
- **THEN** returns list of all available stock symbols with name, price, exchange

### Requirement: Financial Statement Symbols
Get symbols with available financial statements.

#### Scenario: Get symbols with statements
- **WHEN** user runs `fmp directory statement-symbols`
- **THEN** returns list of symbols that have financial statement data available

### Requirement: CIK List
Retrieve SEC CIK mappings.

#### Scenario: Get CIK list
- **WHEN** user runs `fmp directory cik --ticker AAPL`
- **THEN** returns CIK number and company name for the ticker

### Requirement: Symbol Changes
Track ticker symbol changes.

#### Scenario: Get symbol changes
- **WHEN** user runs `fmp directory symbol-changes --from 2024-01-01`
- **THEN** returns list of symbol changes with old symbol, new symbol, and date

### Requirement: ETF List
Retrieve available ETFs.

#### Scenario: Get all ETFs
- **WHEN** user runs `fmp directory etfs`
- **THEN** returns list of all available ETF symbols with names

### Requirement: Actively Trading Stocks
Get currently trading stocks.

#### Scenario: Get active stocks
- **WHEN** user runs `fmp directory active`
- **THEN** returns list of stocks that are actively trading

### Requirement: Available Earnings Transcripts
List symbols with earnings transcripts.

#### Scenario: Get transcript symbols
- **WHEN** user runs `fmp directory transcripts`
- **THEN** returns list of symbols that have earnings call transcripts available

### Requirement: Market Metadata
Retrieve available exchanges, sectors, industries, and countries.

#### Scenario: Get exchanges
- **WHEN** user runs `fmp directory exchanges`
- **THEN** returns list of all available exchanges

#### Scenario: Get sectors
- **WHEN** user runs `fmp directory sectors`
- **THEN** returns list of all market sectors

#### Scenario: Get industries
- **WHEN** user runs `fmp directory industries`
- **THEN** returns list of all industries

#### Scenario: Get countries
- **WHEN** user runs `fmp directory countries`
- **THEN** returns list of all countries with traded securities
