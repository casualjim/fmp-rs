## ADDED Requirements

### Requirement: Latest SEC Filings
Access recent SEC filings.

#### Scenario: Get latest 8-K filings
- **WHEN** user runs `fmp sec filings-8k --limit 50`
- **THEN** returns recent 8-K filings with company, date, and filing link

#### Scenario: Get latest financial filings
- **WHEN** user runs `fmp sec filings-financials --from 2024-01-01 --to 2024-01-31`
- **THEN** returns 10-Q and 10-K financial filings in the date range

### Requirement: Search SEC Filings
Search filings by various criteria.

#### Scenario: Search by form type
- **WHEN** user runs `fmp sec search-form --form-type 10-K --limit 20`
- **THEN** returns filings of the specified form type

#### Scenario: Search by symbol
- **WHEN** user runs `fmp sec search-symbol --symbol AAPL --limit 20`
- **THEN** returns all filings for the specified company

#### Scenario: Search by CIK
- **WHEN** user runs `fmp sec search-cik --cik 0000320193 --limit 20`
- **THEN** returns all filings for the specified CIK

### Requirement: Company Search
Search for SEC-registered companies.

#### Scenario: Search by company name
- **WHEN** user runs `fmp sec company-search-name --query Apple`
- **THEN** returns companies with matching names

#### Scenario: Search by symbol
- **WHEN** user runs `fmp sec company-search-symbol --query AAPL`
- **THEN** returns company information for matching symbols

#### Scenario: Search by CIK
- **WHEN** user runs `fmp sec company-search-cik --query 0000320193`
- **THEN** returns company information for matching CIK

### Requirement: SEC Company Profile
Get detailed SEC profile information.

#### Scenario: Get SEC profile
- **WHEN** user runs `fmp sec profile --symbol AAPL`
- **THEN** returns SEC filing profile with company details

### Requirement: Industry Classification
Access SIC and industry classification data.

#### Scenario: Get industry classifications
- **WHEN** user runs `fmp sec industry-list`
- **THEN** returns list of all SIC industry classifications

#### Scenario: Search by industry
- **WHEN** user runs `fmp sec industry-search --industry Technology`
- **THEN** returns companies in the specified industry

#### Scenario: Get all industry classifications
- **WHEN** user runs `fmp sec industry-all --limit 100`
- **THEN** returns all company industry classifications
