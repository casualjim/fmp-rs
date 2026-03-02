use clap::{Args, Subcommand};
use eyre::Result;
use fmp::CalendarApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum CalendarArgs {
    /// Earnings calendar: upcoming and past earnings announcements with EPS estimates
    Earnings(EarningsArgs),
    /// Confirmed earnings dates for a specific company
    EarningsConfirmed(EarningsConfirmedArgs),
    /// IPO calendar: upcoming initial public offerings
    Ipos(IposArgs),
    /// Stock split calendar: upcoming and historical splits
    Splits(SplitsArgs),
    /// Dividend calendar: upcoming and historical dividend payments
    Dividends(DividendsArgs),
}

impl CalendarArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Earnings(args) => args.handle(ctx).await,
            Self::EarningsConfirmed(args) => args.handle(ctx).await,
            Self::Ipos(args) => args.handle(ctx).await,
            Self::Splits(args) => args.handle(ctx).await,
            Self::Dividends(args) => args.handle(ctx).await,
        }
    }
}

fn parse_range_params(
    from: Option<&str>,
    to: Option<&str>,
) -> Result<fmp::types::calendar::CalendarRangeParams> {
    Ok(fmp::types::calendar::CalendarRangeParams {
        from: from.map(str::parse::<jiff::civil::Date>).transpose()?,
        to: to.map(str::parse::<jiff::civil::Date>).transpose()?,
    })
}

#[derive(Args, Debug, Clone)]
pub struct EarningsArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl EarningsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = parse_range_params(self.from.as_deref(), self.to.as_deref())?;
        let data = ctx.client.earnings_calendar(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EarningsConfirmedArgs {
    #[arg(long, required = true, help = "Ticker symbol to get confirmed earnings dates for")]
    pub symbol: String,

    #[arg(long)]
    pub limit: Option<u32>,
}

impl EarningsConfirmedArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::calendar::SymbolLimitParams {
            symbol: self.symbol.clone(),
            limit: self.limit,
        };
        let data = ctx.client.earnings(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct IposArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl IposArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = parse_range_params(self.from.as_deref(), self.to.as_deref())?;
        let data = ctx.client.ipos_calendar(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct SplitsArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl SplitsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = parse_range_params(self.from.as_deref(), self.to.as_deref())?;
        let data = ctx.client.splits_calendar(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct DividendsArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl DividendsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = parse_range_params(self.from.as_deref(), self.to.as_deref())?;
        let data = ctx.client.dividends_calendar(params).await?;
        crate::output::output_json(&data)
    }
}
