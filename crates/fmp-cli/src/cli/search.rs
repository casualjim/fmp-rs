use clap::{Args, Subcommand};
use eyre::Result;
use fmp::SearchApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum SearchArgs {
    Symbol(SymbolArgs),
    Name(NameArgs),
    Cik(CikArgs),
    Cusip(CusipArgs),
    Isin(IsinArgs),
    ExchangeVariants(ExchangeVariantsArgs),
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
    #[arg(long, required = true)]
    pub query: String,
    
    #[arg(long)]
    pub limit: Option<u32>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub query: String,
    
    #[arg(long)]
    pub limit: Option<u32>,
    
    #[arg(long)]
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
    #[arg(long, required = true)]
    pub cik: String,
    
    #[arg(long)]
    pub limit: Option<u32>,
}

impl CikArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.limit {
            Some(limit) => fmp::types::search::CikSearchParams::builder()
                .cik(&self.cik)
                .limit(limit)
                .build(),
            None => fmp::types::search::CikSearchParams::builder()
                .cik(&self.cik)
                .build(),
        };
        let data = ctx.client.search_cik(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CusipArgs {
    #[arg(long, required = true)]
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
    #[arg(long, required = true)]
    pub isin: String,
}

impl IsinArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::search::IsinSearchParams::builder()
            .isin(&self.isin)
            .build();
        let data = ctx.client.search_isin(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ExchangeVariantsArgs {
    #[arg(long, required = true)]
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
    #[arg(long)]
    pub market_cap_more_than: Option<f64>,
    
    #[arg(long)]
    pub market_cap_less_than: Option<f64>,
    
    #[arg(long)]
    pub price_more_than: Option<f64>,
    
    #[arg(long)]
    pub price_less_than: Option<f64>,
    
    #[arg(long)]
    pub beta_more_than: Option<f64>,
    
    #[arg(long)]
    pub beta_less_than: Option<f64>,
    
    #[arg(long)]
    pub volume_more_than: Option<f64>,
    
    #[arg(long)]
    pub volume_less_than: Option<f64>,
    
    #[arg(long)]
    pub dividend: Option<f64>,
    
    #[arg(long)]
    pub sector: Option<String>,
    
    #[arg(long)]
    pub industry: Option<String>,
    
    #[arg(long)]
    pub exchange: Option<String>,
    
    #[arg(long)]
    pub country: Option<String>,
    
    #[arg(long)]
    pub is_etf: Option<bool>,
    
    #[arg(long)]
    pub is_fund: Option<bool>,
    
    #[arg(long)]
    pub is_actively_trading: Option<bool>,
    
    #[arg(long)]
    pub limit: Option<u32>,
    
    #[arg(long)]
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
