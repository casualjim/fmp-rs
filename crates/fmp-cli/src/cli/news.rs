use clap::{Args, Subcommand};
use eyre::Result;
use fmp::NewsApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum NewsArgs {
  /// Latest general market and financial news from all sources
  Latest(LatestArgs),
  /// Stock-specific news articles (filtered to equity/stock coverage)
  Stock(StockArgs),
  /// Cryptocurrency news articles
  Crypto(CryptoArgs),
  /// Forex/currency market news articles
  Forex(ForexArgs),
  /// Search news by ticker symbols with optional date filters
  Search(SearchArgs),
  /// FMP's own editorial articles and market analysis
  FmpArticles(FmpArticlesArgs),
  /// Company press releases
  PressReleases(PressReleasesArgs),
  /// Search press releases by ticker symbols
  SearchPressReleases(SearchPressReleasesArgs),
  /// Search cryptocurrency news by symbols
  SearchCrypto(SearchCryptoArgs),
  /// Search forex news by symbols
  SearchForex(SearchForexArgs),
}

impl NewsArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::Latest(args) => args.handle(ctx).await,
      Self::Stock(args) => args.handle(ctx).await,
      Self::Crypto(args) => args.handle(ctx).await,
      Self::Forex(args) => args.handle(ctx).await,
      Self::Search(args) => args.handle(ctx).await,
      Self::FmpArticles(args) => args.handle(ctx).await,
      Self::PressReleases(args) => args.handle(ctx).await,
      Self::SearchPressReleases(args) => args.handle(ctx).await,
      Self::SearchCrypto(args) => args.handle(ctx).await,
      Self::SearchForex(args) => args.handle(ctx).await,
    }
  }
}

fn parse_news_params(
  from: Option<&str>,
  to: Option<&str>,
  limit: Option<u32>,
  page: Option<u32>,
) -> Result<fmp::types::news::NewsParams> {
  Ok(fmp::types::news::NewsParams {
    from: from.map(str::parse::<jiff::civil::Date>).transpose()?.map(fmp::FmpDate),
    to: to.map(str::parse::<jiff::civil::Date>).transpose()?.map(fmp::FmpDate),
    limit,
    page,
  })
}

#[derive(Args, Debug, Clone)]
pub struct LatestArgs {
  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl LatestArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?;
    let data = ctx.client.general_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct StockArgs {
  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl StockArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?;
    let data = ctx.client.stock_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct CryptoArgs {
  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl CryptoArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?;
    let data = ctx.client.crypto_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct ForexArgs {
  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl ForexArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?;
    let data = ctx.client.forex_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
  #[arg(
    long,
    required = true,
    help = "Comma-separated ticker symbols to filter news (e.g., AAPL,MSFT)"
  )]
  pub symbols: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl SearchArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::news::NewsSearchParams {
      symbols: self.symbols.clone(),
      params: parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?,
    };
    let data = ctx.client.search_stock_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct FmpArticlesArgs {
  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl FmpArticlesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?;
    let data = ctx.client.fmp_articles(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct PressReleasesArgs {
  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl PressReleasesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?;
    let data = ctx.client.press_releases(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct SearchPressReleasesArgs {
  #[arg(
    long,
    required = true,
    help = "Comma-separated ticker symbols to filter press releases (e.g., AAPL,MSFT)"
  )]
  pub symbols: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl SearchPressReleasesArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::news::NewsSearchParams {
      symbols: self.symbols.clone(),
      params: parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?,
    };
    let data = ctx.client.search_press_releases(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct SearchCryptoArgs {
  #[arg(
    long,
    required = true,
    help = "Comma-separated crypto symbols to filter news (e.g., BTCUSD,ETHUSD)"
  )]
  pub symbols: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl SearchCryptoArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::news::NewsSearchParams {
      symbols: self.symbols.clone(),
      params: parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?,
    };
    let data = ctx.client.search_crypto_news(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct SearchForexArgs {
  #[arg(
    long,
    required = true,
    help = "Comma-separated forex pair symbols to filter news (e.g., EURUSD,GBPUSD)"
  )]
  pub symbols: String,

  #[arg(long, help = "Start date in YYYY-MM-DD format")]
  pub from: Option<String>,

  #[arg(long, help = "End date in YYYY-MM-DD format")]
  pub to: Option<String>,

  #[arg(long, help = "Maximum number of articles to return")]
  pub limit: Option<u32>,

  #[arg(long, help = "Page number for pagination")]
  pub page: Option<u32>,
}

impl SearchForexArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::news::NewsSearchParams {
      symbols: self.symbols.clone(),
      params: parse_news_params(self.from.as_deref(), self.to.as_deref(), self.limit, self.page)?,
    };
    let data = ctx.client.search_forex_news(params).await?;
    crate::output::output_json(&data)
  }
}
