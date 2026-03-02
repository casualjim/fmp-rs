use clap::{Args, Subcommand};
use eyre::Result;
use fmp::DirectoryApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum DirectoryArgs {
    StockList(StockListArgs),
    FinancialStatementSymbols(FinancialStatementSymbolsArgs),
    CikList(CikListArgs),
    SymbolChange(SymbolChangeArgs),
    EtfList(EtfListArgs),
    ActivelyTradingList(ActivelyTradingListArgs),
    EarningsTranscriptList(EarningsTranscriptListArgs),
    AvailableExchanges(AvailableExchangesArgs),
    AvailableSectors(AvailableSectorsArgs),
    AvailableIndustries(AvailableIndustriesArgs),
    AvailableCountries(AvailableCountriesArgs),
}

impl DirectoryArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::StockList(args) => args.handle(ctx).await,
            Self::FinancialStatementSymbols(args) => args.handle(ctx).await,
            Self::CikList(args) => args.handle(ctx).await,
            Self::SymbolChange(args) => args.handle(ctx).await,
            Self::EtfList(args) => args.handle(ctx).await,
            Self::ActivelyTradingList(args) => args.handle(ctx).await,
            Self::EarningsTranscriptList(args) => args.handle(ctx).await,
            Self::AvailableExchanges(args) => args.handle(ctx).await,
            Self::AvailableSectors(args) => args.handle(ctx).await,
            Self::AvailableIndustries(args) => args.handle(ctx).await,
            Self::AvailableCountries(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct StockListArgs;

impl StockListArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.stock_list(()).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct FinancialStatementSymbolsArgs;

impl FinancialStatementSymbolsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.financial_statement_symbol_list(()).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CikListArgs {
    #[arg(long)]
    pub limit: Option<u32>,
}

impl CikListArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::directory::CikListParams {
            limit: self.limit,
        };
        let data = ctx.client.cik_list(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct SymbolChangeArgs {
    #[arg(long)]
    pub invalid: Option<bool>,
    
    #[arg(long)]
    pub limit: Option<u32>,
}

impl SymbolChangeArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::directory::SymbolChangeParams {
            invalid: self.invalid,
            limit: self.limit,
        };
        let data = ctx.client.symbol_change(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EtfListArgs;

impl EtfListArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.etf_list(()).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ActivelyTradingListArgs;

impl ActivelyTradingListArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.actively_trading_list(()).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct EarningsTranscriptListArgs;

impl EarningsTranscriptListArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.earnings_transcript_list(()).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AvailableExchangesArgs;

impl AvailableExchangesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.available_exchanges(()).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AvailableSectorsArgs;

impl AvailableSectorsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.available_sectors(()).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AvailableIndustriesArgs;

impl AvailableIndustriesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.available_industries(()).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct AvailableCountriesArgs;

impl AvailableCountriesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.available_countries(()).await?;
        crate::output::output_json(&data)
    }
}
