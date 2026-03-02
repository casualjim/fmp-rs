use clap::{Args, Subcommand};
use eyre::Result;
use fmp::EarningsTranscriptApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum TranscriptArgs {
    /// Most recent earnings call transcripts across all companies
    Latest(LatestArgs),
    /// Retrieve a specific earnings call transcript by symbol, year, and quarter
    Get(GetArgs),
    /// List all available transcript dates for a company
    Dates(DatesArgs),
    /// List all companies that have earnings call transcripts available
    Available(AvailableArgs),
}

impl TranscriptArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Latest(args) => args.handle(ctx).await,
            Self::Get(args) => args.handle(ctx).await,
            Self::Dates(args) => args.handle(ctx).await,
            Self::Available(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct LatestArgs {
    #[arg(long, help = "Maximum number of transcripts to return")]
    pub limit: Option<u32>,

    #[arg(long, help = "Page number for pagination")]
    pub page: Option<u32>,
}

impl LatestArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::earnings_transcript::LatestTranscriptsParams {
            limit: self.limit,
            page: self.page,
        };
        let data = ctx.client.latest_transcripts(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct GetArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, required = true, help = "Fiscal year of the earnings call (e.g., 2024)")]
    pub year: String,

    #[arg(long, required = true, help = "Fiscal quarter: 1, 2, 3, or 4")]
    pub quarter: String,

    #[arg(long)]
    pub limit: Option<u32>,
}

impl GetArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::earnings_transcript::TranscriptParams {
            symbol: self.symbol.clone(),
            year: self.year.clone(),
            quarter: self.quarter.clone(),
            limit: self.limit,
        };
        let data = ctx.client.transcript(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct DatesArgs {
    #[arg(long, required = true, help = "Ticker symbol to list available transcript dates for")]
    pub symbol: String,
}

impl DatesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::earnings_transcript::TranscriptDatesParams {
            symbol: self.symbol.clone(),
        };
        let data = ctx.client.transcript_dates(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AvailableArgs;

impl AvailableArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.available_transcript_symbols(()).await?;
        crate::output::output_json(&data)
    }
}
