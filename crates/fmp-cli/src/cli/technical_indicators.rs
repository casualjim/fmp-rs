use clap::{Args, Subcommand};
use eyre::Result;
use fmp::TechnicalIndicatorsApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum TechnicalIndicatorsArgs {
    /// Simple Moving Average (SMA)
    Sma(IndicatorArgs),
    /// Exponential Moving Average (EMA)
    Ema(IndicatorArgs),
    /// Weighted Moving Average (WMA)
    Wma(IndicatorArgs),
    /// Double Exponential Moving Average (DEMA)
    Dema(IndicatorArgs),
    /// Triple Exponential Moving Average (TEMA)
    Tema(IndicatorArgs),
    /// Relative Strength Index (RSI) momentum oscillator
    Rsi(IndicatorArgs),
    /// Standard Deviation of price (volatility indicator)
    StdDev(IndicatorArgs),
    /// Williams %R momentum indicator (overbought/oversold)
    Williams(IndicatorArgs),
    /// Average Directional Index (ADX) trend strength indicator
    Adx(IndicatorArgs),
}

impl TechnicalIndicatorsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Sma(args) => args.handle_sma(ctx).await,
            Self::Ema(args) => args.handle_ema(ctx).await,
            Self::Wma(args) => args.handle_wma(ctx).await,
            Self::Dema(args) => args.handle_dema(ctx).await,
            Self::Tema(args) => args.handle_tema(ctx).await,
            Self::Rsi(args) => args.handle_rsi(ctx).await,
            Self::StdDev(args) => args.handle_stddev(ctx).await,
            Self::Williams(args) => args.handle_williams(ctx).await,
            Self::Adx(args) => args.handle_adx(ctx).await,
        }
    }
}

/// Arguments shared by all technical indicator subcommands.
///
/// Timeframes: 1min, 5min, 15min, 30min, 1hour, 4hour, daily
///
/// Examples:
///   fmp technical-indicators sma --symbol AAPL --period 50 --timeframe daily
///   fmp technical-indicators rsi --symbol AAPL --period 14 --timeframe daily
///   fmp technical-indicators ema --symbol SPY  --period 20 --timeframe 1hour
#[derive(Args, Debug, Clone)]
pub struct IndicatorArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, default_value = "20", help = "Lookback period in bars (e.g., 14 for RSI, 20 or 50 for SMA/EMA)")]
    pub period: u32,

    #[arg(long, default_value = "daily", help = "Price timeframe: 1min, 5min, 15min, 30min, 1hour, 4hour, daily")]
    pub timeframe: String,

    #[arg(long, help = "Earliest datetime to return (ISO 8601, e.g., 2024-01-01T00:00:00Z)")]
    pub from: Option<String>,

    #[arg(long, help = "Latest datetime to return (ISO 8601, e.g., 2024-12-31T23:59:59Z)")]
    pub to: Option<String>,
}

impl IndicatorArgs {
    fn build_params(&self) -> Result<fmp::types::technical_indicators::TechnicalIndicatorParams> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::technical_indicators::TechnicalIndicatorParams::builder()
                .symbol(&self.symbol)
                .period_length(self.period)
                .timeframe(&self.timeframe)
                .from(from.parse::<jiff::Timestamp>()?)
                .to(to.parse::<jiff::Timestamp>()?)
                .build(),
            (Some(from), None) => fmp::types::technical_indicators::TechnicalIndicatorParams::builder()
                .symbol(&self.symbol)
                .period_length(self.period)
                .timeframe(&self.timeframe)
                .from(from.parse::<jiff::Timestamp>()?)
                .build(),
            (None, Some(to)) => fmp::types::technical_indicators::TechnicalIndicatorParams::builder()
                .symbol(&self.symbol)
                .period_length(self.period)
                .timeframe(&self.timeframe)
                .to(to.parse::<jiff::Timestamp>()?)
                .build(),
            (None, None) => fmp::types::technical_indicators::TechnicalIndicatorParams::builder()
                .symbol(&self.symbol)
                .period_length(self.period)
                .timeframe(&self.timeframe)
                .build(),
        };
        Ok(params)
    }

    pub async fn handle_sma(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.sma(self.build_params()?).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_ema(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.ema(self.build_params()?).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_wma(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.wma(self.build_params()?).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_dema(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.dema(self.build_params()?).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_tema(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.tema(self.build_params()?).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_rsi(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.rsi(self.build_params()?).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_stddev(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.standard_deviation(self.build_params()?).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_williams(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.williams(self.build_params()?).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_adx(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.adx(self.build_params()?).await?;
        crate::output::output_json(&data)
    }
}
