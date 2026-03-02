use clap::{Args, Subcommand};
use eyre::Result;
use fmp::EconomicsApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum EconomicsArgs {
    /// Economic indicator time series (GDP, CPI, unemployment, etc.)
    Indicators(IndicatorsArgs),
    /// US Treasury yield curve rates (3-month to 30-year)
    TreasuryRates(TreasuryRatesArgs),
    /// Federal funds effective rate historical data
    FederalFundRate(FederalFundRateArgs),
    /// Upcoming economic calendar events (central bank meetings, data releases)
    CalendarEvents(CalendarEventsArgs),
    /// Market risk premium by country
    MarketRiskPremium(MarketRiskPremiumArgs),
}

impl EconomicsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Indicators(args) => args.handle(ctx).await,
            Self::TreasuryRates(args) => args.handle(ctx).await,
            Self::FederalFundRate(args) => args.handle(ctx).await,
            Self::CalendarEvents(args) => args.handle(ctx).await,
            Self::MarketRiskPremium(args) => args.handle(ctx).await,
        }
    }
}

fn parse_date_range(
    from: Option<&str>,
    to: Option<&str>,
) -> Result<fmp::types::economics::DateRangeParams> {
    Ok(fmp::types::economics::DateRangeParams {
        from: from.map(str::parse::<jiff::civil::Date>).transpose()?,
        to: to.map(str::parse::<jiff::civil::Date>).transpose()?,
    })
}

#[derive(Args, Debug, Clone)]
pub struct IndicatorsArgs {
    #[arg(long, required = true, help = "Indicator name (e.g., GDP, CPI, unemploymentRate, retailSales)")]
    pub name: String,

    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl IndicatorsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::economics::EconomicIndicatorParams {
            name: self.name.clone(),
            from: self.from.as_deref().map(str::parse::<jiff::civil::Date>).transpose()?,
            to: self.to.as_deref().map(str::parse::<jiff::civil::Date>).transpose()?,
        };
        let data = ctx.client.economic_indicator(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct TreasuryRatesArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl TreasuryRatesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = parse_date_range(self.from.as_deref(), self.to.as_deref())?;
        let data = ctx.client.treasury_rates(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct FederalFundRateArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl FederalFundRateArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::economics::EconomicIndicatorParams {
            name: "federalFundsEffectiveRate".to_string(),
            from: self.from.as_deref().map(str::parse::<jiff::civil::Date>).transpose()?,
            to: self.to.as_deref().map(str::parse::<jiff::civil::Date>).transpose()?,
        };
        let data = ctx.client.economic_indicator(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CalendarEventsArgs {
    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl CalendarEventsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = parse_date_range(self.from.as_deref(), self.to.as_deref())?;
        let data = ctx.client.economic_calendar(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct MarketRiskPremiumArgs;

impl MarketRiskPremiumArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.market_risk_premium(()).await?;
        crate::output::output_json(&data)
    }
}
