use clap::{Args, Subcommand};
use eyre::Result;
use fmp::QuotesApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum QuotesArgs {
  /// Full quote for one or more symbols (price, volume, change, market cap, etc.)
  Get(GetArgs),
  /// Lightweight quote with price and basic metrics only
  Short(ShortArgs),
  /// Batch full quotes using comma-separated symbols string
  Batch(BatchArgs),
  /// Batch lightweight quotes using comma-separated symbols string
  BatchShort(BatchShortArgs),
  /// Batch after-market quotes for multiple symbols
  BatchAftermarket(BatchAftermarketArgs),
  /// All quotes for a specific exchange (e.g., NYSE, NASDAQ)
  Exchange(ExchangeArgs),
  /// After-market trade data for one or more symbols
  AftermarketTrade(AftermarketTradeArgs),
  /// After-market quote data for one or more symbols
  AftermarketQuote(AftermarketQuoteArgs),
  /// Price change over multiple periods (1D, 5D, 1M, 3M, 6M, YTD, 1Y, 3Y, 5Y, 10Y)
  PriceChange(PriceChangeArgs),
  /// All mutual fund quotes (optionally as short/lightweight format)
  MutualFund(MutualFundArgs),
  /// All ETF quotes (optionally as short/lightweight format)
  Etf(EtfArgs),
  /// All commodity quotes (optionally as short/lightweight format)
  Commodity(CommodityArgs),
  /// All cryptocurrency quotes (optionally as short/lightweight format)
  Crypto(CryptoArgs),
  /// All forex pair quotes (optionally as short/lightweight format)
  Forex(ForexArgs),
  /// All market index quotes (optionally as short/lightweight format)
  Index(IndexArgs),
}

impl QuotesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::Get(args) => args.handle(ctx).await,
      Self::Short(args) => args.handle(ctx).await,
      Self::Batch(args) => args.handle(ctx).await,
      Self::BatchShort(args) => args.handle(ctx).await,
      Self::BatchAftermarket(args) => args.handle(ctx).await,
      Self::Exchange(args) => args.handle(ctx).await,
      Self::AftermarketTrade(args) => args.handle(ctx).await,
      Self::AftermarketQuote(args) => args.handle(ctx).await,
      Self::PriceChange(args) => args.handle(ctx).await,
      Self::MutualFund(args) => args.handle(ctx).await,
      Self::Etf(args) => args.handle(ctx).await,
      Self::Commodity(args) => args.handle(ctx).await,
      Self::Crypto(args) => args.handle(ctx).await,
      Self::Forex(args) => args.handle(ctx).await,
      Self::Index(args) => args.handle(ctx).await,
    }
  }
}

/// Fetch full real-time quotes for one or more symbols.
///
/// Returns price, change, change-percent, volume, market cap, 52-week high/low,
/// 50/200-day moving averages, open, previous close, and last-update timestamp.
///
/// Examples:
///   fmp quotes get AAPL
///   fmp quotes get AAPL MSFT GOOGL
#[derive(Args, Debug, Clone)]
pub struct GetArgs {
  #[arg(required = true, help = "One or more ticker symbols (e.g., AAPL MSFT GOOGL)")]
  pub symbols: Vec<String>,
}

impl GetArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let symbols = self.symbols.join(",");
    let params = fmp::types::quotes::BatchQuoteParams::builder().symbols(symbols).build();
    let quotes = ctx.client.batch_quote(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch lightweight quotes for one or more symbols.
///
/// Returns only symbol, price, change, and volume — much smaller payload than
/// the full quote. Useful for dashboards that poll many symbols frequently.
///
/// Examples:
///   fmp quotes short AAPL
///   fmp quotes short AAPL MSFT GOOGL
#[derive(Args, Debug, Clone)]
pub struct ShortArgs {
  #[arg(required = true, help = "One or more ticker symbols (e.g., AAPL MSFT)")]
  pub symbols: Vec<String>,
}

impl ShortArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let symbols = self.symbols.join(",");
    let params = fmp::types::quotes::BatchQuoteParams::builder().symbols(symbols).build();
    let quotes = ctx.client.batch_quote_short(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch full quotes for multiple symbols in a single API request.
///
/// Pass symbols as a single comma-separated string. More efficient than calling
/// `get` repeatedly when you need full quote data for many tickers at once.
///
/// Examples:
///   fmp quotes batch "AAPL,MSFT,GOOGL"
///   fmp quotes batch "SPY,QQQ,IWM,DIA"
#[derive(Args, Debug, Clone)]
pub struct BatchArgs {
  #[arg(required = true, help = "Comma-separated symbols (e.g., \"AAPL,MSFT,GOOGL\")")]
  pub symbols: String,
}

impl BatchArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::quotes::BatchQuoteParams::builder()
      .symbols(&self.symbols)
      .build();
    let quotes = ctx.client.batch_quote(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch lightweight quotes for multiple symbols in a single API request.
///
/// Returns only symbol, price, change, and volume. Combine this with `batch`
/// when you want to minimise bandwidth while polling a large watchlist.
///
/// Example:
///   fmp quotes batch-short "AAPL,MSFT,GOOGL,AMZN,META"
#[derive(Args, Debug, Clone)]
pub struct BatchShortArgs {
  #[arg(required = true, help = "Comma-separated symbols (e.g., \"AAPL,MSFT,GOOGL\")")]
  pub symbols: String,
}

impl BatchShortArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::quotes::BatchQuoteParams::builder()
      .symbols(&self.symbols)
      .build();
    let quotes = ctx.client.batch_quote_short(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch after-hours bid/ask quotes for multiple symbols in a single request.
///
/// Returns the best bid price/size and ask price/size during extended-hours
/// trading, along with total extended-hours volume and timestamp.
///
/// Example:
///   fmp quotes batch-aftermarket "AAPL,MSFT,NVDA"
#[derive(Args, Debug, Clone)]
pub struct BatchAftermarketArgs {
  #[arg(required = true, help = "Comma-separated symbols (e.g., \"AAPL,MSFT,NVDA\")")]
  pub symbols: String,
}

impl BatchAftermarketArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::quotes::BatchQuoteParams::builder()
      .symbols(&self.symbols)
      .build();
    let quotes = ctx.client.batch_aftermarket_quote(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch quotes for all securities listed on a given exchange.
///
/// Returns either full or lightweight quotes for every actively trading
/// security on the specified exchange. Large exchanges (NYSE, NASDAQ) return
/// thousands of records; use `--short` to reduce payload size.
///
/// Common exchange codes: NYSE, NASDAQ, AMEX, LSE, TSX, ASX, HKEX
///
/// Examples:
///   fmp quotes exchange NASDAQ
///   fmp quotes exchange NYSE --short true
#[derive(Args, Debug, Clone)]
pub struct ExchangeArgs {
  #[arg(required = true, help = "Exchange code (e.g., NYSE, NASDAQ, AMEX, LSE, TSX)")]
  pub exchange: String,

  #[arg(
    long,
    help = "Return lightweight quotes (symbol, price, change, volume) instead of full quotes"
  )]
  pub short: Option<bool>,
}

impl ExchangeArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.short {
      Some(short) => fmp::types::quotes::ExchangeQuoteParams::builder()
        .exchange(&self.exchange)
        .short(short)
        .build(),
      None => fmp::types::quotes::ExchangeQuoteParams::builder()
        .exchange(&self.exchange)
        .build(),
    };
    let quotes = ctx.client.exchange_quotes(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch the most recent extended-hours trade for one or more symbols.
///
/// Returns the last executed trade price, share size, and timestamp for
/// after-hours or pre-market sessions. Useful for monitoring overnight
/// price movements before regular market open.
///
/// Examples:
///   fmp quotes aftermarket-trade AAPL
///   fmp quotes aftermarket-trade AAPL MSFT NVDA
#[derive(Args, Debug, Clone)]
pub struct AftermarketTradeArgs {
  #[arg(required = true, help = "One or more ticker symbols (e.g., AAPL MSFT)")]
  pub symbols: Vec<String>,
}

impl AftermarketTradeArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let symbols = self.symbols.join(",");
    let params = fmp::types::quotes::BatchQuoteParams::builder().symbols(symbols).build();
    let quotes = ctx.client.batch_aftermarket_trade(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch after-hours or pre-market bid/ask quotes for one or more symbols.
///
/// Returns the current best bid (price + size) and ask (price + size), total
/// extended-hours volume, and a timestamp. The bid-ask spread during
/// extended hours is typically wider than during regular session.
///
/// Examples:
///   fmp quotes aftermarket-quote AAPL
///   fmp quotes aftermarket-quote AAPL TSLA
#[derive(Args, Debug, Clone)]
pub struct AftermarketQuoteArgs {
  #[arg(required = true, help = "One or more ticker symbols (e.g., AAPL TSLA)")]
  pub symbols: Vec<String>,
}

impl AftermarketQuoteArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let symbols = self.symbols.join(",");
    let params = fmp::types::quotes::BatchQuoteParams::builder().symbols(symbols).build();
    let quotes = ctx.client.batch_aftermarket_quote(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch percentage price returns over standard time horizons.
///
/// Returns 1D, 5D, 1M, 3M, 6M, YTD, 1Y, 3Y, 5Y, 10Y, and max (since IPO)
/// percentage changes for each symbol. All values are percentages
/// (e.g., 5.23 means +5.23%).
///
/// Examples:
///   fmp quotes price-change AAPL
///   fmp quotes price-change AAPL MSFT GOOGL
#[derive(Args, Debug, Clone)]
pub struct PriceChangeArgs {
  #[arg(required = true, help = "One or more ticker symbols (e.g., AAPL MSFT)")]
  pub symbols: Vec<String>,
}

impl PriceChangeArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let symbols = self.symbols.join(",");
    let params = fmp::types::quotes::QuoteParams::builder().symbol(symbols).build();
    let quotes = ctx.client.stock_price_change(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch quotes for all mutual funds tracked by FMP.
///
/// Returns NAV-based pricing data for thousands of mutual fund share classes.
/// Use `--short true` to get only symbol, price, change, and volume.
///
/// Example:
///   fmp quotes mutual-fund
///   fmp quotes mutual-fund --short true
#[derive(Args, Debug, Clone)]
pub struct MutualFundArgs {
  #[arg(
    long,
    help = "Return lightweight quotes (symbol, price, change, volume) instead of full quotes"
  )]
  pub short: Option<bool>,
}

impl MutualFundArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.short {
      Some(short) => fmp::types::quotes::ShortParams::builder().short(short).build(),
      None => fmp::types::quotes::ShortParams::builder().build(),
    };
    let quotes = ctx.client.mutual_fund_quotes(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch quotes for all ETFs tracked by FMP.
///
/// Covers equity, bond, commodity, currency, and leveraged/inverse ETFs
/// listed globally. Use `--short true` to reduce payload size.
///
/// Example:
///   fmp quotes etf
///   fmp quotes etf --short true
#[derive(Args, Debug, Clone)]
pub struct EtfArgs {
  #[arg(
    long,
    help = "Return lightweight quotes (symbol, price, change, volume) instead of full quotes"
  )]
  pub short: Option<bool>,
}

impl EtfArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.short {
      Some(short) => fmp::types::quotes::ShortParams::builder().short(short).build(),
      None => fmp::types::quotes::ShortParams::builder().build(),
    };
    let quotes = ctx.client.etf_quotes(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch quotes for all commodity futures contracts tracked by FMP.
///
/// Covers energy (crude oil, natural gas), metals (gold, silver, copper),
/// and agricultural commodities. Symbols typically follow the pattern GCUSD
/// (gold), CLUSD (crude oil), SIUSD (silver).
///
/// Example:
///   fmp quotes commodity
///   fmp quotes commodity --short true
#[derive(Args, Debug, Clone)]
pub struct CommodityArgs {
  #[arg(
    long,
    help = "Return lightweight quotes (symbol, price, change, volume) instead of full quotes"
  )]
  pub short: Option<bool>,
}

impl CommodityArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.short {
      Some(short) => fmp::types::quotes::ShortParams::builder().short(short).build(),
      None => fmp::types::quotes::ShortParams::builder().build(),
    };
    let quotes = ctx.client.commodity_quotes(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch quotes for all cryptocurrencies tracked by FMP.
///
/// Covers hundreds of crypto pairs priced in USD and other currencies.
/// Symbols follow the pattern BTCUSD, ETHUSD, etc.
///
/// Example:
///   fmp quotes crypto
///   fmp quotes crypto --short true
#[derive(Args, Debug, Clone)]
pub struct CryptoArgs {
  #[arg(
    long,
    help = "Return lightweight quotes (symbol, price, change, volume) instead of full quotes"
  )]
  pub short: Option<bool>,
}

impl CryptoArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.short {
      Some(short) => fmp::types::quotes::ShortParams::builder().short(short).build(),
      None => fmp::types::quotes::ShortParams::builder().build(),
    };
    let quotes = ctx.client.crypto_quotes(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch quotes for all forex currency pairs tracked by FMP.
///
/// Covers major, minor, and exotic FX pairs. Symbols follow the pattern
/// EURUSD, GBPUSD, USDJPY, etc. Prices represent the exchange rate of
/// the base currency in terms of the quote currency.
///
/// Example:
///   fmp quotes forex
///   fmp quotes forex --short true
#[derive(Args, Debug, Clone)]
pub struct ForexArgs {
  #[arg(
    long,
    help = "Return lightweight quotes (symbol, price, change, volume) instead of full quotes"
  )]
  pub short: Option<bool>,
}

impl ForexArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.short {
      Some(short) => fmp::types::quotes::ShortParams::builder().short(short).build(),
      None => fmp::types::quotes::ShortParams::builder().build(),
    };
    let quotes = ctx.client.forex_quotes(params).await?;
    crate::output::output_json(&quotes)
  }
}

/// Fetch quotes for all market indexes tracked by FMP.
///
/// Covers major global equity indexes (S&P 500, NASDAQ Composite, Dow Jones,
/// FTSE 100, DAX, Nikkei 225, etc.) and sector indexes. Symbols typically
/// start with ^ (e.g., ^GSPC for S&P 500, ^IXIC for NASDAQ).
///
/// Example:
///   fmp quotes index
///   fmp quotes index --short true
#[derive(Args, Debug, Clone)]
pub struct IndexArgs {
  #[arg(
    long,
    help = "Return lightweight quotes (symbol, price, change, volume) instead of full quotes"
  )]
  pub short: Option<bool>,
}

impl IndexArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.short {
      Some(short) => fmp::types::quotes::ShortParams::builder().short(short).build(),
      None => fmp::types::quotes::ShortParams::builder().build(),
    };
    let quotes = ctx.client.index_quotes(params).await?;
    crate::output::output_json(&quotes)
  }
}
