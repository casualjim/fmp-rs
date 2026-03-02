## ADDED Requirements

### Requirement: Discounted Cash Flow Valuation
Access DCF valuation models.

#### Scenario: Get DCF valuation
- **WHEN** user runs `fmp dcf valuation --symbol AAPL`
- **THEN** returns DCF intrinsic value, current price, and valuation date

#### Scenario: Get levered DCF
- **WHEN** user runs `fmp dcf levered --symbol AAPL`
- **THEN** returns levered DCF valuation accounting for debt

### Requirement: Custom DCF Calculation
Perform custom DCF analysis.

#### Scenario: Custom unlevered DCF
- **WHEN** user runs `fmp dcf custom --symbol AAPL --growth-rate 0.05 --discount-rate 0.10 --terminal-growth 0.03`
- **THEN** returns custom DCF valuation with user-specified parameters

#### Scenario: Custom levered DCF
- **WHEN** user runs `fmp dcf custom-levered --symbol AAPL --growth-rate 0.05 --discount-rate 0.10`
- **THEN** returns custom levered DCF valuation

#### Scenario: Custom DCF with revenue projections
- **WHEN** user runs `fmp dcf custom --symbol AAPL --revenue-growth 0.08 --ebitda-margin 0.25`
- **THEN** returns DCF valuation based on custom revenue and margin assumptions
