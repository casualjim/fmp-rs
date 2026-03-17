use clap::{Args, Subcommand};
use eyre::Result;
use fmp::SecFilingsApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum FilingsArgs {
    /// Latest 8-K current report filings (material events, earnings releases)
    Latest8k(Latest8kArgs),
    /// Latest annual and quarterly financial statement filings
    LatestFinancials(LatestFinancialsArgs),
    /// SEC filings filtered by form type (10-K, 10-Q, 8-K, DEF 14A, etc.)
    ByType(ByTypeArgs),
    /// All SEC filings for a specific company by ticker symbol
    BySymbol(BySymbolArgs),
    /// All SEC filings for a company by SEC CIK number
    ByCik(ByCikArgs),
    /// Search SEC filings by company name
    SearchByName(SearchByNameArgs),
    /// Search SEC filings by ticker symbol
    SearchBySymbol(SearchBySymbolArgs),
}

impl FilingsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Latest8k(args) => args.handle(ctx).await,
            Self::LatestFinancials(args) => args.handle(ctx).await,
            Self::ByType(args) => args.handle(ctx).await,
            Self::BySymbol(args) => args.handle(ctx).await,
            Self::ByCik(args) => args.handle(ctx).await,
            Self::SearchByName(args) => args.handle(ctx).await,
            Self::SearchBySymbol(args) => args.handle(ctx).await,
        }
    }
}

fn parse_date_range_params(
    from: Option<&str>,
    to: Option<&str>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<fmp::types::sec_filings::DateRangeParams> {
    Ok(fmp::types::sec_filings::DateRangeParams {
        from: from.map(str::parse::<jiff::civil::Date>).transpose()?.map(fmp::FmpDate),
        to: to.map(str::parse::<jiff::civil::Date>).transpose()?.map(fmp::FmpDate),
        limit,
        page,
    })
}

#[derive(Args, Debug, Clone)]
pub struct Latest8kArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,

    #[arg(long, help = "Maximum number of filings to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,
}

impl Latest8kArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = parse_date_range_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?;
        let data = ctx.client.latest_8k_filings(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct LatestFinancialsArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,

    #[arg(long, help = "Maximum number of filings to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,
}

impl LatestFinancialsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = parse_date_range_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?;
        let data = ctx.client.latest_financial_filings(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ByTypeArgs {
    #[arg(long, required = true, help = "SEC form type (e.g., 10-K, 10-Q, 8-K, DEF 14A, SC 13G)")]
    pub form_type: String,

    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,

    #[arg(long, help = "Maximum number of filings to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,
}

impl ByTypeArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::sec_filings::FormTypeParams {
            form_type: self.form_type.clone(),
            range: parse_date_range_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?,
        };
        let data = ctx.client.filings_by_form_type(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct BySymbolArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,

    #[arg(long, help = "Maximum number of filings to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,
}

impl BySymbolArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::sec_filings::SymbolParams {
            symbol: self.symbol.clone(),
            range: parse_date_range_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?,
        };
        let data = ctx.client.filings_by_symbol(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ByCikArgs {
    #[arg(long, required = true, help = "SEC CIK number (e.g., 0000320193)")]
    pub cik: String,

    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,

    #[arg(long, help = "Maximum number of filings to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,
}

impl ByCikArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::sec_filings::CikParams {
            cik: self.cik.clone(),
            range: parse_date_range_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?,
        };
        let data = ctx.client.filings_by_cik(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct SearchByNameArgs {
    #[arg(long, required = true, help = "Company name to search for")]
    pub name: String,
}

impl SearchByNameArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::sec_filings::CompanyNameSearchParams {
            company: self.name.clone(),
        };
        let data = ctx.client.company_search_by_name(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct SearchBySymbolArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl SearchBySymbolArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::sec_filings::CompanySymbolSearchParams {
            symbol: self.symbol.clone(),
        };
        let data = ctx.client.company_search_by_symbol(params).await?;
        crate::output::output_json(&data)
    }
}
