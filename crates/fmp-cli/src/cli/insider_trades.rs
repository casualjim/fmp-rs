use clap::{Args, Subcommand};
use eyre::Result;
use fmp::InsiderTradesApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum InsiderTradesArgs {
    /// Latest insider trading transactions across all companies
    Latest(LatestArgs),
    /// Search insider trading by symbol, CIK, or transaction type
    Search(SearchArgs),
    /// Insider trading transactions by reporting person name
    ByName(ByNameArgs),
    /// Transaction types filed for a specific company
    TransactionTypes(TransactionTypesArgs),
    /// Insider trading statistics by quarter for a company
    Statistics(StatisticsArgs),
    /// Beneficial ownership (5%+ holders) for a company
    BeneficialOwnership(BeneficialOwnershipArgs),
}

impl InsiderTradesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Latest(args) => args.handle(ctx).await,
            Self::Search(args) => args.handle(ctx).await,
            Self::ByName(args) => args.handle(ctx).await,
            Self::TransactionTypes(args) => args.handle(ctx).await,
            Self::Statistics(args) => args.handle(ctx).await,
            Self::BeneficialOwnership(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct LatestArgs {
    #[arg(long, help = "Filter by date (YYYY-MM-DD)")]
    pub date: Option<String>,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl LatestArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::insider_trades::InsiderLatestParams {
            date: self
                .date
                .as_deref()
                .map(str::parse::<jiff::civil::Date>)
                .transpose()?
                .map(fmp::FmpDate),
            page: self.page,
            limit: self.limit,
        };
        let data = ctx.client.insider_trading_latest(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    #[arg(long, help = "Ticker symbol to filter by")]
    pub symbol: Option<String>,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Filter by reporting person CIK")]
    pub reporting_cik: Option<String>,

    #[arg(long, help = "Filter by company CIK")]
    pub company_cik: Option<String>,

    #[arg(long, help = "Filter by transaction type (e.g., P-Purchase, S-Sale)")]
    pub transaction_type: Option<String>,
}

impl SearchArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::insider_trades::InsiderSearchParams {
            symbol: self.symbol.clone(),
            page: self.page,
            limit: self.limit,
            reporting_cik: self.reporting_cik.clone(),
            company_cik: self.company_cik.clone(),
            transaction_type: self.transaction_type.clone(),
        };
        let data = ctx.client.insider_trading_search(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ByNameArgs {
    #[arg(long, required = true, help = "Reporting person name to search for")]
    pub name: String,
}

impl ByNameArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::insider_trades::ReportingNameParams {
            name: self.name.clone(),
        };
        let data = ctx.client.insider_trading_reporting_name(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct TransactionTypesArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl TransactionTypesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::insider_trades::InsiderSymbolParams {
            symbol: self.symbol.clone(),
        };
        let data = ctx.client.insider_trading_transaction_type(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct StatisticsArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl StatisticsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::insider_trades::InsiderSymbolParams {
            symbol: self.symbol.clone(),
        };
        let data = ctx.client.insider_trading_statistics(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct BeneficialOwnershipArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl BeneficialOwnershipArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.limit {
            Some(limit) => fmp::types::insider_trades::AcquisitionOwnershipParams::builder()
                .symbol(&self.symbol)
                .limit(limit)
                .build(),
            None => fmp::types::insider_trades::AcquisitionOwnershipParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.acquisition_of_beneficial_ownership(params).await?;
        crate::output::output_json(&data)
    }
}
