## ADDED Requirements

### Requirement: Treasury Rates
Retrieve US Treasury yield data.

#### Scenario: Get treasury rates
- **WHEN** user runs `fmp economics treasury --from 2024-01-01 --to 2024-12-31`
- **THEN** returns daily treasury rates for 1M, 3M, 6M, 1Y, 2Y, 5Y, 10Y, 20Y, 30Y maturities

#### Scenario: Latest treasury rates
- **WHEN** user runs `fmp economics treasury --limit 1`
- **THEN** returns the most recent treasury rate data

### Requirement: Economic Indicators
Access macroeconomic indicator data.

#### Scenario: Get GDP data
- **WHEN** user runs `fmp economics indicator --name GDP`
- **THEN** returns GDP historical data with dates and values

#### Scenario: Get inflation data
- **WHEN** user runs `fmp economics indicator --name CPI`
- **THEN** returns Consumer Price Index historical data

#### Scenario: Get unemployment data
- **WHEN** user runs `fmp economics indicator --name unemploymentRate`
- **THEN** returns unemployment rate historical data

### Requirement: Economic Calendar
Track upcoming economic events.

#### Scenario: Get economic calendar
- **WHEN** user runs `fmp economics calendar --from 2024-01-01 --to 2024-01-31`
- **THEN** returns economic events with date, event name, impact level, actual, estimate

#### Scenario: Filter by impact
- **WHEN** user runs `fmp economics calendar --from 2024-01-01 --impact high`
- **THEN** returns only high-impact economic events

### Requirement: Market Risk Premium
Access market risk premium data.

#### Scenario: Get risk premiums
- **WHEN** user runs `fmp economics risk-premium`
- **THEN** returns market risk premium data by country
