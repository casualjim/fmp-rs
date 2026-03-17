use clap::{CommandFactory, Parser};
use clap_complete::Shell;
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
    /// Real-time and delayed stock, ETF, crypto, forex, and index quotes
    #[command(subcommand)]
    Quotes(crate::cli::QuotesArgs),
    /// Historical OHLCV price data and technical indicators (SMA, EMA, RSI, etc.)
    #[command(subcommand)]
    Chart(crate::cli::ChartArgs),
    /// Company profiles, executives, market cap, employees, M&A activity
    #[command(subcommand)]
    Company(crate::cli::CompanyArgs),
    /// Income statements, balance sheets, cash flow, ratios, and key metrics
    #[command(subcommand)]
    Statements(crate::cli::StatementsArgs),
    /// Cryptocurrency quotes, historical prices, and news
    #[command(subcommand)]
    Crypto(crate::cli::CryptoArgs),
    /// Forex currency pair quotes, historical rates, and news
    #[command(subcommand)]
    Forex(crate::cli::ForexArgs),
    /// Commodity contract listings and pricing data
    #[command(subcommand)]
    Commodities(crate::cli::CommoditiesArgs),
    /// Market index quotes and historical values
    #[command(subcommand)]
    Indexes(crate::cli::IndexesArgs),
    /// Discounted cash flow (DCF) valuation models
    #[command(subcommand)]
    Dcf(crate::cli::DcfArgs),
    /// CFTC Commitment of Traders (COT) reports and analysis
    #[command(subcommand)]
    Cot(crate::cli::CotArgs),
    /// Environmental, Social, and Governance (ESG) scores and disclosures
    #[command(subcommand)]
    Esg(crate::cli::EsgArgs),
    /// Exchange trading hours, holidays, and market status
    #[command(subcommand)]
    MarketHours(crate::cli::MarketHoursArgs),
    /// Market movers, sector/industry performance, and P/E snapshots
    #[command(subcommand)]
    MarketPerformance(crate::cli::MarketPerformanceArgs),
    /// Search symbols, companies, CIK/CUSIP/ISIN lookup, and stock screener
    #[command(subcommand)]
    Search(crate::cli::SearchArgs),
    /// Symbol directories, exchange listings, sectors, industries, and countries
    #[command(subcommand)]
    Directory(crate::cli::DirectoryArgs),
    /// Financial news for stocks, crypto, forex, and general markets
    #[command(subcommand)]
    News(crate::cli::NewsArgs),
    /// Earnings, IPO, stock split, and dividend calendars
    #[command(subcommand)]
    Calendar(crate::cli::CalendarArgs),
    /// Economic indicators, treasury rates, Fed funds rate, and calendar events
    #[command(subcommand)]
    Economics(crate::cli::EconomicsArgs),
    /// Analyst ratings, price targets, estimates, and grade changes
    #[command(subcommand)]
    Analyst(crate::cli::AnalystArgs),
    /// SEC filings (10-K, 10-Q, 8-K) by symbol, CIK, or form type
    #[command(subcommand)]
    Filings(crate::cli::FilingsArgs),
    /// Earnings call transcripts by company, quarter, and year
    #[command(subcommand)]
    Transcript(crate::cli::TranscriptArgs),
    /// Insider trading transactions, beneficial ownership, and statistics
    #[command(subcommand)]
    InsiderTrades(crate::cli::InsiderTradesArgs),
    /// Senate and House financial disclosure trades
    #[command(subcommand)]
    GovernmentTrading(crate::cli::GovernmentTradingArgs),
    /// Form 13F institutional ownership filings and analytics
    #[command(subcommand)]
    Form13F(crate::cli::Form13FArgs),
    /// ETF holdings, info, sector/country weightings, and fund disclosures
    #[command(subcommand)]
    Fund(crate::cli::FundArgs),
    /// Crowdfunding offerings and equity fundraising (Reg D/Reg A) filings
    #[command(subcommand)]
    Fundraisers(crate::cli::FundraisersArgs),
    /// Technical indicators: SMA, EMA, WMA, DEMA, TEMA, RSI, StdDev, Williams, ADX
    #[command(subcommand)]
    TechnicalIndicators(crate::cli::TechnicalIndicatorsArgs),
    /// Generate shell completion scripts (bash, zsh, fish)
    Completions(CompletionsArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(long, value_enum)]
    pub shell: Shell,
}

impl CompletionsArgs {
    pub fn handle(&self) {
        let mut cmd = Cli::command();
        clap_complete::generate(self.shell, &mut cmd, "fmp", &mut std::io::stdout());
    }
}
