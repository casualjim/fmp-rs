use clap::{Args, Subcommand};
use eyre::Result;
use fmp::GovernmentTradingApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum GovernmentTradingArgs {
  /// Latest Senate financial disclosures (paginated)
  SenateLatest(SenateLatestArgs),
  /// Latest House financial disclosures (paginated)
  HouseLatest(HouseLatestArgs),
  /// Senate trades filtered by ticker symbol
  SenateBySymbol(SenateBySymbolArgs),
  /// Senate trades filtered by senator name
  SenateByName(SenateByNameArgs),
  /// House trades filtered by ticker symbol
  HouseBySymbol(HouseBySymbolArgs),
  /// House trades filtered by representative name
  HouseByName(HouseByNameArgs),
}

impl GovernmentTradingArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::SenateLatest(args) => args.handle(ctx).await,
      Self::HouseLatest(args) => args.handle(ctx).await,
      Self::SenateBySymbol(args) => args.handle(ctx).await,
      Self::SenateByName(args) => args.handle(ctx).await,
      Self::HouseBySymbol(args) => args.handle(ctx).await,
      Self::HouseByName(args) => args.handle(ctx).await,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct SenateLatestArgs {
  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl SenateLatestArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::government_trading::PaginationParams {
      page: self.page,
      limit: self.limit,
    };
    let data = ctx.client.senate_latest(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct HouseLatestArgs {
  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,

  #[arg(long, help = "Maximum number of records to return")]
  pub limit: Option<u32>,
}

impl HouseLatestArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::government_trading::PaginationParams {
      page: self.page,
      limit: self.limit,
    };
    let data = ctx.client.house_latest(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct SenateBySymbolArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,
}

impl SenateBySymbolArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::government_trading::SymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.senate_trades(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct SenateByNameArgs {
  #[arg(long, required = true, help = "Senator name to search for")]
  pub name: String,
}

impl SenateByNameArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::government_trading::NameParams {
      name: self.name.clone(),
    };
    let data = ctx.client.senate_trades_by_name(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct HouseBySymbolArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,
}

impl HouseBySymbolArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::government_trading::SymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.house_trades(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct HouseByNameArgs {
  #[arg(long, required = true, help = "Representative name to search for")]
  pub name: String,
}

impl HouseByNameArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::government_trading::NameParams {
      name: self.name.clone(),
    };
    let data = ctx.client.house_trades_by_name(params).await?;
    crate::output::output_json(&data)
  }
}
