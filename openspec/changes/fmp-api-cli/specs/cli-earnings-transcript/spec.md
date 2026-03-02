## ADDED Requirements

### Requirement: Latest Earnings Transcripts
Access recent earnings call transcripts.

#### Scenario: Get latest transcripts
- **WHEN** user runs `fmp transcript latest --limit 20`
- **THEN** returns latest earnings call transcripts with company, date, and content

#### Scenario: Get latest for symbol
- **WHEN** user runs `fmp transcript latest --symbol AAPL`
- **THEN** returns the most recent earnings transcript for the specified company

### Requirement: Specific Transcript
Retrieve a specific earnings transcript.

#### Scenario: Get transcript by date
- **WHEN** user runs `fmp transcript get --symbol AAPL --quarter 1 --year 2024`
- **THEN** returns the Q1 2024 earnings call transcript content

#### Scenario: Get transcript by quarter
- **WHEN** user runs `fmp transcript get --symbol AAPL --quarter 4 --year 2023`
- **THEN** returns the Q4 2023 earnings call transcript content

### Requirement: Transcript Dates
Get available transcript dates for a company.

#### Scenario: Get transcript dates
- **WHEN** user runs `fmp transcript dates --symbol AAPL`
- **THEN** returns list of all available earnings transcript dates with quarter and year

### Requirement: Available Transcript Symbols
List companies with available transcripts.

#### Scenario: Get all transcript symbols
- **WHEN** user runs `fmp transcript symbols`
- **THEN** returns list of all symbols that have earnings call transcripts available
