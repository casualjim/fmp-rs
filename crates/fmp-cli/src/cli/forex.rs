use clap::{Args, Subcommand};
use eyre::Result;
use fmp::{ForexApi, NewsApi};

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum ForexArgs {
  /// List all available forex currency pairs
  List(ListArgs),
  /// Full quote for a single forex pair (bid, ask, price, change, volume)
  Quote(QuoteArgs),
  /// Lightweight quote for a single forex pair (price and basic metrics)
  QuoteShort(QuoteShortArgs),
  /// Batch quotes for all forex pairs (optionally as lightweight format)
  QuoteBatch(QuoteBatchArgs),
  /// End-of-day exchange rate history lightweight format with optional date range
  EodLight(EodLightArgs),
  /// End-of-day exchange rate history full format with optional date range
  EodFull(EodFullArgs),
  /// 1-minute intraday rate bars for a forex pair
  Intraday1min(Intraday1minArgs),
  /// 5-minute intraday rate bars for a forex pair
  Intraday5min(Intraday5minArgs),
  /// 1-hour intraday rate bars for a forex pair
  Intraday1hour(Intraday1hourArgs),
  /// Latest forex market news
  News(NewsArgs),
}

impl ForexArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::List(args) => args.handle(ctx).await,
      Self::Quote(args) => args.handle(ctx).await,
      Self::QuoteShort(args) => args.handle(ctx).await,
      Self::QuoteBatch(args) => args.handle(ctx).await,
      Self::EodLight(args) => args.handle(ctx).await,
      Self::EodFull(args) => args.handle(ctx).await,
      Self::Intraday1min(args) => args.handle(ctx).await,
      Self::Intraday5min(args) => args.handle(ctx).await,
      Self::Intraday1hour(args) => args.handle(ctx).await,
      Self::News(args) => args.handle(ctx).await,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct ListArgs;

impl ListArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let data = ctx.client.forex_list(()).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct QuoteArgs {
  #[arg(long, required = true, help = "Forex pair symbol (e.g., EURUSD, GBPJPY)")]
  pub symbol: String,
}

impl QuoteArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::forex::ForexSymbolParams::builder()
      .symbol(&self.symbol)
      .build();
    let data = ctx.client.quote(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct QuoteShortArgs {
  #[arg(long, required = true, help = "Forex pair symbol (e.g., EURUSD)")]
  pub symbol: String,
}

impl QuoteShortArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::forex::ForexSymbolParams::builder()
      .symbol(&self.symbol)
      .build();
    let data = ctx.client.quote_short(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct QuoteBatchArgs {
  #[arg(long, help = "Return lightweight quotes instead of full quotes")]
  pub short: Option<bool>,
}

impl QuoteBatchArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.short {
      Some(short) => fmp::types::forex::ForexBatchParams::builder().short(short).build(),
      None => fmp::types::forex::ForexBatchParams::builder().build(),
    };
    let data = ctx.client.batch_forex_quotes(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EodLightArgs {
  #[arg(long, required = true, help = "Forex pair symbol (e.g., EURUSD)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl EodLightArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_price_eod_light(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EodFullArgs {
  #[arg(long, required = true, help = "Forex pair symbol (e.g., EURUSD)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl EodFullArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_price_eod_full(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday1minArgs {
  #[arg(long, required = true, help = "Forex pair symbol (e.g., EURUSD)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl Intraday1minArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_chart_1min(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday5minArgs {
  #[arg(long, required = true, help = "Forex pair symbol (e.g., EURUSD)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl Intraday5minArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_chart_5min(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday1hourArgs {
  #[arg(long, required = true, help = "Forex pair symbol (e.g., EURUSD)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl Intraday1hourArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::forex::ForexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_chart_1hour(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct NewsArgs {
  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl NewsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::news::NewsParams {
      from: self
        .from
        .as_ref()
        .map(|s| s.parse::<jiff::civil::Date>())
        .transpose()?
        .map(fmp::FmpDate),
      to: self
        .to
        .as_ref()
        .map(|s| s.parse::<jiff::civil::Date>())
        .transpose()?
        .map(fmp::FmpDate),
      limit: self.limit,
      page: self.page,
    };
    let data = ctx.client.forex_news(params).await?;
    crate::output::output_json(&data)
  }
}
