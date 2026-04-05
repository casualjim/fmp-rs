use clap::{Args, Subcommand};
use eyre::Result;
use fmp::AnalystApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum AnalystArgs {
  /// Current analyst buy/sell/hold ratings for a symbol
  Ratings(RatingsArgs),
  /// Historical analyst rating changes over time
  RatingsHistorical(RatingsHistoricalArgs),
  /// Analyst price target consensus and individual targets
  PriceTarget(PriceTargetArgs),
  /// Price target summary (average, high, low, consensus count)
  PriceTargetSummary(PriceTargetSummaryArgs),
  /// Analyst EPS and revenue estimates by period
  Estimates(EstimatesArgs),
  /// Analyst grade changes (e.g., upgrade, downgrade, initiation)
  Grades(GradesArgs),
  /// Analyst grades consensus summary
  GradesConsensus(GradesConsensusArgs),
  /// Price target news articles for a symbol
  PriceTargetNews(PriceTargetNewsArgs),
  /// Latest price target news across all symbols (paginated)
  PriceTargetLatestNews(PriceTargetLatestNewsArgs),
  /// Grade change news articles for a symbol
  GradesNews(GradesNewsArgs),
  /// Latest grade change news across all symbols (paginated)
  GradesLatestNews(GradesLatestNewsArgs),
  /// Historical analyst grades buy/hold/sell counts for a symbol
  GradesHistorical(GradesHistoricalArgs),
}

impl AnalystArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::Ratings(args) => args.handle(ctx).await,
      Self::RatingsHistorical(args) => args.handle(ctx).await,
      Self::PriceTarget(args) => args.handle(ctx).await,
      Self::PriceTargetSummary(args) => args.handle(ctx).await,
      Self::Estimates(args) => args.handle(ctx).await,
      Self::Grades(args) => args.handle(ctx).await,
      Self::GradesConsensus(args) => args.handle(ctx).await,
      Self::PriceTargetNews(args) => args.handle(ctx).await,
      Self::PriceTargetLatestNews(args) => args.handle(ctx).await,
      Self::GradesNews(args) => args.handle(ctx).await,
      Self::GradesLatestNews(args) => args.handle(ctx).await,
      Self::GradesHistorical(args) => args.handle(ctx).await,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct RatingsArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl RatingsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolLimitParams {
      symbol: self.symbol.clone(),
      limit: self.limit,
    };
    let data = ctx.client.ratings_snapshot(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct RatingsHistoricalArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl RatingsHistoricalArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolLimitParams {
      symbol: self.symbol.clone(),
      limit: self.limit,
    };
    let data = ctx.client.ratings_historical(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct PriceTargetArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,
}

impl PriceTargetArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolLimitParams {
      symbol: self.symbol.clone(),
      limit: None,
    };
    let data = ctx.client.price_target_consensus(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct PriceTargetSummaryArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl PriceTargetSummaryArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolLimitParams {
      symbol: self.symbol.clone(),
      limit: self.limit,
    };
    let data = ctx.client.price_target_summary(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EstimatesArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, required = true, help = "Period type: annual or quarterly")]
  pub period: String,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl EstimatesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::AnalystEstimatesParams {
      symbol: self.symbol.clone(),
      period: self.period.clone(),
      limit: self.limit,
      page: self.page,
    };
    let data = ctx.client.analyst_estimates(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct GradesArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl GradesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolLimitParams {
      symbol: self.symbol.clone(),
      limit: self.limit,
    };
    let data = ctx.client.grades(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct GradesConsensusArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl GradesConsensusArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolLimitParams {
      symbol: self.symbol.clone(),
      limit: self.limit,
    };
    let data = ctx.client.grades_consensus(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct PriceTargetNewsArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl PriceTargetNewsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolPageParams {
      symbol: self.symbol.clone(),
      page: self.page,
      limit: self.limit,
    };
    let data = ctx.client.price_target_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct PriceTargetLatestNewsArgs {
  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl PriceTargetLatestNewsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::PageParams {
      page: self.page,
      limit: self.limit,
    };
    let data = ctx.client.price_target_latest_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct GradesNewsArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl GradesNewsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolPageParams {
      symbol: self.symbol.clone(),
      page: self.page,
      limit: self.limit,
    };
    let data = ctx.client.grades_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct GradesLatestNewsArgs {
  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl GradesLatestNewsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::PageParams {
      page: self.page,
      limit: self.limit,
    };
    let data = ctx.client.grades_latest_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct GradesHistoricalArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl GradesHistoricalArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::analyst::SymbolLimitParams {
      symbol: self.symbol.clone(),
      limit: self.limit,
    };
    let data = ctx.client.grades_historical(params).await?;
    crate::output::output_json(&data)
  }
}
