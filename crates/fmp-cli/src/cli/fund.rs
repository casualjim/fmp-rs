use clap::{Args, Subcommand};
use eyre::Result;
use fmp::FundApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum FundArgs {
  /// ETF holdings list for a fund symbol
  EtfHoldings(EtfHoldingsArgs),
  /// ETF fund information (AUM, NAV, expense ratio, description)
  EtfInfo(EtfInfoArgs),
  /// ETF country allocation weightings
  EtfCountries(EtfCountriesArgs),
  /// ETF individual asset exposures
  EtfAssets(EtfAssetsArgs),
  /// ETF sector weightings
  EtfSectors(EtfSectorsArgs),
  /// Latest fund disclosure holders for a symbol
  DisclosureHoldersLatest(DisclosureHoldersLatestArgs),
  /// Search fund disclosure holders by fund name
  DisclosureHoldersSearch(DisclosureHoldersSearchArgs),
  /// Available disclosure dates for a fund symbol
  DisclosureDates(DisclosureDatesArgs),
  /// Fund portfolio disclosure for a specific year and quarter
  Disclosure(DisclosureArgs),
}

impl FundArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::EtfHoldings(args) => args.handle(ctx).await,
      Self::EtfInfo(args) => args.handle(ctx).await,
      Self::EtfCountries(args) => args.handle(ctx).await,
      Self::EtfAssets(args) => args.handle(ctx).await,
      Self::EtfSectors(args) => args.handle(ctx).await,
      Self::DisclosureHoldersLatest(args) => args.handle(ctx).await,
      Self::DisclosureHoldersSearch(args) => args.handle(ctx).await,
      Self::DisclosureDates(args) => args.handle(ctx).await,
      Self::Disclosure(args) => args.handle(ctx).await,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct EtfHoldingsArgs {
  #[arg(long, required = true, help = "ETF ticker symbol (e.g., SPY, QQQ)")]
  pub symbol: String,
}

impl EtfHoldingsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::fund::FundSymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.etf_holdings(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EtfInfoArgs {
  #[arg(long, required = true, help = "ETF ticker symbol (e.g., SPY, QQQ)")]
  pub symbol: String,
}

impl EtfInfoArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::fund::FundSymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.etf_info(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EtfCountriesArgs {
  #[arg(long, required = true, help = "ETF ticker symbol (e.g., SPY, QQQ)")]
  pub symbol: String,
}

impl EtfCountriesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::fund::FundSymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.etf_country_weightings(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EtfAssetsArgs {
  #[arg(long, required = true, help = "ETF ticker symbol (e.g., SPY, QQQ)")]
  pub symbol: String,
}

impl EtfAssetsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::fund::FundSymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.etf_asset_exposure(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct EtfSectorsArgs {
  #[arg(long, required = true, help = "ETF ticker symbol (e.g., SPY, QQQ)")]
  pub symbol: String,
}

impl EtfSectorsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::fund::FundSymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.etf_sector_weightings(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct DisclosureHoldersLatestArgs {
  #[arg(long, required = true, help = "Fund ticker symbol")]
  pub symbol: String,
}

impl DisclosureHoldersLatestArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::fund::FundSymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.funds_disclosure_holders_latest(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct DisclosureHoldersSearchArgs {
  #[arg(long, required = true, help = "Fund name to search for")]
  pub name: String,
}

impl DisclosureHoldersSearchArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::fund::FundDisclosureSearchParams {
      name: self.name.clone(),
    };
    let data = ctx.client.funds_disclosure_holders_search(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct DisclosureDatesArgs {
  #[arg(long, required = true, help = "Fund ticker symbol")]
  pub symbol: String,

  #[arg(long, help = "Fund CIK number (optional, for disambiguation)")]
  pub cik: Option<String>,
}

impl DisclosureDatesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match &self.cik {
      Some(cik) => fmp::types::fund::FundDisclosureDatesParams::builder()
        .symbol(&self.symbol)
        .cik(cik)
        .build(),
      None => fmp::types::fund::FundDisclosureDatesParams::builder()
        .symbol(&self.symbol)
        .build(),
    };
    let data = ctx.client.funds_disclosure_dates(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct DisclosureArgs {
  #[arg(long, required = true, help = "Fund ticker symbol")]
  pub symbol: String,

  #[arg(long, required = true, help = "Disclosure year")]
  pub year: i32,

  #[arg(long, required = true, help = "Disclosure quarter (1-4)")]
  pub quarter: i32,

  #[arg(long, help = "Fund CIK number (optional, for disambiguation)")]
  pub cik: Option<String>,
}

impl DisclosureArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match &self.cik {
      Some(cik) => fmp::types::fund::FundDisclosureParams::builder()
        .symbol(&self.symbol)
        .year(self.year)
        .quarter(self.quarter)
        .cik(cik)
        .build(),
      None => fmp::types::fund::FundDisclosureParams::builder()
        .symbol(&self.symbol)
        .year(self.year)
        .quarter(self.quarter)
        .build(),
    };
    let data = ctx.client.funds_disclosure(params).await?;
    crate::output::output_json(&data)
  }
}
