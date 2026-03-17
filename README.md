# fmp-rs

Rust client for the [Financial Modeling Prep](https://financialmodelingprep.com/) stable API.

## Features

- Trait-based API — each endpoint group is an async trait implemented on `FmpHttpClient`
- Strongly typed request params and response structs
- Built-in retry with exponential backoff and request tracing via `reqwest-middleware`
- `jiff` for dates and timestamps, `secrecy` for the API key

## Installation

```toml
[dependencies]
fmp = { git = "https://code.lotsa.ai/lbyl/fmp-rs" }
```

## Quick Start

```rust
use fmp::{FmpConfig, FmpHttpClient, QuotesApi};
use fmp::types::quotes::QuoteParams;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = FmpConfig::new("your-api-key")?;
    let client = FmpHttpClient::new(config)?;

    let params = QuoteParams::builder().symbol("AAPL".into()).build();
    let quotes = client.quote(params).await?;
    println!("{:#?}", quotes[0]);
    Ok(())
}
```

## Configuration

```rust
use std::time::Duration;
use fmp::FmpConfig;

let config = FmpConfig::builder()
    .api_key("your-api-key")
    .timeout(Duration::from_secs(30))
    .retry_attempts(5u32)
    .build();
```

The default base URL is `https://financialmodelingprep.com/stable/`.

## Usage

Each endpoint group is brought into scope by importing its trait. All methods take a typed params struct and return a `FmpResult<T>`.

```rust
use fmp::{FmpHttpClient, StatementsApi, QuotesApi, NewsApi};
use fmp::types::statements::PeriodParams;
use fmp::types::quotes::QuoteParams;

// Financial statements
let params = PeriodParams::builder().symbol("AAPL".into()).limit(5u32).build();
let income = client.income_statement(params).await?;

// News
let news = client.stock_news_latest(Default::default()).await?;
```

### Shared client

`FmpHttpClient` is `Clone` and cheaply shareable via `Arc`:

```rust
let client = FmpHttpClient::new(config)?.into_arc();
```

## API Coverage

| Trait | Endpoints |
|-------|-----------|
| `QuotesApi` | quote, quote-short, aftermarket, batch quotes, exchange/ETF/crypto/forex/index quotes |
| `ChartApi` / `ChartIntervalApi` | EOD (light/full/unadjusted), intraday (1min–1day) |
| `CompanyApi` | profile, executives, market cap, peers, notes, share float, M&A |
| `StatementsApi` | income/balance/cash-flow (annual+quarterly+TTM+growth+as-reported), key metrics, ratios, scores |
| `CalendarApi` | earnings, dividends, IPOs, splits |
| `NewsApi` | FMP articles, stock/crypto/forex news, press releases |
| `AnalystApi` | estimates, ratings, price targets, grades |
| `SearchApi` | symbol/name/CIK/CUSIP/ISIN search, stock screener |
| `MarketPerformanceApi` | gainers, losers, actives, sector/industry PE and performance |
| `CryptoApi` | crypto list, quotes, historical |
| `ForexApi` | forex pairs, quotes, historical |
| `IndexesApi` | index list, quotes, historical |
| `EconomicsApi` | treasury rates, economic indicators, calendar, market risk premium |
| `EsgApi` | ESG disclosures, ratings, benchmarks |
| `TechnicalIndicatorsApi` | SMA, EMA, WMA, DEMA, TEMA, RSI, StdDev, Williams, ADX |
| `DcfApi` | DCF valuation, levered DCF, custom DCF |
| `CotApi` | Commitment of Traders reports and analysis |
| `InsiderTradesApi` | insider transactions, statistics, beneficial ownership |
| `MarketHoursApi` | exchange hours, holidays |
| `SecFilingsApi` | SEC filings search, company search, SEC profile, SIC classification |
| `Form13FApi` | institutional ownership (latest, extract, dates, holder summary, industry) |
| `FundApi` | ETF holdings, info, country/sector/asset weightings, fund disclosures |
| `GovernmentTradingApi` | Senate and House trade disclosures |
| `FundraisersApi` | crowdfunding offerings, equity fundraising |
| `EarningsTranscriptApi` | transcript text, dates, latest |
| `DirectoryApi` | available exchanges, sectors, industries, countries, CIK list |
| `CommodityApi` | commodities list |

## Tests

Fixture-based serde unit tests are co-located in each type module and require no API key:

```bash
cargo test
```

Live integration tests hit the real API and require `FMP_API_KEY`:

```bash
FMP_API_KEY=your-key cargo test --all-features
```

Regenerate fixtures (requires API key):

```bash
FMP_API_KEY=your-key bash scripts/fetch-fixtures.sh
```
