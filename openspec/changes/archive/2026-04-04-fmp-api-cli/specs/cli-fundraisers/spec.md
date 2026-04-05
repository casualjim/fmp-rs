## ADDED Requirements

### Requirement: Crowdfunding Offerings
Track equity crowdfunding campaigns.

#### Scenario: Get latest crowdfunding offerings
- **WHEN** user runs `fmp fundraiser crowdfunding-latest --limit 20`
- **THEN** returns recent crowdfunding campaigns with company, offering amount, and status

#### Scenario: Search crowdfunding offerings
- **WHEN** user runs `fmp fundraiser crowdfunding-search --name "Tech"`
- **THEN** returns crowdfunding campaigns matching the search criteria

#### Scenario: Get crowdfunding by CIK
- **WHEN** user runs `fmp fundraiser crowdfunding --cik 0001234567`
- **THEN** returns crowdfunding offerings for the specified company

### Requirement: Equity Offerings
Track equity fundraising activities.

#### Scenario: Get latest equity offerings
- **WHEN** user runs `fmp fundraiser equity-latest --limit 20`
- **THEN** returns recent equity offerings with company, offering type, and amount

#### Scenario: Search equity offerings
- **WHEN** user runs `fmp fundraiser equity-search --symbol AAPL`
- **THEN** returns equity offerings for the specified company

#### Scenario: Get equity offerings by CIK
- **WHEN** user runs `fmp fundraiser equity --cik 0000320193`
- **THEN** returns equity offerings for the specified company
