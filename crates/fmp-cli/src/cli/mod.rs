pub mod quotes;
pub mod chart;
pub mod company;
pub mod statements;
pub mod crypto;
pub mod forex;
pub mod commodities;
pub mod indexes;
pub mod dcf;
pub mod cot;
pub mod esg;
pub mod market_hours;
pub mod market_performance;
pub mod search;
pub mod directory;
pub mod news;
pub mod calendar;
pub mod economics;
pub mod analyst;
pub mod filings;
pub mod transcript;
pub mod insider_trades;
pub mod government_trading;
pub mod form_13f;
pub mod fund;
pub mod fundraisers;
pub mod technical_indicators;

pub use quotes::QuotesArgs;
pub use chart::ChartArgs;
pub use company::CompanyArgs;
pub use statements::StatementsArgs;
pub use crypto::CryptoArgs;
pub use forex::ForexArgs;
pub use commodities::CommoditiesArgs;
pub use indexes::IndexesArgs;
pub use dcf::DcfArgs;
pub use cot::CotArgs;
pub use esg::EsgArgs;
pub use market_hours::MarketHoursArgs;
pub use market_performance::MarketPerformanceArgs;
pub use search::SearchArgs;
pub use directory::DirectoryArgs;
pub use news::NewsArgs;
pub use calendar::CalendarArgs;
pub use economics::EconomicsArgs;
pub use analyst::AnalystArgs;
pub use filings::FilingsArgs;
pub use transcript::TranscriptArgs;
pub use insider_trades::InsiderTradesArgs;
pub use government_trading::GovernmentTradingArgs;
pub use form_13f::Form13FArgs;
pub use fund::FundArgs;
pub use fundraisers::FundraisersArgs;
pub use technical_indicators::TechnicalIndicatorsArgs;

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

#[derive(serde::Deserialize)]
struct Secrets {
    #[serde(rename = "api-key")]
    api_key: String,
}

fn load_secrets_api_key() -> Option<String> {
    let path = directories::ProjectDirs::from("", "", "fmp")?
        .config_dir()
        .join("secrets.toml");
    let content = std::fs::read_to_string(path).ok()?;
    toml::from_str::<Secrets>(&content).ok().map(|s| s.api_key)
}

pub async fn dispatch(cli: Cli) -> Result<()> {
    // Handle commands that don't require an API key
    if let crate::config::Commands::Completions(args) = &cli.command {
        args.handle();
        return Ok(());
    }

    let api_key = cli.api_key
        .or_else(load_secrets_api_key)
        .ok_or_else(|| eyre::eyre!("API key required via --api-key, FMP_API_KEY, or $CONFIG_DIR/fmp/secrets.toml"))?;
    let api_key = SecretString::new(api_key.into_boxed_str());

    let ctx = Context::new(api_key, cli.base_url)?;

    match cli.command {
        crate::config::Commands::Quotes(args) => args.handle(&ctx).await,
        crate::config::Commands::Chart(args) => args.handle(&ctx).await,
        crate::config::Commands::Company(args) => args.handle(&ctx).await,
        crate::config::Commands::Statements(args) => args.handle(&ctx).await,
        crate::config::Commands::Crypto(args) => args.handle(&ctx).await,
        crate::config::Commands::Forex(args) => args.handle(&ctx).await,
        crate::config::Commands::Commodities(args) => args.handle(&ctx).await,
        crate::config::Commands::Indexes(args) => args.handle(&ctx).await,
        crate::config::Commands::Dcf(args) => args.handle(&ctx).await,
        crate::config::Commands::Cot(args) => args.handle(&ctx).await,
        crate::config::Commands::Esg(args) => args.handle(&ctx).await,
        crate::config::Commands::MarketHours(args) => args.handle(&ctx).await,
        crate::config::Commands::MarketPerformance(args) => args.handle(&ctx).await,
        crate::config::Commands::Search(args) => args.handle(&ctx).await,
        crate::config::Commands::Directory(args) => args.handle(&ctx).await,
        crate::config::Commands::News(args) => args.handle(&ctx).await,
        crate::config::Commands::Calendar(args) => args.handle(&ctx).await,
        crate::config::Commands::Economics(args) => args.handle(&ctx).await,
        crate::config::Commands::Analyst(args) => args.handle(&ctx).await,
        crate::config::Commands::Filings(args) => args.handle(&ctx).await,
        crate::config::Commands::Transcript(args) => args.handle(&ctx).await,
        crate::config::Commands::InsiderTrades(args) => args.handle(&ctx).await,
        crate::config::Commands::GovernmentTrading(args) => args.handle(&ctx).await,
        crate::config::Commands::Form13F(args) => args.handle(&ctx).await,
        crate::config::Commands::Fund(args) => args.handle(&ctx).await,
        crate::config::Commands::Fundraisers(args) => args.handle(&ctx).await,
        crate::config::Commands::TechnicalIndicators(args) => args.handle(&ctx).await,
        crate::config::Commands::Completions(_) => unreachable!("handled above"),
    }
}
