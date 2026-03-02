## ADDED Requirements

### Requirement: Institutional Ownership Latest
Track recent institutional holdings.

#### Scenario: Get latest 13F filings
- **WHEN** user runs `fmp 13f latest --page 0`
- **THEN** returns recent institutional 13F filings with holder, positions, and filing date

### Requirement: 13F Filing Extract
Extract detailed 13F position data.

#### Scenario: Extract filing by date
- **WHEN** user runs `fmp 13f extract --cik 0001067983 --date 2024-03-31`
- **THEN** returns all positions from the specified 13F filing

### Requirement: Filing Dates
Get available 13F filing dates.

#### Scenario: Get filing dates
- **WHEN** user runs `fmp 13f dates --cik 0001067983`
- **THEN** returns all available 13F filing dates for the institution

### Requirement: Holder Analytics
Analyze institutional holder performance.

#### Scenario: Get holder performance summary
- **WHEN** user runs `fmp 13f holder-performance --cik 0001067983`
- **THEN** returns performance metrics for the institutional holder

#### Scenario: Get holder industry breakdown
- **WHEN** user runs `fmp 13f holder-industry --cik 0001067983`
- **THEN** returns industry allocation breakdown of holdings

### Requirement: Position Analysis
Analyze specific positions across institutions.

#### Scenario: Get symbol positions summary
- **WHEN** user runs `fmp 13f positions-summary --symbol AAPL`
- **THEN** returns summary of all institutional positions in the stock

### Requirement: Industry Summary
Get industry-level institutional ownership.

#### Scenario: Get industry summary
- **WHEN** user runs `fmp 13f industry-summary --industry Technology`
- **THEN** returns institutional ownership summary for the industry
