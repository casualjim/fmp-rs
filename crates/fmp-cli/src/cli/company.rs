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
    /// SEC-registered notes and bonds issued by a company
    Notes(NotesArgs),
    /// Delisted companies (paginated)
    Delisted(DelistedArgs),
    /// Industry-average executive compensation benchmarks by year
    ExecutiveCompensationBenchmark(ExecutiveCompensationBenchmarkArgs),
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
            Self::Notes(args) => args.handle(ctx).await,
            Self::Delisted(args) => args.handle(ctx).await,
            Self::ExecutiveCompensationBenchmark(args) => args.handle(ctx).await,
        }
    }
}

/// Fetch the full company profile for a ticker symbol.
///
/// Returns sector, industry, CEO, description, employee count, website, exchange,
/// market cap, beta, 52-week range, dividend yield, IPO date, and global identifiers
/// (CIK, ISIN, CUSIP). Also includes flags for ETF, ADR, and fund status.
///
/// Example:
///   fmp company profile --symbol AAPL
///   fmp company profile --symbol MSFT
#[derive(Args, Debug, Clone)]
pub struct ProfileArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL, MSFT, GOOGL)")]
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

/// Fetch a company profile using the SEC CIK number instead of a ticker symbol.
///
/// Useful when you have a CIK from an SEC filing but not the trading symbol,
/// or for companies that trade under multiple symbols. Returns the same
/// profile data as the `profile` command.
///
/// Example:
///   fmp company profile-cik --cik 0000320193     # Apple Inc.
///   fmp company profile-cik --cik 0000789019     # Microsoft Corp.
#[derive(Args, Debug, Clone)]
pub struct ProfileCikArgs {
    #[arg(long, required = true, help = "SEC CIK number (e.g., 0000320193 for Apple)")]
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

/// Fetch peer companies in the same sector and market-cap tier.
///
/// Returns a list of comparable companies with their name, current price,
/// and market cap. Useful for relative valuation analysis and competitive
/// benchmarking.
///
/// Example:
///   fmp company peers --symbol AAPL
///   fmp company peers --symbol TSLA
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

/// Fetch the current market capitalisation for a symbol.
///
/// Returns the date and market cap (shares outstanding × price) in the
/// company's reporting currency. Updated daily after market close.
///
/// Example:
///   fmp company market-cap --symbol AAPL
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

/// Fetch current market capitalisation for multiple symbols in one request.
///
/// More efficient than calling `market-cap` repeatedly. Returns the same
/// date + market-cap data for each symbol in the list.
///
/// Example:
///   fmp company market-cap-batch --symbols "AAPL,MSFT,GOOGL,AMZN,META"
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

/// Fetch daily historical market capitalisation over a date range.
///
/// Returns a time series of market cap values (date + market cap) for the
/// symbol. Useful for tracking a company's growth trajectory or for building
/// historical index weights.
///
/// Examples:
///   fmp company market-cap-history --symbol AAPL
///   fmp company market-cap-history --symbol AAPL --from 2020-01-01 --to 2024-12-31
///   fmp company market-cap-history --symbol MSFT --limit 252   # ~1 trading year
#[derive(Args, Debug, Clone)]
pub struct MarketCapHistoryArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return (most recent first)")]
    pub limit: Option<u32>,

    #[arg(long, help = "Earliest date to include (YYYY-MM-DD, inclusive)")]
    pub from: Option<String>,

    #[arg(long, help = "Latest date to include (YYYY-MM-DD, inclusive)")]
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

/// Fetch key executives (officers and directors) for a company.
///
/// Returns each executive's title, name, annual compensation (where disclosed),
/// compensation currency, gender, and birth year. By default returns both
/// current and former executives; use `--active true` for current only.
///
/// Examples:
///   fmp company executives --symbol AAPL
///   fmp company executives --symbol AAPL --active true
#[derive(Args, Debug, Clone)]
pub struct ExecutivesArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Filter by active status: \"true\" for current executives, \"false\" for former")]
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

/// Fetch detailed executive compensation from SEC proxy (DEF 14A) filings.
///
/// Returns itemised compensation for each named executive officer (NEO):
/// base salary, cash bonus, stock awards, option awards, incentive plan
/// compensation, other compensation, and total. Data sourced directly
/// from SEC EDGAR proxy statement filings.
///
/// Example:
///   fmp company compensation --symbol AAPL
///   fmp company compensation --symbol MSFT
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

/// Fetch current full-time employee count from the latest SEC filing.
///
/// Employee counts are sourced from 10-K annual reports and proxy statements.
/// The most recent filing's count is returned first. Use `employees-history`
/// for a multi-year time series.
///
/// Example:
///   fmp company employees --symbol AAPL
///   fmp company employees --symbol AMZN --limit 1
#[derive(Args, Debug, Clone)]
pub struct EmployeesArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return (default: most recent)")]
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

/// Fetch historical employee headcount across multiple SEC filings.
///
/// Returns a time series of employee counts, one record per filing that
/// disclosed headcount. Useful for tracking workforce growth/reduction
/// trends over years.
///
/// Example:
///   fmp company employees-history --symbol AAPL
///   fmp company employees-history --symbol META --limit 10
#[derive(Args, Debug, Clone)]
pub struct EmployeesHistoryArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of historical records to return (most recent first)")]
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

/// Fetch share float data — the portion of shares available for public trading.
///
/// Returns free float percentage, float share count, and total shares
/// outstanding. Low float stocks (under ~20M shares) tend to be more
/// volatile. High insider/institutional ownership reduces the float.
///
/// Example:
///   fmp company float --symbol AAPL
///   fmp company float --symbol GME
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

/// Fetch the latest merger and acquisition announcements.
///
/// Returns recent M&A events sorted by announcement date, with acquiring
/// company, target company, transaction date, and a link to the SEC filing.
/// Use `--page` to paginate through older events.
///
/// Examples:
///   fmp company ma-latest
///   fmp company ma-latest --page 1
#[derive(Args, Debug, Clone)]
pub struct MaLatestArgs {
    #[arg(long, default_value = "0", help = "Zero-indexed page number for pagination (default: 0)")]
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

/// Search mergers and acquisitions by acquiring or target company name.
///
/// Returns all M&A events where the acquiring or target company name
/// contains the search string. Useful for researching a specific company's
/// acquisition history.
///
/// Examples:
///   fmp company ma-search --name "Apple"
///   fmp company ma-search --name "Microsoft"
///   fmp company ma-search --name "Goldman"
#[derive(Args, Debug, Clone)]
pub struct MaSearchArgs {
    #[arg(long, required = true, help = "Partial or full company name to search M&A events (acquiring or target)")]
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

#[derive(Args, Debug, Clone)]
pub struct NotesArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl NotesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::SymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.company_notes(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct DelistedArgs {
    #[arg(long, help = "Page number for pagination (0-indexed)")]
    pub page: Option<u32>,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl DelistedArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.page, &self.limit) {
            (Some(page), Some(limit)) => fmp::types::company::PaginationParams::builder()
                .page(*page)
                .limit(*limit)
                .build(),
            (Some(page), None) => fmp::types::company::PaginationParams::builder()
                .page(*page)
                .build(),
            (None, Some(limit)) => fmp::types::company::PaginationParams::builder()
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::company::PaginationParams::builder().build(),
        };
        let data = ctx.client.delisted_companies(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ExecutiveCompensationBenchmarkArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl ExecutiveCompensationBenchmarkArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::company::SymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.executive_compensation_benchmark(params).await?;
        crate::output::output_json(&data)
    }
}
