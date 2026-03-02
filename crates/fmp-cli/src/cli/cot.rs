use clap::{Args, Subcommand};
use eyre::Result;
use fmp::CotApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum CotArgs {
    Report(ReportArgs),
    Analysis(AnalysisArgs),
}

impl CotArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Report(args) => args.handle(ctx).await,
            Self::Analysis(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct ReportArgs {
    #[arg(long)]
    pub symbol: Option<String>,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
    pub to: Option<String>,
}

impl ReportArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.symbol, &self.from, &self.to) {
            (Some(symbol), Some(from), Some(to)) => fmp::types::cot::CotRangeParams::builder()
                .symbol(symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(symbol), Some(from), None) => fmp::types::cot::CotRangeParams::builder()
                .symbol(symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(symbol), None, Some(to)) => fmp::types::cot::CotRangeParams::builder()
                .symbol(symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(symbol), None, None) => fmp::types::cot::CotRangeParams::builder()
                .symbol(symbol)
                .build(),
            (None, Some(from), Some(to)) => fmp::types::cot::CotRangeParams::builder()
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(from), None) => fmp::types::cot::CotRangeParams::builder()
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None, Some(to)) => fmp::types::cot::CotRangeParams::builder()
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None, None) => fmp::types::cot::CotRangeParams::builder().build(),
        };
        let data = ctx.client.commitment_of_traders_report(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AnalysisArgs {
    #[arg(long)]
    pub symbol: Option<String>,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
    pub to: Option<String>,
}

impl AnalysisArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.symbol, &self.from, &self.to) {
            (Some(symbol), Some(from), Some(to)) => fmp::types::cot::CotRangeParams::builder()
                .symbol(symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(symbol), Some(from), None) => fmp::types::cot::CotRangeParams::builder()
                .symbol(symbol)
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(symbol), None, Some(to)) => fmp::types::cot::CotRangeParams::builder()
                .symbol(symbol)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (Some(symbol), None, None) => fmp::types::cot::CotRangeParams::builder()
                .symbol(symbol)
                .build(),
            (None, Some(from), Some(to)) => fmp::types::cot::CotRangeParams::builder()
                .from(from.parse::<jiff::civil::Date>()?)
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, Some(from), None) => fmp::types::cot::CotRangeParams::builder()
                .from(from.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None, Some(to)) => fmp::types::cot::CotRangeParams::builder()
                .to(to.parse::<jiff::civil::Date>()?)
                .build(),
            (None, None, None) => fmp::types::cot::CotRangeParams::builder().build(),
        };
        let data = ctx.client.commitment_of_traders_analysis(params).await?;
        crate::output::output_json(&data)
    }
}
