use clap::{Args, Subcommand};
use eyre::Result;
use fmp::IndexesApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum IndexesArgs {
  /// List all available market indexes (optionally filter by symbol)
  List(ListArgs),
  /// Full quote for a market index (value, change, volume)
  Quote(QuoteArgs),
  /// Lightweight quote for a market index
  QuoteShort(QuoteShortArgs),
  /// Batch quotes for all market indexes (optionally as lightweight format)
  QuoteBatch(QuoteBatchArgs),
  /// End-of-day index value history lightweight format with optional date range
  EodLight(EodLightArgs),
  /// End-of-day index value history full format with optional date range
  EodFull(EodFullArgs),
  /// 1-minute intraday index value bars
  Intraday1min(Intraday1minArgs),
  /// 5-minute intraday index value bars
  Intraday5min(Intraday5minArgs),
  /// 1-hour intraday index value bars
  Intraday1hour(Intraday1hourArgs),
}

impl IndexesArgs {
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
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct ListArgs {
  #[arg(long, help = "Filter by index symbol (e.g., SPX, DJI, NDX)")]
  pub symbol: Option<String>,
}

impl ListArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match &self.symbol {
      Some(symbol) => fmp::types::indexes::IndexSymbolParams::builder().symbol(symbol).build(),
      None => fmp::types::indexes::IndexSymbolParams::builder().symbol("").build(),
    };
    let data = ctx.client.index_list(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct QuoteArgs {
  #[arg(
    long,
    required = true,
    help = "Index symbol (e.g., SPX for S&P 500, DJI for Dow Jones)"
  )]
  pub symbol: String,
}

impl QuoteArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::indexes::IndexSymbolParams::builder()
      .symbol(&self.symbol)
      .build();
    let data = ctx.client.quote(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct QuoteShortArgs {
  #[arg(long, required = true, help = "Index symbol (e.g., SPX)")]
  pub symbol: String,
}

impl QuoteShortArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::indexes::IndexSymbolParams::builder()
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
      Some(short) => fmp::types::indexes::IndexBatchParams::builder().short(short).build(),
      None => fmp::types::indexes::IndexBatchParams::builder().build(),
    };
    let data = ctx.client.batch_index_quotes(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EodLightArgs {
  #[arg(long, required = true, help = "Index symbol (e.g., SPX)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl EodLightArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_price_eod_light(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EodFullArgs {
  #[arg(long, required = true, help = "Index symbol (e.g., SPX)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl EodFullArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_price_eod_full(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday1minArgs {
  #[arg(long, required = true, help = "Index symbol (e.g., SPX)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl Intraday1minArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_chart_1min(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday5minArgs {
  #[arg(long, required = true, help = "Index symbol (e.g., SPX)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl Intraday5minArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_chart_5min(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday1hourArgs {
  #[arg(long, required = true, help = "Index symbol (e.g., SPX)")]
  pub symbol: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,
}

impl Intraday1hourArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to) {
      (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.historical_chart_1hour(params).await?;
    crate::output::output_json(&data)
  }
}
