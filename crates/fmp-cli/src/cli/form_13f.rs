use clap::{Args, Subcommand};
use eyre::Result;
use fmp::Form13FApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum Form13FArgs {
    /// Latest 13F institutional ownership filings (paginated)
    Latest(LatestArgs),
    /// Extract holdings from a specific 13F filing by CIK, year, and quarter
    Extract(ExtractArgs),
    /// Available 13F filing dates for a given CIK
    Dates(DatesArgs),
    /// Institutional holder analytics for a symbol by year and quarter
    HolderAnalytics(HolderAnalyticsArgs),
    /// Performance summary for an institutional holder by CIK
    HolderPerformance(HolderPerformanceArgs),
    /// Industry breakdown for an institutional holder by CIK, year, and quarter
    HolderIndustry(HolderIndustryArgs),
    /// Positions summary for a symbol across all 13F filers by year and quarter
    SymbolPositions(SymbolPositionsArgs),
    /// Industry performance summary across all 13F filers by year and quarter
    IndustrySummary(IndustrySummaryArgs),
}

impl Form13FArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Latest(args) => args.handle(ctx).await,
            Self::Extract(args) => args.handle(ctx).await,
            Self::Dates(args) => args.handle(ctx).await,
            Self::HolderAnalytics(args) => args.handle(ctx).await,
            Self::HolderPerformance(args) => args.handle(ctx).await,
            Self::HolderIndustry(args) => args.handle(ctx).await,
            Self::SymbolPositions(args) => args.handle(ctx).await,
            Self::IndustrySummary(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct LatestArgs {
    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl LatestArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::form_13f::PaginationParams {
            page: self.page,
            limit: self.limit,
        };
        let data = ctx.client.institutional_ownership_latest(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ExtractArgs {
    #[arg(long, required = true, help = "SEC CIK number of the institutional filer")]
    pub cik: String,

    #[arg(long, required = true, help = "Filing year")]
    pub year: i32,

    #[arg(long, required = true, help = "Filing quarter (1-4)")]
    pub quarter: i32,
}

impl ExtractArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::form_13f::FilingExtractParams {
            cik: self.cik.clone(),
            year: self.year,
            quarter: self.quarter,
        };
        let data = ctx.client.institutional_ownership_extract(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct DatesArgs {
    #[arg(long, required = true, help = "SEC CIK number of the institutional filer")]
    pub cik: String,
}

impl DatesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::form_13f::FilingDatesParams {
            cik: self.cik.clone(),
        };
        let data = ctx.client.institutional_ownership_dates(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct HolderAnalyticsArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, required = true, help = "Filing year")]
    pub year: i32,

    #[arg(long, required = true, help = "Filing quarter (1-4)")]
    pub quarter: i32,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl HolderAnalyticsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.page, &self.limit) {
            (Some(page), Some(limit)) => fmp::types::form_13f::FilingExtractAnalyticsByHolderParams::builder()
                .symbol(&self.symbol)
                .year(self.year)
                .quarter(self.quarter)
                .page(*page)
                .limit(*limit)
                .build(),
            (Some(page), None) => fmp::types::form_13f::FilingExtractAnalyticsByHolderParams::builder()
                .symbol(&self.symbol)
                .year(self.year)
                .quarter(self.quarter)
                .page(*page)
                .build(),
            (None, Some(limit)) => fmp::types::form_13f::FilingExtractAnalyticsByHolderParams::builder()
                .symbol(&self.symbol)
                .year(self.year)
                .quarter(self.quarter)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::form_13f::FilingExtractAnalyticsByHolderParams::builder()
                .symbol(&self.symbol)
                .year(self.year)
                .quarter(self.quarter)
                .build(),
        };
        let data = ctx
            .client
            .institutional_ownership_extract_analytics_holder(params)
            .await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct HolderPerformanceArgs {
    #[arg(long, required = true, help = "SEC CIK number of the institutional filer")]
    pub cik: String,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,
}

impl HolderPerformanceArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.page {
            Some(page) => fmp::types::form_13f::HolderPerformanceParams::builder()
                .cik(&self.cik)
                .page(page)
                .build(),
            None => fmp::types::form_13f::HolderPerformanceParams::builder()
                .cik(&self.cik)
                .build(),
        };
        let data = ctx
            .client
            .institutional_ownership_holder_performance_summary(params)
            .await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct HolderIndustryArgs {
    #[arg(long, required = true, help = "SEC CIK number of the institutional filer")]
    pub cik: String,

    #[arg(long, required = true, help = "Filing year")]
    pub year: i32,

    #[arg(long, required = true, help = "Filing quarter (1-4)")]
    pub quarter: i32,
}

impl HolderIndustryArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::form_13f::HolderIndustryBreakdownParams {
            cik: self.cik.clone(),
            year: self.year,
            quarter: self.quarter,
        };
        let data = ctx
            .client
            .institutional_ownership_holder_industry_breakdown(params)
            .await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct SymbolPositionsArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, required = true, help = "Filing year")]
    pub year: i32,

    #[arg(long, required = true, help = "Filing quarter (1-4)")]
    pub quarter: i32,
}

impl SymbolPositionsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::form_13f::PositionsSummaryParams {
            symbol: self.symbol.clone(),
            year: self.year,
            quarter: self.quarter,
        };
        let data = ctx
            .client
            .institutional_ownership_symbol_positions_summary(params)
            .await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct IndustrySummaryArgs {
    #[arg(long, required = true, help = "Filing year")]
    pub year: i32,

    #[arg(long, required = true, help = "Filing quarter (1-4)")]
    pub quarter: i32,
}

impl IndustrySummaryArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::form_13f::IndustryPerformanceParams {
            year: self.year,
            quarter: self.quarter,
        };
        let data = ctx
            .client
            .institutional_ownership_industry_summary(params)
            .await?;
        crate::output::output_json(&data)
    }
}
