## ADDED Requirements

### Requirement: Income Statement
Retrieve company income statement data.

#### Scenario: Annual income statement
- **WHEN** user runs `fmp statements income --symbol AAPL --period annual --limit 5`
- **THEN** returns annual income statements with revenue, gross profit, operating income, net income, EPS

#### Scenario: Quarterly income statement
- **WHEN** user runs `fmp statements income --symbol AAPL --period quarterly --limit 4`
- **THEN** returns quarterly income statement data

#### Scenario: TTM income statement
- **WHEN** user runs `fmp statements income-ttm --symbol AAPL`
- **THEN** returns trailing twelve months income statement data

### Requirement: Balance Sheet Statement
Retrieve company balance sheet data.

#### Scenario: Annual balance sheet
- **WHEN** user runs `fmp statements balance-sheet --symbol AAPL --period annual --limit 5`
- **THEN** returns assets, liabilities, and shareholders equity data

#### Scenario: As-reported balance sheet
- **WHEN** user runs `fmp statements balance-sheet-as-reported --symbol AAPL`
- **THEN** returns raw balance sheet data as filed with SEC

### Requirement: Cash Flow Statement
Retrieve company cash flow data.

#### Scenario: Cash flow statement
- **WHEN** user runs `fmp statements cash-flow --symbol AAPL --period annual --limit 5`
- **THEN** returns operating, investing, and financing cash flow data

#### Scenario: Cash flow growth
- **WHEN** user runs `fmp statements cash-flow-growth --symbol AAPL --period annual`
- **THEN** returns cash flow metrics with period-over-period growth percentages

### Requirement: Financial Growth Metrics
Analyze financial statement growth trends.

#### Scenario: Income statement growth
- **WHEN** user runs `fmp statements income-growth --symbol AAPL --period annual`
- **THEN** returns revenue growth, net income growth, and EPS growth percentages

#### Scenario: Overall financial growth
- **WHEN** user runs `fmp statements financial-growth --symbol AAPL --period annual`
- **THEN** returns comprehensive growth metrics across all financial statements

### Requirement: Financial Reports
Access full financial reports and filing dates.

#### Scenario: Report dates
- **WHEN** user runs `fmp statements report-dates --symbol AAPL`
- **THEN** returns list of available financial report filing dates

#### Scenario: 10-K report JSON
- **WHEN** user runs `fmp statements report-json --symbol AAPL --year 2024 --period FY`
- **THEN** returns the complete 10-K annual report in JSON format

### Requirement: Revenue Segmentation
Analyze revenue by product and geography.

#### Scenario: Product segmentation
- **WHEN** user runs `fmp statements revenue-product --symbol AAPL`
- **THEN** returns revenue breakdown by product line

#### Scenario: Geographic segmentation
- **WHEN** user runs `fmp statements revenue-geographic --symbol AAPL`
- **THEN** returns revenue breakdown by geographic region

### Requirement: Key Metrics and Ratios
Access financial ratios and key performance indicators.

#### Scenario: Key metrics
- **WHEN** user runs `fmp statements key-metrics --symbol AAPL --period annual`
- **THEN** returns PE ratio, PB ratio, ROE, ROA, debt-to-equity, and other metrics

#### Scenario: Financial ratios
- **WHEN** user runs `fmp statements ratios --symbol AAPL --period annual`
- **THEN** returns comprehensive financial ratios for analysis

#### Scenario: Financial scores
- **WHEN** user runs `fmp statements scores --symbol AAPL`
- **THEN** returns Altman Z-score, Piotroski score, and other scoring metrics
