use clap::{Args, Subcommand};
use eyre::Result;
use fmp::CommodityApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum CommoditiesArgs {
    List(ListArgs),
}

impl CommoditiesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::List(args) => args.handle(ctx).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct ListArgs;

impl ListArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let data = ctx.client.commodity_list(()).await?;
        crate::output::output_json(&data)
    }
}
