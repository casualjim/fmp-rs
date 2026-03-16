use clap::{Args, Subcommand};
use eyre::Result;
use fmp::{ChartApi, ChartIntervalApi, TechnicalIndicatorsApi};

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum ChartArgs {
    /// End-of-day price history (OHLCV) with split/dividend adjustments (lightweight)
    EodLight(EodLightArgs),
    /// End-of-day price history (OHLCV) with split/dividend adjustments (full data)
    EodFull(EodFullArgs),
    /// End-of-day price history without split adjustments (raw prices)
    EodNonSplit(EodNonSplitArgs),
    /// End-of-day price history adjusted for dividends only
    EodDividend(EodDividendArgs),
    /// Intraday OHLCV bars at a specified interval (1min, 5min, 15min, 30min, 1hour, 4hour)
    Intraday(IntradayArgs),
    /// Simple Moving Average (SMA) technical indicator
    Sma(TechnicalIndicatorArgs),
    /// Exponential Moving Average (EMA) technical indicator
    Ema(TechnicalIndicatorArgs),
    /// Weighted Moving Average (WMA) technical indicator
    Wma(TechnicalIndicatorArgs),
    /// Double Exponential Moving Average (DEMA) technical indicator
    Dema(TechnicalIndicatorArgs),
    /// Triple Exponential Moving Average (TEMA) technical indicator
    Tema(TechnicalIndicatorArgs),
    /// Relative Strength Index (RSI) momentum oscillator
    Rsi(TechnicalIndicatorArgs),
    /// Standard Deviation of price (volatility indicator)
    Stddev(TechnicalIndicatorArgs),
    /// Williams %R momentum indicator (overbought/oversold)
    Williams(TechnicalIndicatorArgs),
    /// Average Directional Index (ADX) trend strength indicator
    Adx(TechnicalIndicatorArgs),
}

impl ChartArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::EodLight(args) => args.handle(ctx).await,
            Self::EodFull(args) => args.handle(ctx).await,
            Self::EodNonSplit(args) => args.handle(ctx).await,
            Self::EodDividend(args) => args.handle(ctx).await,
            Self::Intraday(args) => args.handle(ctx).await,
            Self::Sma(args) => args.handle_sma(ctx).await,
            Self::Ema(args) => args.handle_ema(ctx).await,
            Self::Wma(args) => args.handle_wma(ctx).await,
            Self::Dema(args) => args.handle_dema(ctx).await,
            Self::Tema(args) => args.handle_tema(ctx).await,
            Self::Rsi(args) => args.handle_rsi(ctx).await,
            Self::Stddev(args) => args.handle_stddev(ctx).await,
            Self::Williams(args) => args.handle_williams(ctx).await,
            Self::Adx(args) => args.handle_adx(ctx).await,
        }
    }
}

/// Fetch lightweight end-of-day price history (close price + volume only).
///
/// Returns a time series of adjusted closing prices and volume. Prices are
/// adjusted for stock splits and dividends. Ideal when you only need closing
/// prices for charting or return calculations — smaller payload than eod-full.
///
/// Examples:
///   fmp chart eod-light --symbol AAPL
///   fmp chart eod-light --symbol AAPL --from 2023-01-01 --to 2023-12-31
#[derive(Args, Debug, Clone)]
pub struct EodLightArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Earliest date to return (YYYY-MM-DD, inclusive; defaults to 5 years ago)")]
    pub from: Option<String>,

    #[arg(long, help = "Latest date to return (YYYY-MM-DD, inclusive; defaults to today)")]
    pub to: Option<String>,
}

impl EodLightArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        
        let data = ctx.client.historical_price_eod_light(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch full end-of-day OHLCV price history with split and dividend adjustments.
///
/// Returns open, high, low, close, volume, dollar change, percent change, and
/// VWAP for each trading day. All prices are adjusted for stock splits and
/// dividend distributions, making this suitable for backtesting.
///
/// Examples:
///   fmp chart eod-full --symbol AAPL
///   fmp chart eod-full --symbol AAPL --from 2023-01-01 --to 2023-12-31
#[derive(Args, Debug, Clone)]
pub struct EodFullArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Earliest date to return (YYYY-MM-DD, inclusive; defaults to 5 years ago)")]
    pub from: Option<String>,

    #[arg(long, help = "Latest date to return (YYYY-MM-DD, inclusive; defaults to today)")]
    pub to: Option<String>,
}

impl EodFullArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        
        let data = ctx.client.historical_price_eod_full(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch end-of-day price history without split adjustments (raw traded prices).
///
/// Prices reflect the actual transaction prices at the time — no split factor
/// is applied. Dividend adjustments are also not applied. Use this when you
/// need true historical transaction prices rather than backadjusted data.
///
/// Examples:
///   fmp chart eod-non-split --symbol AAPL
///   fmp chart eod-non-split --symbol AAPL --from 2020-01-01
#[derive(Args, Debug, Clone)]
pub struct EodNonSplitArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Earliest date to return (YYYY-MM-DD, inclusive)")]
    pub from: Option<String>,

    #[arg(long, help = "Latest date to return (YYYY-MM-DD, inclusive)")]
    pub to: Option<String>,
}

impl EodNonSplitArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        
        let data = ctx.client.historical_price_eod_non_split_adjusted(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch end-of-day price history adjusted for dividends only (not splits).
///
/// Prices are adjusted for dividend distributions but not for stock splits.
/// This preserves the effect of splits in the raw price series while still
/// enabling accurate total-return calculations.
///
/// Examples:
///   fmp chart eod-dividend --symbol AAPL
///   fmp chart eod-dividend --symbol JNJ --from 2020-01-01
#[derive(Args, Debug, Clone)]
pub struct EodDividendArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Earliest date to return (YYYY-MM-DD, inclusive)")]
    pub from: Option<String>,

    #[arg(long, help = "Latest date to return (YYYY-MM-DD, inclusive)")]
    pub to: Option<String>,
}

impl EodDividendArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::chart::ChartHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        
        let data = ctx.client.historical_price_eod_dividend_adjusted(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch intraday OHLCV bars at a specified time interval.
///
/// Returns open, high, low, close, and volume for each bar. Available
/// intervals range from 1-minute (finest granularity) to 4-hour. Data
/// availability varies by plan — intraday history is typically limited
/// to a rolling window (e.g., 30–90 days for 1-minute data).
///
/// Intervals: 1min, 5min, 15min, 30min, 1hour, 4hour
///
/// Examples:
///   fmp chart intraday --symbol AAPL --interval 5min
///   fmp chart intraday --symbol AAPL --interval 1hour --from 2024-01-15 --to 2024-01-19
///   fmp chart intraday --symbol SPY  --interval 1min  --from 2024-06-03
#[derive(Args, Debug, Clone)]
pub struct IntradayArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, default_value = "1hour", help = "Bar interval: 1min, 5min, 15min, 30min, 1hour, 4hour")]
    pub interval: String,

    #[arg(long, help = "Earliest date to return (YYYY-MM-DD, inclusive)")]
    pub from: Option<String>,

    #[arg(long, help = "Latest date to return (YYYY-MM-DD, inclusive)")]
    pub to: Option<String>,
}

impl IntradayArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let interval = match self.interval.as_str() {
            "1min" => fmp::types::chart::Interval::I1Min,
            "5min" => fmp::types::chart::Interval::I5Min,
            "15min" => fmp::types::chart::Interval::I15Min,
            "30min" => fmp::types::chart::Interval::I30Min,
            "1hour" => fmp::types::chart::Interval::I1Hour,
            "4hour" => fmp::types::chart::Interval::I4Hour,
            _ => return Err(eyre::eyre!("Invalid interval. Use: 1min, 5min, 15min, 30min, 1hour, 4hour")),
        };

        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::chart::ChartIntradayParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::chart::ChartIntradayParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::chart::ChartIntradayParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::chart::ChartIntradayParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        
        let data = ctx.client.historical_chart_interval(interval, params).await?;
        crate::output::output_json(&data)
    }
}

/// Shared arguments for all technical indicator commands (SMA, EMA, RSI, etc.).
///
/// Each indicator returns a time series of (date, indicator_value) pairs
/// calculated over `period` bars on the chosen `timeframe`.
///
/// Common period values:
///   SMA/EMA: 20 (short), 50 (medium), 200 (long)
///   RSI:     14 (standard), 9 (fast)
///   ADX:     14 (standard)
///   Williams %R: 14 (standard)
///
/// Timeframes: 1min, 5min, 15min, 30min, 1hour, 4hour, daily
///
/// Examples:
///   fmp chart sma --symbol AAPL --period 50 --timeframe daily
///   fmp chart rsi --symbol AAPL --period 14 --from 2024-01-01T00:00:00Z
///   fmp chart ema --symbol SPY  --period 20 --timeframe 1hour
#[derive(Args, Debug, Clone)]
pub struct TechnicalIndicatorArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, default_value = "20", help = "Lookback period in bars (e.g., 14 for RSI, 20 or 50 for SMA/EMA, 14 for ADX)")]
    pub period: u32,

    #[arg(long, default_value = "daily", help = "Price timeframe: 1min, 5min, 15min, 30min, 1hour, 4hour, daily")]
    pub timeframe: String,

    #[arg(long, help = "Earliest datetime to return (ISO 8601 / RFC 3339, e.g., 2024-01-01T00:00:00Z)")]
    pub from: Option<String>,

    #[arg(long, help = "Latest datetime to return (ISO 8601 / RFC 3339, e.g., 2024-12-31T23:59:59Z)")]
    pub to: Option<String>,
}

impl TechnicalIndicatorArgs {
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
        let params = self.build_params()?;
        let data = ctx.client.sma(params).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_ema(&self, ctx: &Context) -> Result<()> {
        let params = self.build_params()?;
        let data = ctx.client.ema(params).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_wma(&self, ctx: &Context) -> Result<()> {
        let params = self.build_params()?;
        let data = ctx.client.wma(params).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_dema(&self, ctx: &Context) -> Result<()> {
        let params = self.build_params()?;
        let data = ctx.client.dema(params).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_tema(&self, ctx: &Context) -> Result<()> {
        let params = self.build_params()?;
        let data = ctx.client.tema(params).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_rsi(&self, ctx: &Context) -> Result<()> {
        let params = self.build_params()?;
        let data = ctx.client.rsi(params).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_stddev(&self, ctx: &Context) -> Result<()> {
        let params = self.build_params()?;
        let data = ctx.client.standard_deviation(params).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_williams(&self, ctx: &Context) -> Result<()> {
        let params = self.build_params()?;
        let data = ctx.client.williams(params).await?;
        crate::output::output_json(&data)
    }

    pub async fn handle_adx(&self, ctx: &Context) -> Result<()> {
        let params = self.build_params()?;
        let data = ctx.client.adx(params).await?;
        crate::output::output_json(&data)
    }
}
