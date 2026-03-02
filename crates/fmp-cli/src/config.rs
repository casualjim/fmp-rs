use clap::Parser;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[command(name = "fmp")]
#[command(about = "Financial Modeling Prep CLI", long_about = None)]
pub struct Cli {
    #[arg(long = "api-key", env = "FMP_API_KEY")]
    pub api_key: Option<String>,

    #[arg(long = "base-url", env = "FMP_BASE_URL", 
          default_value = "https://financialmodelingprep.com/stable/")]
    pub base_url: Url,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(subcommand)]
    Quotes(crate::cli::QuotesArgs),
    #[command(subcommand)]
    Chart(crate::cli::ChartArgs),
    #[command(subcommand)]
    Company(crate::cli::CompanyArgs),
    #[command(subcommand)]
    Statements(crate::cli::StatementsArgs),
    #[command(subcommand)]
    Crypto(crate::cli::CryptoArgs),
    #[command(subcommand)]
    Forex(crate::cli::ForexArgs),
    #[command(subcommand)]
    Commodities(crate::cli::CommoditiesArgs),
    #[command(subcommand)]
    Indexes(crate::cli::IndexesArgs),
    #[command(subcommand)]
    Dcf(crate::cli::DcfArgs),
    #[command(subcommand)]
    Cot(crate::cli::CotArgs),
    #[command(subcommand)]
    Esg(crate::cli::EsgArgs),
}
