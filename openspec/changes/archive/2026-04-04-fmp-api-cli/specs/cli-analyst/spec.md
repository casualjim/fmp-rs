## ADDED Requirements

### Requirement: Analyst Estimates
Retrieve analyst earnings estimates.

#### Scenario: Get earnings estimates
- **WHEN** user runs `fmp analyst estimates --symbol AAPL --period quarterly`
- **THEN** returns analyst EPS estimates with consensus, high, low, and number of analysts

#### Scenario: Get revenue estimates
- **WHEN** user runs `fmp analyst estimates --symbol AAPL --type revenue`
- **THEN** returns analyst revenue estimates

### Requirement: Stock Ratings
Access analyst ratings and recommendations.

#### Scenario: Get ratings snapshot
- **WHEN** user runs `fmp analyst ratings-snapshot --symbol AAPL`
- **THEN** returns current rating summary with buy/sell/hold recommendations

#### Scenario: Get historical ratings
- **WHEN** user runs `fmp analyst ratings-historical --symbol AAPL --limit 20`
- **THEN** returns historical rating changes with dates and analysts

### Requirement: Price Targets
Track analyst price targets.

#### Scenario: Get price target summary
- **WHEN** user runs `fmp analyst price-target-summary --symbol AAPL`
- **THEN** returns consensus price target with high, low, median, and current price

#### Scenario: Get price target consensus
- **WHEN** user runs `fmp analyst price-target-consensus --symbol AAPL`
- **THEN** returns price target consensus data

#### Scenario: Get price target news
- **WHEN** user runs `fmp analyst price-target-news --symbol AAPL --page 0`
- **THEN** returns recent price target updates with analyst firms

### Requirement: Stock Grades
Access analyst stock grades.

#### Scenario: Get stock grades
- **WHEN** user runs `fmp analyst grades --symbol AAPL --limit 10`
- **THEN** returns recent analyst grades with firm, action, and grade

#### Scenario: Get historical grades
- **WHEN** user runs `fmp analyst grades-historical --symbol AAPL --limit 20`
- **THEN** returns historical grade changes

#### Scenario: Get grade consensus
- **WHEN** user runs `fmp analyst grades-consensus --symbol AAPL`
- **THEN** returns overall grade consensus summary

#### Scenario: Get grade news
- **WHEN** user runs `fmp analyst grades-news --symbol AAPL --page 0`
- **THEN** returns recent grading news updates
