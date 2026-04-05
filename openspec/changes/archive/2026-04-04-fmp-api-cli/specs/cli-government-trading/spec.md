## ADDED Requirements

### Requirement: Senate Trading
Track US Senate member stock trades.

#### Scenario: Get latest Senate trades
- **WHEN** user runs `fmp gov senate-latest --limit 50`
- **THEN** returns recent Senate trading disclosures with member name, asset, and transaction details

#### Scenario: Get Senate trades by symbol
- **WHEN** user runs `fmp gov senate-trades --symbol AAPL`
- **THEN** returns Senate trades involving the specified stock

#### Scenario: Get Senate trades by member name
- **WHEN** user runs `fmp gov senate-by-name --name "Pelosi"`
- **THEN** returns trades by the specified Senate member

### Requirement: House Trading
Track US House member stock trades.

#### Scenario: Get latest House trades
- **WHEN** user runs `fmp gov house-latest --limit 50`
- **THEN** returns recent House trading disclosures with member name, asset, and transaction details

#### Scenario: Get House trades by symbol
- **WHEN** user runs `fmp gov house-trades --symbol AAPL`
- **THEN** returns House trades involving the specified stock

#### Scenario: Get House trades by member name
- **WHEN** user runs `fmp gov house-by-name --name "Nancy"`
- **THEN** returns trades by the specified House member
