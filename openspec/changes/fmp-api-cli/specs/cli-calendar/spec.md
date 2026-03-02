## ADDED Requirements

### Requirement: Dividends Calendar
Track dividend payments.

#### Scenario: Get dividends for symbol
- **WHEN** user runs `fmp calendar dividends --symbol AAPL --limit 10`
- **THEN** returns dividend history with date, amount, and yield

#### Scenario: Get dividend calendar
- **WHEN** user runs `fmp calendar dividends-calendar --from 2024-01-01 --to 2024-03-31`
- **THEN** returns all dividend events in the date range

### Requirement: Earnings Calendar
Track earnings announcements.

#### Scenario: Get earnings for symbol
- **WHEN** user runs `fmp calendar earnings --symbol AAPL --limit 10`
- **THEN** returns earnings history with date, EPS actual, EPS estimate

#### Scenario: Get earnings calendar
- **WHEN** user runs `fmp calendar earnings-calendar --from 2024-01-15 --to 2024-01-31`
- **THEN** returns all earnings announcements in the date range

### Requirement: IPO Calendar
Track initial public offerings.

#### Scenario: Get IPO calendar
- **WHEN** user runs `fmp calendar ipos --from 2024-01-01 --to 2024-03-31`
- **THEN** returns IPO events with company name, symbol, price range, date

#### Scenario: Get IPO disclosures
- **WHEN** user runs `fmp calendar ipos-disclosure --from 2024-01-01`
- **THEN** returns IPO disclosure documents

#### Scenario: Get IPO prospectuses
- **WHEN** user runs `fmp calendar ipos-prospectus --from 2024-01-01`
- **THEN** returns IPO prospectus filings

### Requirement: Stock Splits Calendar
Track stock split events.

#### Scenario: Get splits for symbol
- **WHEN** user runs `fmp calendar splits --symbol AAPL --limit 10`
- **THEN** returns stock split history with date and ratio

#### Scenario: Get splits calendar
- **WHEN** user runs `fmp calendar splits-calendar --from 2024-01-01 --to 2024-12-31`
- **THEN** returns all stock split events in the date range
