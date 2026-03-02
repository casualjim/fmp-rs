use clap::{Args, Subcommand};
use eyre::Result;
use fmp::IndexesApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum IndexesArgs {
    List(ListArgs),
    Quote(QuoteArgs),
    QuoteShort(QuoteShortArgs),
    QuoteBatch(QuoteBatchArgs),
    EodLight(EodLightArgs),
    EodFull(EodFullArgs),
    Intraday1min(Intraday1minArgs),
    Intraday5min(Intraday5minArgs),
    Intraday1hour(Intraday1hourArgs),
}

impl IndexesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::List(args) => args.handle(ctx).await,
            Self::Quote(args) => args.handle(ctx).await,
            Self::QuoteShort(args) => args.handle(ctx).await,
            Self::QuoteBatch(args) => args.handle(ctx).await,
            Self::EodLight(args) => args.handle(ctx).await,
            Self::EodFull(args) => args.handle(ctx).await,
            Self::Intraday1min(args) => args.handle(ctx).await,
            Self::Intraday5min(args) => args.handle(ctx).await,
            Self::Intraday1hour(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct ListArgs {
    #[arg(long)]
    pub symbol: Option<String>,
}

impl ListArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match &self.symbol {
            Some(symbol) => fmp::types::indexes::IndexSymbolParams::builder()
                .symbol(symbol)
                .build(),
            None => fmp::types::indexes::IndexSymbolParams::builder()
                .symbol("")
                .build(),
        };
        let data = ctx.client.index_list(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct QuoteArgs {
    #[arg(long, required = true)]
    pub symbol: String,
}

impl QuoteArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::indexes::IndexSymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.quote(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct QuoteShortArgs {
    #[arg(long, required = true)]
    pub symbol: String,
}

impl QuoteShortArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::indexes::IndexSymbolParams::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.quote_short(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct QuoteBatchArgs {
    #[arg(long)]
    pub short: Option<bool>,
}

impl QuoteBatchArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.short {
            Some(short) => fmp::types::indexes::IndexBatchParams::builder()
                .short(short)
                .build(),
            None => fmp::types::indexes::IndexBatchParams::builder().build(),
        };
        let data = ctx.client.batch_index_quotes(params).await?;
        crate::output::output_json(&data)
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
            (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
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
            (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.historical_price_eod_full(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday1minArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
    pub to: Option<String>,
}

impl Intraday1minArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.historical_chart_1min(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday5minArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
    pub to: Option<String>,
}

impl Intraday5minArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.historical_chart_5min(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct Intraday1hourArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
    pub to: Option<String>,
}

impl Intraday1hourArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.from, &self.to) {
            (Some(from), Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(from), None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(to)) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None) => fmp::types::indexes::IndexHistoryParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.historical_chart_1hour(params).await?;
        crate::output::output_json(&data)
    }
}
