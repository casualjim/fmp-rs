use clap::{Args, Subcommand};
use eyre::Result;
use fmp::CompanyApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum CompanyArgs {
    /// Full company profile: sector, industry, description, CEO, employees, website, financials summary
    Profile(ProfileArgs),
    /// Company profile lookup by SEC CIK number instead of ticker symbol
    ProfileCik(ProfileCikArgs),
    /// List of peer companies (similar sector/size)
    Peers(PeersArgs),
    /// Current market capitalization for a symbol
    MarketCap(MarketCapArgs),
    /// Current market capitalization for multiple symbols in one request
    MarketCapBatch(MarketCapBatchArgs),
    /// Historical daily market capitalization over a date range
    MarketCapHistory(MarketCapHistoryArgs),
    /// Key executives: CEO, CFO, CTO with titles and compensation
    Executives(ExecutivesArgs),
    /// Executive compensation data (governance/proxy statement)
    Compensation(CompensationArgs),
    /// Current employee headcount
    Employees(EmployeesArgs),
    /// Historical employee headcount over time
    EmployeesHistory(EmployeesHistoryArgs),
    /// Shares float (tradeable shares) for a symbol
    Float(FloatArgs),
    /// Shares float for all companies (paginated)
    FloatAll(FloatAllArgs),
    /// Latest mergers and acquisitions announcements
    MaLatest(MaLatestArgs),
    /// Search mergers and acquisitions by company name
    MaSearch(MaSearchArgs),
}

impl CompanyArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Profile(args) => args.handle(ctx).await,
            Self::ProfileCik(args) => args.handle(ctx).await,
            Self::Peers(args) => args.handle(ctx).await,
            Self::MarketCap(args) => args.handle(ctx).await,
            Self::MarketCapBatch(args) => args.handle(ctx).await,
            Self::MarketCapHistory(args) => args.handle(ctx).await,
            Self::Executives(args) => args.handle(ctx).await,
            Self::Compensation(args) => args.handle(ctx).await,
            Self::Employees(args) => args.handle(ctx).await,
            Self::EmployeesHistory(args) => args.handle(ctx).await,
            Self::Float(args) => args.handle(ctx).await,
            Self::FloatAll(args) => args.handle(ctx).await,
            Self::MaLatest(args) => args.handle(ctx).await,
            Self::MaSearch(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct ProfileArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl ProfileArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::SymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.profile(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ProfileCikArgs {
    #[arg(long, required = true, help = "SEC CIK number (e.g., 0000320193)")]
    pub cik: String,
}

impl ProfileCikArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::CikParams::builder()
            .cik(&self.cik)
            .build();
        let data = ctx.client.profile_cik(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct PeersArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl PeersArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::SymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.stock_peers(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct MarketCapArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl MarketCapArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::SymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.market_capitalization(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct MarketCapBatchArgs {
    #[arg(long, required = true, help = "Comma-separated symbols (e.g., \"AAPL,MSFT,GOOGL\")")]
    pub symbols: String,
}

impl MarketCapBatchArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::SymbolsParams::builder()
            .symbols(&self.symbols)
            .build();
        let data = ctx.client.market_capitalization_batch(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct MarketCapHistoryArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl MarketCapHistoryArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.limit, &self.from, &self.to) {
            (Some(limit), Some(from), Some(to)) => fmp::types::company::MarketCapHistoryParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(limit), Some(from), None) => fmp::types::company::MarketCapHistoryParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(limit), None, Some(to)) => fmp::types::company::MarketCapHistoryParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(limit), None, None) => fmp::types::company::MarketCapHistoryParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, Some(from), Some(to)) => fmp::types::company::MarketCapHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(from), None) => fmp::types::company::MarketCapHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None, Some(to)) => fmp::types::company::MarketCapHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None, None) => fmp::types::company::MarketCapHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.historical_market_capitalization(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ExecutivesArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Filter by active status (true/false)")]
    pub active: Option<String>,
}

impl ExecutivesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match &self.active {
            Some(active) => fmp::types::company::ExecutivesParams::builder()
                .symbol(&self.symbol)
                .active(active)
                .build(),
            None => fmp::types::company::ExecutivesParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.key_executives(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CompensationArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl CompensationArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::SymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.governance_executive_compensation(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EmployeesArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl EmployeesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.limit {
            Some(limit) => fmp::types::company::SymbolLimitParams::builder()
                .symbol(&self.symbol)
                .limit(limit)
                .build(),
            None => fmp::types::company::SymbolLimitParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.employee_count(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EmployeesHistoryArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl EmployeesHistoryArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.limit {
            Some(limit) => fmp::types::company::SymbolLimitParams::builder()
                .symbol(&self.symbol)
                .limit(limit)
                .build(),
            None => fmp::types::company::SymbolLimitParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.historical_employee_count(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct FloatArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl FloatArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::SymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.shares_float(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct FloatAllArgs {
    #[arg(long, help = "Page number for pagination (0-indexed)")]
    pub page: Option<u32>,

    #[arg(long, help = "Number of results per page")]
    pub limit: Option<u32>,
}

impl FloatAllArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.page, &self.limit) {
            (Some(page), Some(limit)) => fmp::types::company::ShareFloatAllParams::builder()
                .page(*page)
                .limit(*limit)
                .build(),
            (Some(page), None) => fmp::types::company::ShareFloatAllParams::builder()
                .page(*page)
                .build(),
            (None, Some(limit)) => fmp::types::company::ShareFloatAllParams::builder()
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::company::ShareFloatAllParams::builder().build(),
        };
        let data = ctx.client.shares_float_all(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct MaLatestArgs {
    #[arg(long, default_value = "0", help = "Page number for pagination (0-indexed)")]
    pub page: u32,
}

impl MaLatestArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::PaginationParams::builder()
            .page(self.page)
            .build();
        let data = ctx.client.mergers_acquisitions_latest(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct MaSearchArgs {
    #[arg(long, required = true, help = "Company name to search for")]
    pub name: String,
}

impl MaSearchArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::MnaSearchParams::builder()
            .name(&self.name)
            .build();
        let data = ctx.client.mergers_acquisitions_search(params).await?;
        crate::output::output_json(&data)
    }
}
