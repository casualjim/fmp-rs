## ADDED Requirements

### Requirement: Sector Performance
Track sector-level market performance.

#### Scenario: Sector snapshot
- **WHEN** user runs `fmp performance sector-snapshot`
- **THEN** returns current performance metrics for all market sectors with percentage changes

#### Scenario: Historical sector performance
- **WHEN** user runs `fmp performance sector-history --date 2024-06-15`
- **THEN** returns sector performance data for the specified date

### Requirement: Industry Performance
Track industry-level market performance.

#### Scenario: Industry snapshot
- **WHEN** user runs `fmp performance industry-snapshot`
- **THEN** returns current performance metrics for all industries

#### Scenario: Historical industry performance
- **WHEN** user runs `fmp performance industry-history --date 2024-06-15`
- **THEN** returns industry performance data for the specified date

### Requirement: PE Ratios by Sector/Industry
Analyze valuation metrics across sectors and industries.

#### Scenario: Sector PE snapshot
- **WHEN** user runs `fmp performance sector-pe`
- **THEN** returns current PE ratios for all market sectors

#### Scenario: Historical sector PE
- **WHEN** user runs `fmp performance sector-pe-history --date 2024-06-15`
- **THEN** returns sector PE ratios for the specified date

### Requirement: Market Movers
Track stocks with significant price movements.

#### Scenario: Biggest gainers
- **WHEN** user runs `fmp performance gainers`
- **THEN** returns list of stocks with largest percentage gains

#### Scenario: Biggest losers
- **WHEN** user runs `fmp performance losers`
- **THEN** returns list of stocks with largest percentage losses

#### Scenario: Most active stocks
- **WHEN** user runs `fmp performance actives`
- **THEN** returns list of stocks with highest trading volume
