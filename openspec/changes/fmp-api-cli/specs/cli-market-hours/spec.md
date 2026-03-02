## ADDED Requirements

### Requirement: Exchange Market Hours
Retrieve trading hours for specific exchanges.

#### Scenario: Single exchange hours
- **WHEN** user runs `fmp hours exchange --exchange NYSE`
- **THEN** returns market hours including open time, close time, timezone, and current status

#### Scenario: Multiple exchange hours
- **WHEN** user runs `fmp hours exchange --exchange NASDAQ`
- **THEN** returns market hours for the specified exchange

### Requirement: All Exchange Hours
Retrieve trading hours for all exchanges.

#### Scenario: Get all exchange hours
- **WHEN** user runs `fmp hours all`
- **THEN** returns market hours for all available exchanges in the system

### Requirement: Exchange Holidays
Retrieve holiday schedules for exchanges.

#### Scenario: Holidays by exchange
- **WHEN** user runs `fmp hours holidays --exchange NYSE --year 2024`
- **THEN** returns list of holidays with dates and holiday names

#### Scenario: Current year holidays
- **WHEN** user runs `fmp hours holidays --exchange NASDAQ`
- **THEN** returns holidays for the current year
