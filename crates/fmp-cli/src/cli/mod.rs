pub mod quotes;
pub mod chart;
pub mod company;

pub use quotes::QuotesArgs;
pub use chart::ChartArgs;
pub use company::CompanyArgs;

use crate::config::Cli;
use eyre::Result;
use fmp::{FmpConfig, FmpHttpClient};
use secrecy::SecretString;

pub struct Context {
    pub client: FmpHttpClient,
}

impl Context {
    pub fn new(api_key: SecretString, base_url: url::Url) -> Result<Self> {
        let config = FmpConfig::builder()
            .api_key(api_key)
            .base_url(base_url)
            .build();
        let client = FmpHttpClient::new(config)?;
        Ok(Self { client })
    }
}

pub async fn dispatch(cli: Cli) -> Result<()> {
    let api_key = cli.api_key
        .ok_or_else(|| eyre::eyre!("API key required via --api-key or FMP_API_KEY"))?;
    let api_key = SecretString::new(api_key.into_boxed_str());
    
    let ctx = Context::new(api_key, cli.base_url)?;
    
    match cli.command {
        crate::config::Commands::Quotes(args) => args.handle(&ctx).await,
        crate::config::Commands::Chart(args) => args.handle(&ctx).await,
        crate::config::Commands::Company(args) => args.handle(&ctx).await,
    }
}
