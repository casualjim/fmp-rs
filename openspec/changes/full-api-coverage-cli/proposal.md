## Why

The `fmp` API library has full implementations for 6 endpoint modules — insider trades, government trading, 13F filings, fund/ETF data, fundraisers, and technical indicators — that are completely absent from the CLI. Additionally, several implemented modules have partial CLI coverage (analyst, calendar, news, SEC filings, company). Users of the CLI cannot access a significant portion of the API's capabilities.

## What Changes

- Add new CLI command modules for all API modules currently missing from the CLI:
  - `insider-trades`: Expose insider trading latest, search, by name, by transaction type, statistics, and beneficial ownership
  - `government-trading`: Expose Senate and House trade disclosures (latest and by name/ticker)
  - `form-13f`: Expose institutional ownership latest, extract, dates, holder/industry analytics
  - `fund`: Expose ETF holdings, info, sector/country/asset weightings, and fund disclosure endpoints
  - `fundraisers`: Expose crowdfunding offerings and equity fundraising endpoints
  - `technical-indicators`: Add a dedicated `technical-indicators` subcommand (SMA, EMA, WMA, DEMA, TEMA, RSI, StdDev, Williams, ADX) — currently buried in `chart`
- Fill partial coverage gaps in existing CLI modules:
  - `analyst`: Add price target news, grades news, grades historical
  - `calendar`: Add dividends, IPO disclosure, IPO prospectus
  - `news`: Add search for press releases, crypto news, forex news
  - `sec-filings`: Add SEC profile, industry classification list/search/all
  - `company`: Add company notes, delisted companies, executive compensation benchmark
  - `crypto`/`forex`: Add `historical-chart-1hour` subcommand

## Capabilities

### New Capabilities

- `cli-insider-trades`: CLI command exposing all InsiderTradesApi endpoints
- `cli-government-trading`: CLI command exposing all GovernmentTradingApi endpoints
- `cli-form-13f`: CLI command exposing all Form13FApi endpoints
- `cli-fund`: CLI command exposing all FundApi endpoints
- `cli-fundraisers`: CLI command exposing all FundraisersApi endpoints
- `cli-technical-indicators`: Dedicated CLI command for TechnicalIndicatorsApi (currently partially embedded in chart)
- `cli-partial-coverage-gaps`: Fill missing subcommands in analyst, calendar, news, sec-filings, company, crypto, forex

### Modified Capabilities

<!-- No existing spec-level requirement changes -->

## Impact

- `crates/fmp-cli/src/cli/`: New files for each new command module; edits to existing partial modules
- `crates/fmp-cli/src/cli/mod.rs`: Register new command modules
- `crates/fmp-cli/src/main.rs` or top-level CLI enum: Add new subcommands to the top-level dispatch
- No changes to `src/` (API library) — all API implementations already exist
- No new dependencies required
