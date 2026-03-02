use clap::{Args, Subcommand};
use eyre::Result;
use fmp::QuotesApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum QuotesArgs {
    /// Full quote for one or more symbols (price, volume, change, market cap, etc.)
    Get(GetArgs),
    /// Lightweight quote with price and basic metrics only
    Short(ShortArgs),
    /// Batch full quotes using comma-separated symbols string
    Batch(BatchArgs),
    /// Batch lightweight quotes using comma-separated symbols string
    BatchShort(BatchShortArgs),
    /// Batch after-market quotes for multiple symbols
    BatchAftermarket(BatchAftermarketArgs),
    /// All quotes for a specific exchange (e.g., NYSE, NASDAQ)
    Exchange(ExchangeArgs),
    /// After-market trade data for one or more symbols
    AftermarketTrade(AftermarketTradeArgs),
    /// After-market quote data for one or more symbols
    AftermarketQuote(AftermarketQuoteArgs),
    /// Price change over multiple periods (1D, 5D, 1M, 3M, 6M, YTD, 1Y, 3Y, 5Y, 10Y)
    PriceChange(PriceChangeArgs),
    /// All mutual fund quotes (optionally as short/lightweight format)
    MutualFund(MutualFundArgs),
    /// All ETF quotes (optionally as short/lightweight format)
    Etf(EtfArgs),
    /// All commodity quotes (optionally as short/lightweight format)
    Commodity(CommodityArgs),
    /// All cryptocurrency quotes (optionally as short/lightweight format)
    Crypto(CryptoArgs),
    /// All forex pair quotes (optionally as short/lightweight format)
    Forex(ForexArgs),
    /// All market index quotes (optionally as short/lightweight format)
    Index(IndexArgs),
}

impl QuotesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Get(args) => args.handle(ctx).await,
            Self::Short(args) => args.handle(ctx).await,
            Self::Batch(args) => args.handle(ctx).await,
            Self::BatchShort(args) => args.handle(ctx).await,
            Self::BatchAftermarket(args) => args.handle(ctx).await,
            Self::Exchange(args) => args.handle(ctx).await,
            Self::AftermarketTrade(args) => args.handle(ctx).await,
            Self::AftermarketQuote(args) => args.handle(ctx).await,
            Self::PriceChange(args) => args.handle(ctx).await,
            Self::MutualFund(args) => args.handle(ctx).await,
            Self::Etf(args) => args.handle(ctx).await,
            Self::Commodity(args) => args.handle(ctx).await,
            Self::Crypto(args) => args.handle(ctx).await,
            Self::Forex(args) => args.handle(ctx).await,
            Self::Index(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct GetArgs {
    #[arg(required = true, help = "One or more ticker symbols (e.g., AAPL MSFT GOOGL)")]
    pub symbols: Vec<String>,
}

impl GetArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let symbols = self.symbols.join(",");
        let params = fmp::types::quotes::BatchQuoteParams::builder()
            .symbols(symbols)
            .build();
        let quotes = ctx.client.batch_quote(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ShortArgs {
    #[arg(required = true, help = "One or more ticker symbols")]
    pub symbols: Vec<String>,
}

impl ShortArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let symbols = self.symbols.join(",");
        let params = fmp::types::quotes::BatchQuoteParams::builder()
            .symbols(symbols)
            .build();
        let quotes = ctx.client.batch_quote_short(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct BatchArgs {
    #[arg(required = true, help = "Comma-separated symbols (e.g., \"AAPL,MSFT,GOOGL\")")]
    pub symbols: String,
}

impl BatchArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::quotes::BatchQuoteParams::builder()
            .symbols(&self.symbols)
            .build();
        let quotes = ctx.client.batch_quote(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct BatchShortArgs {
    #[arg(required = true, help = "Comma-separated symbols (e.g., \"AAPL,MSFT\")")]
    pub symbols: String,
}

impl BatchShortArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::quotes::BatchQuoteParams::builder()
            .symbols(&self.symbols)
            .build();
        let quotes = ctx.client.batch_quote_short(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct BatchAftermarketArgs {
    #[arg(required = true, help = "Comma-separated symbols")]
    pub symbols: String,
}

impl BatchAftermarketArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::quotes::BatchQuoteParams::builder()
            .symbols(&self.symbols)
            .build();
        let quotes = ctx.client.batch_aftermarket_quote(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ExchangeArgs {
    #[arg(required = true, help = "Exchange name (e.g., NYSE, NASDAQ, AMEX)")]
    pub exchange: String,

    #[arg(long, help = "Return lightweight quotes instead of full quotes")]
    pub short: Option<bool>,
}

impl ExchangeArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.short {
            Some(short) => fmp::types::quotes::ExchangeQuoteParams::builder()
                .exchange(&self.exchange)
                .short(short)
                .build(),
            None => fmp::types::quotes::ExchangeQuoteParams::builder()
                .exchange(&self.exchange)
                .build(),
        };
        let quotes = ctx.client.exchange_quotes(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AftermarketTradeArgs {
    #[arg(required = true, help = "One or more ticker symbols")]
    pub symbols: Vec<String>,
}

impl AftermarketTradeArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let symbols = self.symbols.join(",");
        let params = fmp::types::quotes::BatchQuoteParams::builder()
            .symbols(symbols)
            .build();
        let quotes = ctx.client.batch_aftermarket_trade(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AftermarketQuoteArgs {
    #[arg(required = true, help = "One or more ticker symbols")]
    pub symbols: Vec<String>,
}

impl AftermarketQuoteArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let symbols = self.symbols.join(",");
        let params = fmp::types::quotes::BatchQuoteParams::builder()
            .symbols(symbols)
            .build();
        let quotes = ctx.client.batch_aftermarket_quote(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct PriceChangeArgs {
    #[arg(required = true, help = "One or more ticker symbols")]
    pub symbols: Vec<String>,
}

impl PriceChangeArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let symbols = self.symbols.join(",");
        let params = fmp::types::quotes::QuoteParams::builder()
            .symbol(symbols)
            .build();
        let quotes = ctx.client.stock_price_change(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct MutualFundArgs {
    #[arg(long, help = "Return lightweight quotes instead of full quotes")]
    pub short: Option<bool>,
}

impl MutualFundArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.short {
            Some(short) => fmp::types::quotes::ShortParams::builder()
                .short(short)
                .build(),
            None => fmp::types::quotes::ShortParams::builder().build(),
        };
        let quotes = ctx.client.mutual_fund_quotes(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EtfArgs {
    #[arg(long, help = "Return lightweight quotes instead of full quotes")]
    pub short: Option<bool>,
}

impl EtfArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.short {
            Some(short) => fmp::types::quotes::ShortParams::builder()
                .short(short)
                .build(),
            None => fmp::types::quotes::ShortParams::builder().build(),
        };
        let quotes = ctx.client.etf_quotes(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CommodityArgs {
    #[arg(long, help = "Return lightweight quotes instead of full quotes")]
    pub short: Option<bool>,
}

impl CommodityArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.short {
            Some(short) => fmp::types::quotes::ShortParams::builder()
                .short(short)
                .build(),
            None => fmp::types::quotes::ShortParams::builder().build(),
        };
        let quotes = ctx.client.commodity_quotes(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CryptoArgs {
    #[arg(long, help = "Return lightweight quotes instead of full quotes")]
    pub short: Option<bool>,
}

impl CryptoArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.short {
            Some(short) => fmp::types::quotes::ShortParams::builder()
                .short(short)
                .build(),
            None => fmp::types::quotes::ShortParams::builder().build(),
        };
        let quotes = ctx.client.crypto_quotes(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ForexArgs {
    #[arg(long, help = "Return lightweight quotes instead of full quotes")]
    pub short: Option<bool>,
}

impl ForexArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.short {
            Some(short) => fmp::types::quotes::ShortParams::builder()
                .short(short)
                .build(),
            None => fmp::types::quotes::ShortParams::builder().build(),
        };
        let quotes = ctx.client.forex_quotes(params).await?;
        crate::output::output_json(&quotes)
    }
}

#[derive(Args, Debug, Clone)]
pub struct IndexArgs {
    #[arg(long, help = "Return lightweight quotes instead of full quotes")]
    pub short: Option<bool>,
}

impl IndexArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.short {
            Some(short) => fmp::types::quotes::ShortParams::builder()
                .short(short)
                .build(),
            None => fmp::types::quotes::ShortParams::builder().build(),
        };
        let quotes = ctx.client.index_quotes(params).await?;
        crate::output::output_json(&quotes)
    }
}
