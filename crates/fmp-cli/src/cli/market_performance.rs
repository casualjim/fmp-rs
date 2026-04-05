use clap::{Args, Subcommand};
use eyre::Result;
use fmp::MarketPerformanceApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum MarketPerformanceArgs {
  /// Sector performance snapshot for all sectors on a given date
  SectorPerformanceSnapshot(SectorPerformanceSnapshotArgs),
  /// Industry performance snapshot for all industries on a given date
  IndustryPerformanceSnapshot(IndustryPerformanceSnapshotArgs),
  /// Historical daily performance for a specific market sector
  HistoricalSectorPerformance(HistoricalSectorPerformanceArgs),
  /// Historical daily performance for a specific market industry
  HistoricalIndustryPerformance(HistoricalIndustryPerformanceArgs),
  /// Sector P/E ratio snapshot for all sectors on a given date
  SectorPeSnapshot(SectorPeSnapshotArgs),
  /// Industry P/E ratio snapshot for all industries on a given date
  IndustryPeSnapshot(IndustryPeSnapshotArgs),
  /// Historical P/E ratios for a specific sector over time
  HistoricalSectorPe(HistoricalSectorPeArgs),
  /// Historical P/E ratios for a specific industry over time
  HistoricalIndustryPe(HistoricalIndustryPeArgs),
  /// Top stocks by price gain percentage today
  BiggestGainers(BiggestGainersArgs),
  /// Top stocks by price loss percentage today
  BiggestLosers(BiggestLosersArgs),
  /// Most actively traded stocks by volume today
  MostActives(MostActivesArgs),
}

impl MarketPerformanceArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::SectorPerformanceSnapshot(args) => args.handle(ctx).await,
      Self::IndustryPerformanceSnapshot(args) => args.handle(ctx).await,
      Self::HistoricalSectorPerformance(args) => args.handle(ctx).await,
      Self::HistoricalIndustryPerformance(args) => args.handle(ctx).await,
      Self::SectorPeSnapshot(args) => args.handle(ctx).await,
      Self::IndustryPeSnapshot(args) => args.handle(ctx).await,
      Self::HistoricalSectorPe(args) => args.handle(ctx).await,
      Self::HistoricalIndustryPe(args) => args.handle(ctx).await,
      Self::BiggestGainers(args) => args.handle(ctx).await,
      Self::BiggestLosers(args) => args.handle(ctx).await,
      Self::MostActives(args) => args.handle(ctx).await,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct SectorPerformanceSnapshotArgs {
  #[arg(long, required = true, help = "Date in YYYY-MM-DD format")]
  pub date: String,

  #[arg(long, help = "Filter by exchange (e.g., NYSE, NASDAQ)")]
  pub exchange: Option<String>,

  #[arg(long, help = "Filter by sector name (e.g., Technology, Healthcare)")]
  pub sector: Option<String>,
}

impl SectorPerformanceSnapshotArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let date = self.date.parse::<jiff::civil::Date>()?;
    let params = match (&self.exchange, &self.sector) {
      (Some(exchange), Some(sector)) => fmp::types::market_performance::SectorSnapshotParams::builder()
        .date(date)
        .exchange(exchange)
        .sector(sector)
        .build(),
      (Some(exchange), None) => fmp::types::market_performance::SectorSnapshotParams::builder()
        .date(date)
        .exchange(exchange)
        .build(),
      (None, Some(sector)) => fmp::types::market_performance::SectorSnapshotParams::builder()
        .date(date)
        .sector(sector)
        .build(),
      (None, None) => fmp::types::market_performance::SectorSnapshotParams::builder()
        .date(date)
        .build(),
    };
    let data = ctx.client.sector_performance_snapshot(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct IndustryPerformanceSnapshotArgs {
  #[arg(long, required = true, help = "Date in YYYY-MM-DD format")]
  pub date: String,

  #[arg(long, help = "Filter by exchange (e.g., NYSE, NASDAQ)")]
  pub exchange: Option<String>,

  #[arg(long, help = "Filter by industry name")]
  pub industry: Option<String>,
}

impl IndustryPerformanceSnapshotArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let date = self.date.parse::<jiff::civil::Date>()?;
    let params = match (&self.exchange, &self.industry) {
      (Some(exchange), Some(industry)) => fmp::types::market_performance::IndustrySnapshotParams::builder()
        .date(date)
        .exchange(exchange)
        .industry(industry)
        .build(),
      (Some(exchange), None) => fmp::types::market_performance::IndustrySnapshotParams::builder()
        .date(date)
        .exchange(exchange)
        .build(),
      (None, Some(industry)) => fmp::types::market_performance::IndustrySnapshotParams::builder()
        .date(date)
        .industry(industry)
        .build(),
      (None, None) => fmp::types::market_performance::IndustrySnapshotParams::builder()
        .date(date)
        .build(),
    };
    let data = ctx.client.industry_performance_snapshot(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct HistoricalSectorPerformanceArgs {
  #[arg(
    long,
    required = true,
    help = "Sector name (e.g., Technology, Healthcare, Financials)"
  )]
  pub sector: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Filter by exchange (e.g., NYSE, NASDAQ)")]
  pub exchange: Option<String>,
}

impl HistoricalSectorPerformanceArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to, &self.exchange) {
      (Some(from), Some(to), Some(exchange)) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (Some(from), Some(to), None) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None, Some(exchange)) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .from(from.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (Some(from), None, None) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to), Some(exchange)) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .to(to.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (None, Some(to), None) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None, Some(exchange)) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .exchange(exchange)
        .build(),
      (None, None, None) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .build(),
    };
    let data = ctx.client.historical_sector_performance(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct HistoricalIndustryPerformanceArgs {
  #[arg(long, required = true, help = "Industry name")]
  pub industry: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Filter by exchange (e.g., NYSE, NASDAQ)")]
  pub exchange: Option<String>,
}

impl HistoricalIndustryPerformanceArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to, &self.exchange) {
      (Some(from), Some(to), Some(exchange)) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (Some(from), Some(to), None) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None, Some(exchange)) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .from(from.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (Some(from), None, None) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to), Some(exchange)) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .to(to.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (None, Some(to), None) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None, Some(exchange)) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .exchange(exchange)
        .build(),
      (None, None, None) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .build(),
    };
    let data = ctx.client.historical_industry_performance(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct SectorPeSnapshotArgs {
  #[arg(long, required = true, help = "Date in YYYY-MM-DD format")]
  pub date: String,

  #[arg(long, help = "Filter by exchange (e.g., NYSE, NASDAQ)")]
  pub exchange: Option<String>,

  #[arg(long, help = "Filter by sector name")]
  pub sector: Option<String>,
}

impl SectorPeSnapshotArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let date = self.date.parse::<jiff::civil::Date>()?;
    let params = match (&self.exchange, &self.sector) {
      (Some(exchange), Some(sector)) => fmp::types::market_performance::SectorSnapshotParams::builder()
        .date(date)
        .exchange(exchange)
        .sector(sector)
        .build(),
      (Some(exchange), None) => fmp::types::market_performance::SectorSnapshotParams::builder()
        .date(date)
        .exchange(exchange)
        .build(),
      (None, Some(sector)) => fmp::types::market_performance::SectorSnapshotParams::builder()
        .date(date)
        .sector(sector)
        .build(),
      (None, None) => fmp::types::market_performance::SectorSnapshotParams::builder()
        .date(date)
        .build(),
    };
    let data = ctx.client.sector_pe_snapshot(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct IndustryPeSnapshotArgs {
  #[arg(long, required = true, help = "Date in YYYY-MM-DD format")]
  pub date: String,

  #[arg(long, help = "Filter by exchange (e.g., NYSE, NASDAQ)")]
  pub exchange: Option<String>,

  #[arg(long, help = "Filter by industry name")]
  pub industry: Option<String>,
}

impl IndustryPeSnapshotArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let date = self.date.parse::<jiff::civil::Date>()?;
    let params = match (&self.exchange, &self.industry) {
      (Some(exchange), Some(industry)) => fmp::types::market_performance::IndustrySnapshotParams::builder()
        .date(date)
        .exchange(exchange)
        .industry(industry)
        .build(),
      (Some(exchange), None) => fmp::types::market_performance::IndustrySnapshotParams::builder()
        .date(date)
        .exchange(exchange)
        .build(),
      (None, Some(industry)) => fmp::types::market_performance::IndustrySnapshotParams::builder()
        .date(date)
        .industry(industry)
        .build(),
      (None, None) => fmp::types::market_performance::IndustrySnapshotParams::builder()
        .date(date)
        .build(),
    };
    let data = ctx.client.industry_pe_snapshot(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct HistoricalSectorPeArgs {
  #[arg(long, required = true, help = "Sector name")]
  pub sector: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Filter by exchange")]
  pub exchange: Option<String>,
}

impl HistoricalSectorPeArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to, &self.exchange) {
      (Some(from), Some(to), Some(exchange)) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (Some(from), Some(to), None) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None, Some(exchange)) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .from(from.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (Some(from), None, None) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to), Some(exchange)) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .to(to.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (None, Some(to), None) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None, Some(exchange)) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .exchange(exchange)
        .build(),
      (None, None, None) => fmp::types::market_performance::SectorHistoryParams::builder()
        .sector(&self.sector)
        .build(),
    };
    let data = ctx.client.historical_sector_pe(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct HistoricalIndustryPeArgs {
  #[arg(long, required = true, help = "Industry name")]
  pub industry: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Filter by exchange")]
  pub exchange: Option<String>,
}

impl HistoricalIndustryPeArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.from, &self.to, &self.exchange) {
      (Some(from), Some(to), Some(exchange)) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (Some(from), Some(to), None) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .from(from.parse::<jiff::civil::Date>()?)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (Some(from), None, Some(exchange)) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .from(from.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (Some(from), None, None) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .from(from.parse::<jiff::civil::Date>()?)
        .build(),
      (None, Some(to), Some(exchange)) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .to(to.parse::<jiff::civil::Date>()?)
        .exchange(exchange)
        .build(),
      (None, Some(to), None) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .to(to.parse::<jiff::civil::Date>()?)
        .build(),
      (None, None, Some(exchange)) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .exchange(exchange)
        .build(),
      (None, None, None) => fmp::types::market_performance::IndustryHistoryParams::builder()
        .industry(&self.industry)
        .build(),
    };
    let data = ctx.client.historical_industry_pe(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct BiggestGainersArgs;

impl BiggestGainersArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let data = ctx.client.biggest_gainers(()).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct BiggestLosersArgs;

impl BiggestLosersArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let data = ctx.client.biggest_losers(()).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct MostActivesArgs;

impl MostActivesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let data = ctx.client.most_actives(()).await?;
    crate::output::output_json(&data)
  }
}
