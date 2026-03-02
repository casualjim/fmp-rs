use clap::{Args, Subcommand};
use eyre::Result;
use fmp::MarketPerformanceApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum MarketPerformanceArgs {
    SectorPerformanceSnapshot(SectorPerformanceSnapshotArgs),
    IndustryPerformanceSnapshot(IndustryPerformanceSnapshotArgs),
    HistoricalSectorPerformance(HistoricalSectorPerformanceArgs),
    HistoricalIndustryPerformance(HistoricalIndustryPerformanceArgs),
    SectorPeSnapshot(SectorPeSnapshotArgs),
    IndustryPeSnapshot(IndustryPeSnapshotArgs),
    HistoricalSectorPe(HistoricalSectorPeArgs),
    HistoricalIndustryPe(HistoricalIndustryPeArgs),
    BiggestGainers(BiggestGainersArgs),
    BiggestLosers(BiggestLosersArgs),
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
    #[arg(long, required = true)]
    pub date: String,
    
    #[arg(long)]
    pub exchange: Option<String>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub date: String,
    
    #[arg(long)]
    pub exchange: Option<String>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub sector: String,
    
    #[arg(long)]
    pub from: Option<String>,
    
    #[arg(long)]
    pub to: Option<String>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub industry: String,
    
    #[arg(long)]
    pub from: Option<String>,
    
    #[arg(long)]
    pub to: Option<String>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub date: String,
    
    #[arg(long)]
    pub exchange: Option<String>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub date: String,
    
    #[arg(long)]
    pub exchange: Option<String>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub sector: String,
    
    #[arg(long)]
    pub from: Option<String>,
    
    #[arg(long)]
    pub to: Option<String>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub industry: String,
    
    #[arg(long)]
    pub from: Option<String>,
    
    #[arg(long)]
    pub to: Option<String>,
    
    #[arg(long)]
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
