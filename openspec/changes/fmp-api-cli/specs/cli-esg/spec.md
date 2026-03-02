## ADDED Requirements

### Requirement: ESG Disclosure
Access company ESG disclosure data.

#### Scenario: Get ESG disclosure
- **WHEN** user runs `fmp esg disclosure --symbol AAPL`
- **THEN** returns ESG disclosure data with environmental, social, and governance metrics

### Requirement: ESG Ratings
Retrieve company ESG ratings.

#### Scenario: Get ESG ratings
- **WHEN** user runs `fmp esg ratings --symbol AAPL`
- **THEN** returns ESG ratings with overall score and individual E, S, G component scores

#### Scenario: Compare ESG ratings
- **WHEN** user runs `fmp esg ratings --symbol AAPL --symbol MSFT`
- **THEN** returns ESG ratings for comparison

### Requirement: ESG Benchmark
Compare company ESG performance to benchmarks.

#### Scenario: Get ESG benchmark
- **WHEN** user runs `fmp esg benchmark --symbol AAPL --year 2024`
- **THEN** returns ESG performance compared to industry and market benchmarks

#### Scenario: Get benchmark by sector
- **WHEN** user runs `fmp esg benchmark --sector Technology --year 2024`
- **THEN** returns sector-level ESG benchmark data
