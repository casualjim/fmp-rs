## ADDED Requirements

### Requirement: Company Profile
Retrieve comprehensive company profile information.

#### Scenario: Profile by ticker symbol
- **WHEN** user runs `fmp company profile --symbol AAPL`
- **THEN** returns company profile with name, sector, industry, CEO, employees, description, website, exchange, and market cap

#### Scenario: Profile by CIK number
- **WHEN** user runs `fmp company profile-cik --cik 0000320193`
- **THEN** returns company profile matching the SEC CIK identifier

### Requirement: Stock Peers
Identify peer companies in the same industry.

#### Scenario: Get company peers
- **WHEN** user runs `fmp company peers --symbol AAPL`
- **THEN** returns list of peer company symbols in the same industry sector

### Requirement: Market Capitalization
Retrieve current and historical market cap data.

#### Scenario: Current market cap
- **WHEN** user runs `fmp company market-cap --symbol AAPL`
- **THEN** returns current market capitalization value

#### Scenario: Batch market caps
- **WHEN** user runs `fmp company market-cap-batch --symbols AAPL,MSFT,GOOGL`
- **THEN** returns market caps for multiple symbols in a single request

#### Scenario: Historical market cap
- **WHEN** user runs `fmp company market-cap-history --symbol AAPL --from 2024-01-01 --to 2024-12-31`
- **THEN** returns historical market cap values over the date range

### Requirement: Executive Information
Access company leadership and compensation data.

#### Scenario: Key executives
- **WHEN** user runs `fmp company executives --symbol AAPL`
- **THEN** returns list of key executives with names and titles

#### Scenario: Executive compensation
- **WHEN** user runs `fmp company compensation --symbol AAPL`
- **THEN** returns detailed executive compensation data including salary, bonus, stock awards

### Requirement: Employee Count
Track company workforce size over time.

#### Scenario: Current employee count
- **WHEN** user runs `fmp company employees --symbol AAPL`
- **THEN** returns current and historical employee counts

#### Scenario: Historical employee data
- **WHEN** user runs `fmp company employees-history --symbol AAPL --limit 10`
- **THEN** returns historical employee counts with year-over-year changes

### Requirement: Share Float
Analyze share availability and ownership structure.

#### Scenario: Share float data
- **WHEN** user runs `fmp company float --symbol AAPL`
- **THEN** returns float shares, outstanding shares, and ownership percentages

#### Scenario: All shares float
- **WHEN** user runs `fmp company float-all --exchange NASDAQ`
- **THEN** returns share float data for all companies on the exchange

### Requirement: Mergers and Acquisitions
Track M&A activity.

#### Scenario: Latest M&A activity
- **WHEN** user runs `fmp company ma-latest --page 0`
- **THEN** returns recent merger and acquisition announcements

#### Scenario: Search M&A
- **WHEN** user runs `fmp company ma-search --name Apple`
- **THEN** returns M&A events matching the search criteria
