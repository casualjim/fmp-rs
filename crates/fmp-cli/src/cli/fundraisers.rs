use clap::{Args, Subcommand};
use eyre::Result;
use fmp::FundraisersApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum FundraisersArgs {
    /// Latest crowdfunding offering filings (paginated)
    CrowdfundingLatest(CrowdfundingLatestArgs),
    /// Search crowdfunding offerings by company name
    CrowdfundingSearch(CrowdfundingSearchArgs),
    /// Crowdfunding offerings for a specific company by CIK
    CrowdfundingByCik(CrowdfundingByCikArgs),
    /// Latest equity fundraising (Reg D/Reg A) filings (paginated)
    EquityLatest(EquityLatestArgs),
    /// Search equity fundraising offerings by company name
    EquitySearch(EquitySearchArgs),
    /// Equity fundraising offerings for a specific company by CIK
    EquityByCik(EquityByCikArgs),
}

impl FundraisersArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::CrowdfundingLatest(args) => args.handle(ctx).await,
            Self::CrowdfundingSearch(args) => args.handle(ctx).await,
            Self::CrowdfundingByCik(args) => args.handle(ctx).await,
            Self::EquityLatest(args) => args.handle(ctx).await,
            Self::EquitySearch(args) => args.handle(ctx).await,
            Self::EquityByCik(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct CrowdfundingLatestArgs {
    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl CrowdfundingLatestArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::fundraisers::CrowdfundingLatestParams {
            page: self.page,
            limit: self.limit,
        };
        let data = ctx.client.crowdfunding_offerings_latest(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CrowdfundingSearchArgs {
    #[arg(long, required = true, help = "Company name to search for")]
    pub name: String,
}

impl CrowdfundingSearchArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::fundraisers::CrowdfundingSearchParams {
            name: self.name.clone(),
        };
        let data = ctx.client.crowdfunding_offerings_search(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CrowdfundingByCikArgs {
    #[arg(long, required = true, help = "SEC CIK number of the company")]
    pub cik: String,
}

impl CrowdfundingByCikArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::fundraisers::CrowdfundingCikParams {
            cik: self.cik.clone(),
        };
        let data = ctx.client.crowdfunding_offerings(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EquityLatestArgs {
    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Filter by SEC CIK number")]
    pub cik: Option<String>,
}

impl EquityLatestArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::fundraisers::EquityLatestParams {
            page: self.page,
            limit: self.limit,
            cik: self.cik.clone(),
        };
        let data = ctx.client.fundraising_latest(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EquitySearchArgs {
    #[arg(long, required = true, help = "Company name to search for")]
    pub name: String,
}

impl EquitySearchArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::fundraisers::EquitySearchParams {
            name: self.name.clone(),
        };
        let data = ctx.client.fundraising_search(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EquityByCikArgs {
    #[arg(long, required = true, help = "SEC CIK number of the company")]
    pub cik: String,
}

impl EquityByCikArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::fundraisers::EquityCikParams {
            cik: self.cik.clone(),
        };
        let data = ctx.client.fundraising(params).await?;
        crate::output::output_json(&data)
    }
}
