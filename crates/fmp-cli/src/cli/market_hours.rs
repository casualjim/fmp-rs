use clap::{Args, Subcommand};
use eyre::Result;
use fmp::MarketHoursApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum MarketHoursArgs {
    /// Trading hours for a specific exchange (open/close times, timezone)
    Exchange(ExchangeArgs),
    /// Market holidays for a specific exchange with optional date range
    Holidays(HolidaysArgs),
    /// Trading hours for all supported exchanges
    All(AllArgs),
}

impl MarketHoursArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Exchange(args) => args.handle(ctx).await,
            Self::Holidays(args) => args.handle(ctx).await,
            Self::All(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct ExchangeArgs {
    #[arg(long, required = true, help = "Exchange code (e.g., NYSE, NASDAQ, LSE, TSX)")]
    pub exchange: String,
}

impl ExchangeArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::market_hours::ExchangeParams::builder()
            .exchange(&self.exchange)
            .build();
        let data = ctx.client.exchange_market_hours(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct HolidaysArgs {
    #[arg(long, required = true, help = "Exchange code (e.g., NYSE, NASDAQ)")]
    pub exchange: String,

    #[arg(long, help = "Start date in YYYY-MM-DD format")]
    pub from: Option<String>,

    #[arg(long, help = "End date in YYYY-MM-DD format")]
    pub to: Option<String>,
}

impl HolidaysArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::market_hours::HolidaysByExchangeParams::builder()
                .exchange(&self.exchange)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::market_hours::HolidaysByExchangeParams::builder()
                .exchange(&self.exchange)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::market_hours::HolidaysByExchangeParams::builder()
                .exchange(&self.exchange)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::market_hours::HolidaysByExchangeParams::builder()
                .exchange(&self.exchange)
                .build(),
        };
        let data = ctx.client.holidays_by_exchange(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AllArgs;

impl AllArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.all_exchange_market_hours(()).await?;
        crate::output::output_json(&data)
    }
}
