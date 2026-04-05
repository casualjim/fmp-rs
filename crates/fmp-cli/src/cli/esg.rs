use clap::{Args, Subcommand};
use eyre::Result;
use fmp::EsgApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum EsgArgs {
  /// ESG disclosure data: environmental, social, and governance metrics
  Disclosure(DisclosureArgs),
  /// ESG ratings and scores from FMP's rating system
  Ratings(RatingsArgs),
  /// ESG benchmark averages by sector/industry for a given year
  Benchmark(BenchmarkArgs),
}

impl EsgArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::Disclosure(args) => args.handle(ctx).await,
      Self::Ratings(args) => args.handle(ctx).await,
      Self::Benchmark(args) => args.handle(ctx).await,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct DisclosureArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,
}

impl DisclosureArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::esg::EsgSymbolParams::builder().symbol(&self.symbol).build();
    let data = ctx.client.esg_disclosure(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct RatingsArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,
}

impl RatingsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::esg::EsgSymbolParams::builder().symbol(&self.symbol).build();
    let data = ctx.client.esg_ratings(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct BenchmarkArgs {
  #[arg(long, help = "Year to retrieve benchmark data for (e.g., 2023)")]
  pub year: Option<String>,
}

impl BenchmarkArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match &self.year {
      Some(year) => fmp::types::esg::EsgBenchmarkParams::builder().year(year).build(),
      None => fmp::types::esg::EsgBenchmarkParams::builder().build(),
    };
    let data = ctx.client.esg_benchmark(params).await?;
    crate::output::output_json(&data)
  }
}
