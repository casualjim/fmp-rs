## ADDED Requirements

### Requirement: Commodity List
Retrieve list of available commodities.

#### Scenario: Get all commodities
- **WHEN** user runs `fmp commodity list`
- **THEN** returns list of all supported commodities with symbol, name, exchange, and current price

#### Scenario: Filter by exchange
- **WHEN** user runs `fmp commodity list --exchange COMEX`
- **THEN** returns commodities traded on the specified exchange
