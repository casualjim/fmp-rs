## ADDED Requirements

### Requirement: ETF Holdings
Retrieve ETF constituent holdings.

#### Scenario: Get ETF holdings
- **WHEN** user runs `fmp fund holdings --symbol SPY`
- **THEN** returns list of ETF holdings with symbol, name, shares, and weight

### Requirement: ETF Information
Get detailed ETF metadata.

#### Scenario: Get ETF info
- **WHEN** user runs `fmp fund info --symbol SPY`
- **THEN** returns ETF details including name, description, expense ratio, AUM, and category

### Requirement: ETF Country Weightings
Analyze geographic allocation.

#### Scenario: Get country weightings
- **WHEN** user runs `fmp fund country --symbol SPY`
- **THEN** returns country-level allocation breakdown

### Requirement: ETF Asset Exposure
Analyze asset class exposure.

#### Scenario: Get asset exposure
- **WHEN** user runs `fmp fund exposure --symbol SPY`
- **THEN** returns asset class exposure breakdown

### Requirement: ETF Sector Weightings
Analyze sector allocation.

#### Scenario: Get sector weightings
- **WHEN** user runs `fmp fund sector --symbol SPY`
- **THEN** returns sector-level allocation breakdown

### Requirement: Fund Disclosure Holders
Track mutual fund and ETF holders.

#### Scenario: Get disclosure holders latest
- **WHEN** user runs `fmp fund holders-latest --symbol AAPL`
- **THEN** returns funds that hold the specified stock

#### Scenario: Search disclosure holders
- **WHEN** user runs `fmp fund holders-search --symbol AAPL`
- **THEN** returns searchable fund holder information

### Requirement: Fund Disclosure Data
Access detailed fund disclosure information.

#### Scenario: Get disclosure dates
- **WHEN** user runs `fmp fund disclosure-dates --cik 0001067983`
- **THEN** returns available disclosure filing dates

#### Scenario: Get fund disclosure
- **WHEN** user runs `fmp fund disclosure --cik 0001067983 --date 2024-03-31`
- **THEN** returns detailed fund disclosure data
