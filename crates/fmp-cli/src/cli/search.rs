use clap::{Args, Subcommand};
use eyre::Result;
use fmp::SearchApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum SearchArgs {
  /// Search for ticker symbols by keyword query (company name or symbol prefix)
  Symbol(SymbolArgs),
  /// Search for companies by full name with optional exchange filter
  Name(NameArgs),
  /// Look up a company's ticker symbol by SEC CIK number
  Cik(CikArgs),
  /// Look up a ticker symbol by CUSIP identifier
  Cusip(CusipArgs),
  /// Look up a ticker symbol by ISIN (International Securities Identification Number)
  Isin(IsinArgs),
  /// Find all exchange listings for a symbol (e.g., US + foreign exchanges)
  ExchangeVariants(ExchangeVariantsArgs),
  /// Screen stocks by financial criteria (market cap, price, volume, sector, etc.)
  Screener(ScreenerArgs),
}

impl SearchArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::Symbol(args) => args.handle(ctx).await,
      Self::Name(args) => args.handle(ctx).await,
      Self::Cik(args) => args.handle(ctx).await,
      Self::Cusip(args) => args.handle(ctx).await,
      Self::Isin(args) => args.handle(ctx).await,
      Self::ExchangeVariants(args) => args.handle(ctx).await,
      Self::Screener(args) => args.handle(ctx).await,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct SymbolArgs {
  #[arg(long, required = true, help = "Search query (ticker prefix or company name keyword)")]
  pub query: String,

  #[arg(long, help = "Maximum number of results to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Filter results to a specific exchange (e.g., NYSE, NASDAQ)")]
  pub exchange: Option<String>,
}

impl SymbolArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.limit, &self.exchange) {
      (Some(limit), Some(exchange)) => fmp::types::search::SymbolSearchParams::builder()
        .query(&self.query)
        .limit(*limit)
        .exchange(exchange)
        .build(),
      (Some(limit), None) => fmp::types::search::SymbolSearchParams::builder()
        .query(&self.query)
        .limit(*limit)
        .build(),
      (None, Some(exchange)) => fmp::types::search::SymbolSearchParams::builder()
        .query(&self.query)
        .exchange(exchange)
        .build(),
      (None, None) => fmp::types::search::SymbolSearchParams::builder()
        .query(&self.query)
        .build(),
    };
    let data = ctx.client.search_symbol(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct NameArgs {
  #[arg(long, required = true, help = "Company name or keyword to search for")]
  pub query: String,

  #[arg(long, help = "Maximum number of results to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Filter results to a specific exchange (e.g., NYSE)")]
  pub exchange: Option<String>,
}

impl NameArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match (&self.limit, &self.exchange) {
      (Some(limit), Some(exchange)) => fmp::types::search::NameSearchParams::builder()
        .query(&self.query)
        .limit(*limit)
        .exchange(exchange)
        .build(),
      (Some(limit), None) => fmp::types::search::NameSearchParams::builder()
        .query(&self.query)
        .limit(*limit)
        .build(),
      (None, Some(exchange)) => fmp::types::search::NameSearchParams::builder()
        .query(&self.query)
        .exchange(exchange)
        .build(),
      (None, None) => fmp::types::search::NameSearchParams::builder()
        .query(&self.query)
        .build(),
    };
    let data = ctx.client.search_name(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct CikArgs {
  #[arg(long, required = true, help = "SEC CIK number to look up")]
  pub cik: String,

  #[arg(long, help = "Maximum number of results to return")]
  pub limit: Option<u32>,
}

impl CikArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = match self.limit {
      Some(limit) => fmp::types::search::CikSearchParams::builder()
        .cik(&self.cik)
        .limit(limit)
        .build(),
      None => fmp::types::search::CikSearchParams::builder().cik(&self.cik).build(),
    };
    let data = ctx.client.search_cik(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct CusipArgs {
  #[arg(long, required = true, help = "9-character CUSIP identifier")]
  pub cusip: String,
}

impl CusipArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::search::CusipSearchParams::builder()
      .cusip(&self.cusip)
      .build();
    let data = ctx.client.search_cusip(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct IsinArgs {
  #[arg(long, required = true, help = "12-character ISIN identifier (e.g., US0378331005)")]
  pub isin: String,
}

impl IsinArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::search::IsinSearchParams::builder().isin(&self.isin).build();
    let data = ctx.client.search_isin(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct ExchangeVariantsArgs {
  #[arg(long, required = true, help = "Ticker symbol to find all exchange listings for")]
  pub symbol: String,
}

impl ExchangeVariantsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::search::ExchangeVariantSearchParams::builder()
      .symbol(&self.symbol)
      .build();
    let data = ctx.client.search_exchange_variants(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct ScreenerArgs {
  #[arg(long, help = "Minimum market cap in USD (e.g., 1000000000 for $1B)")]
  pub market_cap_more_than: Option<f64>,

  #[arg(long, help = "Maximum market cap in USD")]
  pub market_cap_less_than: Option<f64>,

  #[arg(long, help = "Minimum stock price in USD")]
  pub price_more_than: Option<f64>,

  #[arg(long, help = "Maximum stock price in USD")]
  pub price_less_than: Option<f64>,

  #[arg(long, help = "Minimum beta (market sensitivity)")]
  pub beta_more_than: Option<f64>,

  #[arg(long, help = "Maximum beta")]
  pub beta_less_than: Option<f64>,

  #[arg(long, help = "Minimum average trading volume")]
  pub volume_more_than: Option<f64>,

  #[arg(long, help = "Maximum average trading volume")]
  pub volume_less_than: Option<f64>,

  #[arg(long, help = "Minimum dividend yield (e.g., 0.02 for 2%)")]
  pub dividend: Option<f64>,

  #[arg(long, help = "Filter by sector (e.g., Technology, Healthcare, Financials)")]
  pub sector: Option<String>,

  #[arg(long, help = "Filter by industry name")]
  pub industry: Option<String>,

  #[arg(long, help = "Filter by exchange (e.g., NYSE, NASDAQ)")]
  pub exchange: Option<String>,

  #[arg(long, help = "Filter by country code (e.g., US, CA, GB)")]
  pub country: Option<String>,

  #[arg(long, help = "Filter to ETFs only (true) or exclude ETFs (false)")]
  pub is_etf: Option<bool>,

  #[arg(long, help = "Filter to mutual funds only (true) or exclude (false)")]
  pub is_fund: Option<bool>,

  #[arg(long, help = "Filter to actively trading stocks only")]
  pub is_actively_trading: Option<bool>,

  #[arg(long, help = "Maximum number of results to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl ScreenerArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::search::StockScreenerParams {
      market_cap_more_than: self.market_cap_more_than,
      market_cap_less_than: self.market_cap_less_than,
      price_more_than: self.price_more_than,
      price_less_than: self.price_less_than,
      beta_more_than: self.beta_more_than,
      beta_less_than: self.beta_less_than,
      volume_more_than: self.volume_more_than,
      volume_less_than: self.volume_less_than,
      dividend: self.dividend,
      sector: self.sector.clone(),
      industry: self.industry.clone(),
      exchange: self.exchange.clone(),
      country: self.country.clone(),
      is_etf: self.is_etf,
      is_fund: self.is_fund,
      is_actively_trading: self.is_actively_trading,
      limit: self.limit,
      page: self.page,
    };

    let data = ctx.client.company_screener(params).await?;
    crate::output::output_json(&data)
  }
}
