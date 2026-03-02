## ADDED Requirements

### Requirement: COT Report
Retrieve Commitment of Traders reports.

#### Scenario: Get COT report
- **WHEN** user runs `fmp cot report --from 2024-01-01 --to 2024-12-31`
- **THEN** returns COT data with date, open interest, and positions by trader category

#### Scenario: Get COT for specific commodity
- **WHEN** user runs `fmp cot report --symbol GOLD --from 2024-01-01`
- **THEN** returns COT data for the specified commodity

### Requirement: COT Analysis
Access analyzed COT positioning data.

#### Scenario: Get COT analysis
- **WHEN** user runs `fmp cot analysis --from 2024-01-01 --to 2024-12-31`
- **THEN** returns analyzed COT data with net positions and historical comparisons

#### Scenario: Get COT analysis for symbol
- **WHEN** user runs `fmp cot analysis --symbol CRUDEOIL --limit 20`
- **THEN** returns COT analysis for crude oil futures
