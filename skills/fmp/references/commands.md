# fmp CLI — Full Command Reference

## Global Flags

```
fmp [--api-key KEY] [--base-url URL] <namespace> <subcommand> [args...]
```

- `--api-key` / `FMP_API_KEY` env var
- `--base-url` (default: `https://financialmodelingprep.com/stable/`)

---

## quotes

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `get` | `SYMBOLS...` (positional, space-sep) | — | Full real-time quote (price, change, volume, mktcap, 52w range, MAs) |
| `short` | `SYMBOLS...` | — | Lightweight: price, change, volume only |
| `batch` | `SYMBOLS` (comma-sep string) | — | Full quotes for comma-sep symbols |
| `batch-short` | `SYMBOLS` | — | Lightweight for comma-sep symbols |
| `batch-aftermarket` | `SYMBOLS` | — | After-market quotes for comma-sep symbols |
| `exchange` | `EXCHANGE` | `--short` | All quotes for an exchange (e.g. NYSE, NASDAQ) |
| `aftermarket-trade` | `SYMBOLS...` | — | Most recent extended-hours trade |
| `aftermarket-quote` | `SYMBOLS...` | — | Extended-hours bid/ask |
| `price-change` | `SYMBOLS...` | — | Returns over 1D/5D/1M/3M/6M/YTD/1Y/3Y/5Y/10Y |
| `mutual-fund` | — | `--short` | All mutual fund quotes |
| `etf` | — | `--short` | All ETF quotes |
| `commodity` | — | `--short` | All commodity quotes |
| `crypto` | — | `--short` | All crypto quotes |
| `forex` | — | `--short` | All forex pair quotes |
| `index` | — | `--short` | All market index quotes |

---

## chart

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `eod-light` | `--symbol` | `--from`, `--to` | EOD OHLCV lightweight (split/div adjusted) |
| `eod-full` | `--symbol` | `--from`, `--to` | EOD OHLCV full (split/div adjusted) |
| `eod-non-split` | `--symbol` | `--from`, `--to` | EOD OHLCV raw (no split adj) |
| `eod-dividend` | `--symbol` | `--from`, `--to` | EOD OHLCV dividend-adjusted only |
| `intraday` | `--symbol` | `--interval` (default: 1hour), `--from`, `--to` | Intraday bars: 1min, 5min, 15min, 30min, 1hour, 4hour |
| `sma` | `--symbol` | `--period` (default: 20), `--timeframe` (default: daily), `--from`, `--to` | Simple Moving Average |
| `ema` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Exponential Moving Average |
| `wma` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Weighted Moving Average |
| `dema` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Double EMA |
| `tema` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Triple EMA |
| `rsi` | `--symbol` | `--period` (default: 20), `--timeframe`, `--from`, `--to` | Relative Strength Index |
| `stddev` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Standard Deviation (volatility) |
| `williams` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Williams %R |
| `adx` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Average Directional Index |

---

## company

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `profile` | `--symbol` | — | Full company profile |
| `profile-cik` | `--cik` | — | Profile by SEC CIK |
| `peers` | `--symbol` | — | Peer companies |
| `market-cap` | `--symbol` | — | Current market cap |
| `market-cap-batch` | `--symbols` (comma-sep) | — | Market cap for multiple symbols |
| `market-cap-history` | `--symbol` | `--limit`, `--from`, `--to` | Historical daily market cap |
| `executives` | `--symbol` | `--active` | Key executives |
| `compensation` | `--symbol` | — | Executive compensation |
| `employees` | `--symbol` | `--limit` | Current headcount |
| `employees-history` | `--symbol` | `--limit` | Historical headcount |
| `float` | `--symbol` | — | Shares float |
| `float-all` | — | `--page`, `--limit` | All companies float (paginated) |
| `ma-latest` | — | `--page` (default: 0) | Latest M&A announcements |
| `ma-search` | `--name` | — | Search M&A by company name |
| `notes` | `--symbol` | — | Company notes/disclosures |
| `delisted` | — | `--page`, `--limit` | Delisted companies (paginated) |
| `executive-compensation-benchmark` | `--symbol` | — | Executive compensation benchmark by industry |

---

## statements

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `income` | `--symbol` | `--period` (annual/quarterly), `--limit` | Income statement |
| `income-ttm` | `--symbol` | `--limit` | Income (trailing 12 months) |
| `income-growth` | `--symbol` | `--period`, `--limit` | Income YoY growth rates |
| `income-as-reported` | `--symbol` | `--period`, `--limit` | Income as filed (XBRL) |
| `balance-sheet` | `--symbol` | `--period`, `--limit` | Balance sheet |
| `balance-sheet-ttm` | `--symbol` | `--limit` | Balance sheet (TTM) |
| `balance-sheet-growth` | `--symbol` | `--period`, `--limit` | Balance sheet YoY growth |
| `balance-sheet-as-reported` | `--symbol` | `--period`, `--limit` | Balance sheet as filed |
| `cash-flow` | `--symbol` | `--period`, `--limit` | Cash flow statement |
| `cash-flow-ttm` | `--symbol` | `--limit` | Cash flow (TTM) |
| `cash-flow-growth` | `--symbol` | `--period`, `--limit` | Cash flow YoY growth |
| `cash-flow-as-reported` | `--symbol` | `--period`, `--limit` | Cash flow as filed |
| `full-as-reported` | `--symbol` | `--period`, `--limit` | Full financials as filed |
| `latest` | — | `--page`, `--limit` | Latest filings across all cos |
| `financial-growth` | `--symbol` | `--period`, `--limit` | Multi-metric growth rates |
| `report-dates` | `--symbol` | — | Filing dates list |
| `report-json` | `--symbol`, `--year`, `--period` (Q1/Q2/Q3/Q4/FY) | — | Full period report as JSON |
| `revenue-product` | `--symbol` | `--period`, `--structure` | Revenue by product |
| `revenue-geographic` | `--symbol` | `--period`, `--structure` | Revenue by region |
| `key-metrics` | `--symbol` | `--period`, `--limit` | P/E, EV/EBITDA, ROE, FCF yield… |
| `key-metrics-ttm` | `--symbol` | — | Key metrics (TTM) |
| `ratios` | `--symbol` | `--period`, `--limit` | Financial ratios |
| `ratios-ttm` | `--symbol` | — | Ratios (TTM) |
| `scores` | `--symbol` | `--limit` | Altman Z, Piotroski F, etc. |
| `owner-earnings` | `--symbol` | — | Buffett-style owner earnings |

---

## analyst

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `ratings` | `--symbol` | `--limit` | Current buy/sell/hold ratings |
| `ratings-historical` | `--symbol` | `--limit` | Historical rating changes |
| `price-target` | `--symbol` | — | Price target consensus |
| `price-target-summary` | `--symbol` | `--limit` | Target summary (avg/high/low) |
| `price-target-news` | `--symbol` | `--page`, `--limit` | Price target news articles for a symbol |
| `price-target-latest-news` | — | `--page`, `--limit` | Latest price target news across all symbols |
| `estimates` | `--symbol`, `--period` (annual/quarterly) | `--limit`, `--page` | EPS & revenue estimates |
| `grades` | `--symbol` | `--limit` | Grade changes (upgrade/downgrade) |
| `grades-news` | `--symbol` | `--page`, `--limit` | Grade change news articles for a symbol |
| `grades-latest-news` | — | `--page`, `--limit` | Latest grade change news across all symbols |
| `grades-historical` | `--symbol` | `--limit` | Historical buy/hold/sell counts |
| `grades-consensus` | `--symbol` | `--limit` | Grades consensus summary |

---

## news

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `latest` | — | `--from`, `--to`, `--limit`, `--page` | General market news |
| `stock` | — | `--from`, `--to`, `--limit`, `--page` | Stock-specific news |
| `crypto` | — | `--from`, `--to`, `--limit`, `--page` | Crypto news |
| `forex` | — | `--from`, `--to`, `--limit`, `--page` | Forex news |
| `search` | `--symbols` (comma-sep) | `--from`, `--to`, `--limit`, `--page` | Stock news for specific symbols |
| `fmp-articles` | — | `--from`, `--to`, `--limit`, `--page` | FMP editorial articles |
| `press-releases` | — | `--from`, `--to`, `--limit`, `--page` | Company press releases |
| `search-press-releases` | `--symbols` (comma-sep) | `--from`, `--to`, `--limit`, `--page` | Press releases for specific symbols |
| `search-crypto` | `--symbols` (comma-sep) | `--from`, `--to`, `--limit`, `--page` | Crypto news for specific symbols |
| `search-forex` | `--symbols` (comma-sep) | `--from`, `--to`, `--limit`, `--page` | Forex news for specific symbols |

---

## calendar

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `earnings` | — | `--from`, `--to` | Earnings announcements + EPS estimates |
| `earnings-confirmed` | `--symbol` | `--limit` | Confirmed dates for a company |
| `ipos` | — | `--from`, `--to` | IPO calendar |
| `ipos-disclosure` | — | `--from`, `--to` | IPO S-1/S-11 disclosure filings calendar |
| `ipos-prospectus` | — | `--from`, `--to` | IPO prospectus filings with pricing details |
| `splits` | — | `--from`, `--to` | Stock split calendar |
| `dividends` | — | `--from`, `--to` | Dividend payment calendar |

---

## filings

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `latest-8k` | — | `--from`, `--to`, `--limit`, `--page` | Latest 8-K filings |
| `latest-financials` | — | `--from`, `--to`, `--limit`, `--page` | Latest 10-K/10-Q filings |
| `by-type` | `--form-type` | `--from`, `--to`, `--limit`, `--page` | Filings by form type |
| `by-symbol` | `--symbol` | `--from`, `--to`, `--limit`, `--page` | All filings for a symbol |
| `by-cik` | `--cik` | `--from`, `--to`, `--limit`, `--page` | All filings for a CIK |
| `search-by-name` | `--name` | — | Search filings by company name |
| `search-by-symbol` | `--symbol` | — | Search filings by ticker |
| `sec-profile` | — | `--symbol`, `--cik` | SEC registrant profile (SIC, addresses) |
| `industry-classification-list` | — | `--industry-title`, `--sic-code` | SIC code list filtered by industry or code |
| `industry-classification-search` | — | `--symbol`, `--cik`, `--sic-code` | Search companies by SIC classification |
| `industry-classification-all` | — | `--page`, `--limit` | All industry classifications (paginated) |

---

## transcript

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `latest` | — | `--limit`, `--page` | Most recent transcripts |
| `get` | `--symbol`, `--year`, `--quarter` | `--limit` | Specific transcript |
| `dates` | `--symbol` | — | Available transcript dates |
| `available` | — | — | Companies with transcripts |

---

## search

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `symbol` | `--query` | `--limit`, `--exchange` | Symbol search |
| `name` | `--query` | `--limit`, `--exchange` | Company name search |
| `cik` | `--cik` | `--limit` | Symbol by SEC CIK |
| `cusip` | `--cusip` | — | Symbol by CUSIP |
| `isin` | `--isin` | — | Symbol by ISIN |
| `exchange-variants` | `--symbol` | — | All exchange listings for symbol |
| `screener` | — | `--market-cap-more-than`, `--market-cap-less-than`, `--price-more-than`, `--price-less-than`, `--beta-more-than`, `--beta-less-than`, `--volume-more-than`, `--volume-less-than`, `--dividend`, `--sector`, `--industry`, `--exchange`, `--country`, `--is-etf`, `--is-fund`, `--is-actively-trading`, `--limit`, `--page` | Stock screener |

---

## economics

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `indicators` | `--name` | `--from`, `--to` | Economic indicator series (GDP, CPI, etc.) |
| `treasury-rates` | — | `--from`, `--to` | US Treasury yield curve |
| `federal-fund-rate` | — | `--from`, `--to` | Fed funds effective rate |
| `calendar-events` | — | `--from`, `--to` | Upcoming economic events |
| `market-risk-premium` | — | — | Risk premium by country |

Common indicator names: `GDP`, `realGDP`, `nominalPotentialGDP`, `realGDPPerCapita`, `federalFunds`, `CPI`, `inflationRate`, `inflation`, `retailSales`, `consumerSentiment`, `durableGoods`, `unemploymentRate`, `totalNonfarmPayroll`, `industrialProductionTotalIndex`, `newPrivatelyOwnedHousingUnitsStartedTotalUnits`, `totalVehicleSales`, `retailMoneyFunds`, `smoothedUSRecessionProbabilities`, `3MonthOr90DayRatesAndYieldsCertificatesOfDeposit`, `commercialBankCreditCardInterestRateAsAPercentOfOutstandingBalancesSeasonallyAdjusted`, `30YearFixedRateMortgageAverage`, `15YearFixedRateMortgageAverage`

---

## crypto

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `list` | — | `--symbol` | All crypto pairs |
| `quote` | `--symbol` | — | Full quote |
| `quote-short` | `--symbol` | — | Lightweight quote |
| `quote-batch` | — | `--short` | All crypto quotes |
| `eod-light` | `--symbol` | `--from`, `--to` | Daily OHLCV lightweight |
| `eod-full` | `--symbol` | `--from`, `--to` | Daily OHLCV full |
| `intraday-1min` | `--symbol` | `--from`, `--to` | 1-min bars |
| `intraday-5min` | `--symbol` | `--from`, `--to` | 5-min bars |
| `intraday-1hour` | `--symbol` | `--from`, `--to` | 1-hour bars |
| `news` | — | `--from`, `--to`, `--limit`, `--page` | Crypto news |

Common symbols: `BTCUSD`, `ETHUSD`, `SOLUSD`, `XRPUSD`, `ADAUSD`

---

## forex

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `list` | — | — | All forex pairs |
| `quote` | `--symbol` | — | Full quote |
| `quote-short` | `--symbol` | — | Lightweight quote |
| `quote-batch` | — | `--short` | All forex quotes |
| `eod-light` | `--symbol` | `--from`, `--to` | Daily rate lightweight |
| `eod-full` | `--symbol` | `--from`, `--to` | Daily rate full |
| `intraday-1min` | `--symbol` | `--from`, `--to` | 1-min bars |
| `intraday-5min` | `--symbol` | `--from`, `--to` | 5-min bars |
| `intraday-1hour` | `--symbol` | `--from`, `--to` | 1-hour bars |
| `news` | — | `--from`, `--to`, `--limit`, `--page` | Forex news |

Common symbols: `EURUSD`, `GBPUSD`, `USDJPY`, `USDCHF`, `AUDUSD`, `GBPJPY`

---

## commodities

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `list` | — | — | All commodity contracts with pricing |

---

## indexes

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `list` | — | `--symbol` | All indexes |
| `quote` | `--symbol` | — | Full quote |
| `quote-short` | `--symbol` | — | Lightweight quote |
| `quote-batch` | — | `--short` | All index quotes |
| `eod-light` | `--symbol` | `--from`, `--to` | Daily value lightweight |
| `eod-full` | `--symbol` | `--from`, `--to` | Daily value full |
| `intraday-1min` | `--symbol` | `--from`, `--to` | 1-min bars |
| `intraday-5min` | `--symbol` | `--from`, `--to` | 5-min bars |
| `intraday-1hour` | `--symbol` | `--from`, `--to` | 1-hour bars |

Common symbols: `SPX` (S&P 500), `DJI` (Dow), `IXIC` (NASDAQ), `RUT` (Russell 2000), `VIX`

---

## market-performance

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `sector-performance-snapshot` | `--date` | `--exchange`, `--sector` | Sector perf on a date |
| `industry-performance-snapshot` | `--date` | `--exchange`, `--industry` | Industry perf on a date |
| `historical-sector-performance` | `--sector` | `--from`, `--to`, `--exchange` | Sector daily history |
| `historical-industry-performance` | `--industry` | `--from`, `--to`, `--exchange` | Industry daily history |
| `sector-pe-snapshot` | `--date` | `--exchange`, `--sector` | Sector P/E on a date |
| `industry-pe-snapshot` | `--date` | `--exchange`, `--industry` | Industry P/E on a date |
| `historical-sector-pe` | `--sector` | `--from`, `--to`, `--exchange` | Sector P/E history |
| `historical-industry-pe` | `--industry` | `--from`, `--to`, `--exchange` | Industry P/E history |
| `biggest-gainers` | — | — | Top % gainers today |
| `biggest-losers` | — | — | Top % losers today |
| `most-actives` | — | — | Most active by volume today |

---

## market-hours

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `exchange` | `--exchange` | — | Hours for one exchange |
| `holidays` | `--exchange` | `--from`, `--to` | Market holidays |
| `all` | — | — | Hours for all exchanges |

---

## directory

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `stock-list` | — | — | All listed stocks |
| `financial-statement-symbols` | — | — | Symbols with financials |
| `cik-list` | — | `--limit` | All SEC CIKs |
| `symbol-change` | — | `--invalid`, `--limit` | Historical symbol changes |
| `etf-list` | — | — | All ETFs |
| `actively-trading-list` | — | — | Actively trading stocks |
| `earnings-transcript-list` | — | — | Cos with transcripts |
| `available-exchanges` | — | — | Supported exchange codes |
| `available-sectors` | — | — | Supported sector names |
| `available-industries` | — | — | Supported industry names |
| `available-countries` | — | — | Supported country codes |

---

## dcf

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `valuation` | `--symbol` | — | Standard DCF intrinsic value |
| `levered-valuation` | `--symbol` | — | Levered DCF |
| `custom` | `--symbol` | 18 financial params (see below) | Custom DCF |
| `custom-levered` | `--symbol` | 18 financial params | Custom levered DCF |

Custom DCF params: `--revenue-growth-pct`, `--ebitda-pct`, `--depreciation-and-amortization-pct`, `--cash-and-short-term-investments-pct`, `--receivables-pct`, `--inventories-pct`, `--payable-pct`, `--ebit-pct`, `--capital-expenditure-pct`, `--operating-cash-flow-pct`, `--selling-general-and-administrative-expenses-pct`, `--tax-rate`, `--long-term-growth-rate`, `--cost-of-debt`, `--cost-of-equity`, `--market-risk-premium`, `--beta`, `--risk-free-rate`

---

## esg

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `disclosure` | `--symbol` | — | ESG disclosure metrics |
| `ratings` | `--symbol` | — | ESG ratings/scores |
| `benchmark` | — | `--year` | Sector/industry ESG benchmarks |

---

## cot

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `report` | — | `--symbol`, `--from`, `--to` | CFTC COT long/short positioning |
| `analysis` | — | `--symbol`, `--from`, `--to` | COT trend analysis |

---

## insider-trades

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `latest` | — | `--date`, `--page`, `--limit` | Latest insider transactions across all companies |
| `search` | — | `--symbol`, `--reporting-cik`, `--company-cik`, `--transaction-type`, `--page`, `--limit` | Search by symbol, CIK, or transaction type |
| `by-name` | `--name` | — | Transactions by reporting person name |
| `transaction-types` | `--symbol` | — | Transaction types filed for a company |
| `statistics` | `--symbol` | — | Insider trading statistics by quarter |
| `beneficial-ownership` | `--symbol` | `--limit` | Beneficial owners with 5%+ holdings |

Common `--transaction-type` values: `P-Purchase`, `S-Sale`, `A-Grant`, `D-Return`, `M-ExerciseOrConversion`

---

## government-trading

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `senate-latest` | — | `--page`, `--limit` | Latest Senate financial disclosure trades |
| `house-latest` | — | `--page`, `--limit` | Latest House financial disclosure trades |
| `senate-by-symbol` | `--symbol` | — | Senate trades for a ticker symbol |
| `senate-by-name` | `--name` | — | Senate trades by senator name |
| `house-by-symbol` | `--symbol` | — | House trades for a ticker symbol |
| `house-by-name` | `--name` | — | House trades by representative name |

---

## form13-f

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `latest` | — | `--page`, `--limit` | Latest 13F filings across all institutions |
| `extract` | `--cik`, `--year`, `--quarter` | — | Extract holdings from a specific filing |
| `dates` | `--cik` | — | Available filing dates for an institution |
| `holder-analytics` | `--symbol`, `--year`, `--quarter` | `--page`, `--limit` | Institutional holders for a symbol |
| `holder-performance` | `--cik` | `--page` | Performance summary for an institution |
| `holder-industry` | `--cik`, `--year`, `--quarter` | — | Industry breakdown for an institution |
| `symbol-positions` | `--symbol`, `--year`, `--quarter` | — | All 13F positions in a symbol |
| `industry-summary` | `--year`, `--quarter` | — | Industry performance across all 13F filers |

---

## fund

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `etf-holdings` | `--symbol` | — | ETF holdings list |
| `etf-info` | `--symbol` | — | ETF info (AUM, NAV, expense ratio, description) |
| `etf-countries` | `--symbol` | — | ETF country allocation weightings |
| `etf-assets` | `--symbol` | — | ETF individual asset exposures |
| `etf-sectors` | `--symbol` | — | ETF sector weightings |
| `disclosure-holders-latest` | `--symbol` | — | Latest fund disclosure holders |
| `disclosure-holders-search` | `--name` | — | Search fund disclosure holders by name |
| `disclosure-dates` | `--symbol` | — | Available disclosure dates for a fund |
| `disclosure` | `--symbol`, `--year`, `--quarter` | `--cik` | Fund portfolio disclosure for a period |

---

## fundraisers

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `crowdfunding-latest` | — | `--page`, `--limit` | Latest Reg CF crowdfunding filings |
| `crowdfunding-search` | `--name` | — | Search crowdfunding offerings by company name |
| `crowdfunding-by-cik` | `--cik` | — | Crowdfunding filings for a specific company |
| `equity-latest` | — | `--page`, `--limit` | Latest Reg D/Reg A equity fundraising filings |
| `equity-search` | `--name` | — | Search equity offerings by company name |
| `equity-by-cik` | `--cik` | — | Equity fundraising filings for a specific company |

---

## technical-indicators

All subcommands share the same argument set:

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `sma` | `--symbol` | `--period` (default: 20), `--timeframe` (default: daily), `--from`, `--to` | Simple Moving Average |
| `ema` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Exponential Moving Average |
| `wma` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Weighted Moving Average |
| `dema` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Double Exponential Moving Average |
| `tema` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Triple Exponential Moving Average |
| `rsi` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Relative Strength Index |
| `std-dev` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Standard Deviation (volatility) |
| `williams` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Williams %R (overbought/oversold) |
| `adx` | `--symbol` | `--period`, `--timeframe`, `--from`, `--to` | Average Directional Index (trend strength) |

`--from`/`--to` accept ISO 8601 datetime strings (e.g., `2024-01-01T00:00:00Z`).
`--timeframe` values: `1min`, `5min`, `15min`, `30min`, `1hour`, `4hour`, `daily`

---

## bulk

| Subcommand | Required | Optional | Description |
|-----------|---------|---------|-------------|
| `profile` | — | `--part <0-3>` (default: 0), `--all`, `--format` | Full company profiles for all symbols |
| `etf-holder` | — | `--part <0-3>` (default: 0), `--all`, `--format` | ETF holdings for all ETFs |
| `eod` | `--date <YYYY-MM-DD>` | `--format` | End-of-day OHLCV for all symbols on a given date |
| `earnings-surprises` | `--year` | `--format` | EPS actual vs estimated for all symbols for a year |
| `income-statement` | `--year`, `--period` | `--format` | Revenue, expenses, net income, EPS for all symbols |
| `balance-sheet-statement` | `--year`, `--period` | `--format` | Assets, liabilities, equity for all symbols |
| `cash-flow-statement` | `--year`, `--period` | `--format` | Operating/investing/financing cash flows for all symbols |
| `income-statement-growth` | `--year`, `--period` | `--format` | YoY income statement growth rates for all symbols |
| `balance-sheet-statement-growth` | `--year`, `--period` | `--format` | YoY balance sheet growth rates for all symbols |
| `cash-flow-statement-growth` | `--year`, `--period` | `--format` | YoY cash flow growth rates for all symbols |
| `key-metrics-ttm` | — | `--format` | TTM key metrics (EV/EBITDA, ROE, FCF yield, etc.) for all symbols |
| `ratios-ttm` | — | `--format` | TTM financial ratios (P/E, P/B, debt/equity, margins, etc.) for all symbols |
| `scores` | — | `--format` | Altman Z-score and Piotroski F-score for all symbols |
| `rating` | — | `--format` | Composite stock rating and component recommendations for all symbols |
| `upgrades-downgrades-consensus` | — | `--format` | Analyst consensus (strong buy/buy/hold/sell counts) for all symbols |
| `price-target-summary` | — | `--format` | Average price targets over 1M/1Q/1Y/all-time for all symbols |
| `peers` | — | `--format` | Peer symbol list for all symbols |
| `dcf` | — | `--format` | Discounted cash flow valuation for all symbols |

`--format` values: `json` (NDJSON, one record per line, default) or `csv` (with header row).
`--period` values: `annual`, `quarter`.
`--all` fetches all 4 parts of `profile`/`etf-holder` sequentially, pacing ≥60 s between requests. Mutually exclusive with `--part`.
