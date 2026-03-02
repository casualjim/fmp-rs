use clap::{Args, Subcommand};
use eyre::Result;
use fmp::{ChartApi, ChartIntervalApi, TechnicalIndicatorsApi};

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum ChartArgs {
    EodLight(EodLightArgs),
    EodFull(EodFullArgs),
    EodNonSplit(EodNonSplitArgs),
    EodDividend(EodDividendArgs),
    Intraday(IntradayArgs),
    Sma(TechnicalIndicatorArgs),
    Ema(TechnicalIndicatorArgs),
    Wma(TechnicalIndicatorArgs),
    Dema(TechnicalIndicatorArgs),
    Tema(TechnicalIndicatorArgs),
    Rsi(TechnicalIndicatorArgs),
    Stddev(TechnicalIndicatorArgs),
    Williams(TechnicalIndicatorArgs),
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

#[derive(Args, Debug, Clone)]
pub struct EodLightArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct EodFullArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct EodNonSplitArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct EodDividendArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct IntradayArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, default_value = "1hour")]
    pub interval: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct TechnicalIndicatorArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, default_value = "20")]
    pub period: u32,

    #[arg(long, default_value = "daily")]
    pub timeframe: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
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
