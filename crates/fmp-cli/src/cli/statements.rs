use clap::{Args, Subcommand};
use eyre::Result;
use fmp::StatementsApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum StatementsArgs {
    /// Annual or quarterly income statement (revenue, expenses, net income, EPS)
    Income(IncomeArgs),
    /// Trailing twelve months income statement
    IncomeTtm(IncomeTtmArgs),
    /// Year-over-year income statement growth rates
    IncomeGrowth(IncomeGrowthArgs),
    /// Income statement as originally reported to the SEC (XBRL data)
    IncomeAsReported(IncomeAsReportedArgs),
    /// Annual or quarterly balance sheet (assets, liabilities, equity)
    BalanceSheet(BalanceSheetArgs),
    /// Trailing twelve months balance sheet
    BalanceSheetTtm(BalanceSheetTtmArgs),
    /// Year-over-year balance sheet growth rates
    BalanceSheetGrowth(BalanceSheetGrowthArgs),
    /// Balance sheet as originally reported to the SEC (XBRL data)
    BalanceSheetAsReported(BalanceSheetAsReportedArgs),
    /// Annual or quarterly cash flow statement (operating, investing, financing)
    CashFlow(CashFlowArgs),
    /// Trailing twelve months cash flow statement
    CashFlowTtm(CashFlowTtmArgs),
    /// Year-over-year cash flow growth rates
    CashFlowGrowth(CashFlowGrowthArgs),
    /// Cash flow statement as originally reported to the SEC (XBRL data)
    CashFlowAsReported(CashFlowAsReportedArgs),
    /// Complete financial statement (income + balance sheet + cash flow) as reported
    FullAsReported(FullAsReportedArgs),
    /// Latest financial statements across all companies (paginated)
    Latest(LatestArgs),
    /// Multi-metric financial growth rates (revenue, earnings, FCF, etc.)
    FinancialGrowth(FinancialGrowthArgs),
    /// List of dates when financial reports were filed
    ReportDates(ReportDatesArgs),
    /// Full financial report data as JSON for a specific period
    ReportJson(ReportJsonArgs),
    /// Revenue segmented by product/service category
    RevenueProduct(RevenueProductArgs),
    /// Revenue segmented by geographic region
    RevenueGeographic(RevenueGeographicArgs),
    /// Key financial metrics (P/E, EV/EBITDA, ROE, FCF yield, etc.)
    KeyMetrics(KeyMetricsArgs),
    /// Trailing twelve months key financial metrics
    KeyMetricsTtm(KeyMetricsTtmArgs),
    /// Financial ratios (current ratio, debt/equity, profit margin, etc.)
    Ratios(RatiosArgs),
    /// Trailing twelve months financial ratios
    RatiosTtm(RatiosTtmArgs),
    /// Altman Z-score, Piotroski F-score, and other financial health scores
    Scores(ScoresArgs),
    /// Owner earnings (Buffett-style: net income + D&A - capex)
    OwnerEarnings(OwnerEarningsArgs),
}

impl StatementsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::Income(args) => args.handle(ctx).await,
            Self::IncomeTtm(args) => args.handle(ctx).await,
            Self::IncomeGrowth(args) => args.handle(ctx).await,
            Self::IncomeAsReported(args) => args.handle(ctx).await,
            Self::BalanceSheet(args) => args.handle(ctx).await,
            Self::BalanceSheetTtm(args) => args.handle(ctx).await,
            Self::BalanceSheetGrowth(args) => args.handle(ctx).await,
            Self::BalanceSheetAsReported(args) => args.handle(ctx).await,
            Self::CashFlow(args) => args.handle(ctx).await,
            Self::CashFlowTtm(args) => args.handle(ctx).await,
            Self::CashFlowGrowth(args) => args.handle(ctx).await,
            Self::CashFlowAsReported(args) => args.handle(ctx).await,
            Self::FullAsReported(args) => args.handle(ctx).await,
            Self::Latest(args) => args.handle(ctx).await,
            Self::FinancialGrowth(args) => args.handle(ctx).await,
            Self::ReportDates(args) => args.handle(ctx).await,
            Self::ReportJson(args) => args.handle(ctx).await,
            Self::RevenueProduct(args) => args.handle(ctx).await,
            Self::RevenueGeographic(args) => args.handle(ctx).await,
            Self::KeyMetrics(args) => args.handle(ctx).await,
            Self::KeyMetricsTtm(args) => args.handle(ctx).await,
            Self::Ratios(args) => args.handle(ctx).await,
            Self::RatiosTtm(args) => args.handle(ctx).await,
            Self::Scores(args) => args.handle(ctx).await,
            Self::OwnerEarnings(args) => args.handle(ctx).await,
        }
    }
}

/// Fetch annual or quarterly income statements (profit & loss statements).
///
/// Returns standardised income statement data including: revenue, cost of revenue,
/// gross profit, R&D expenses, SG&A expenses, operating income, EBITDA, EBIT,
/// interest income/expense, pre-tax income, income tax expense, and net income.
/// Also includes basic and diluted EPS.
///
/// Data is sourced from SEC 10-K (annual) and 10-Q (quarterly) filings,
/// standardised and normalised by FMP across all companies.
///
/// Examples:
///   fmp statements income --symbol AAPL
///   fmp statements income --symbol AAPL --period annual --limit 5
///   fmp statements income --symbol AAPL --period quarterly --limit 8
#[derive(Args, Debug, Clone)]
pub struct IncomeArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: \"annual\" (10-K) or \"quarterly\" (10-Q)")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return (most recent first; default varies by plan)")]
    pub limit: Option<u32>,
}

impl IncomeArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.income_statement(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch annual or quarterly balance sheets (statement of financial position).
///
/// Returns assets (current: cash, receivables, inventory; non-current: PP&E,
/// intangibles, goodwill), liabilities (current: payables, short-term debt;
/// non-current: long-term debt), and shareholders' equity (common stock,
/// retained earnings, accumulated other comprehensive income).
///
/// Examples:
///   fmp statements balance-sheet --symbol AAPL
///   fmp statements balance-sheet --symbol AAPL --period annual --limit 5
///   fmp statements balance-sheet --symbol AAPL --period quarterly --limit 8
#[derive(Args, Debug, Clone)]
pub struct BalanceSheetArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: \"annual\" (10-K) or \"quarterly\" (10-Q)")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return (most recent first)")]
    pub limit: Option<u32>,
}

impl BalanceSheetArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.balance_sheet_statement(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch annual or quarterly cash flow statements.
///
/// Returns cash flows from three activities:
///   Operating: net income, depreciation & amortisation, working capital changes
///   Investing: capital expenditures, acquisitions, asset sales
///   Financing: debt issuance/repayment, share buybacks, dividends paid
/// Also includes free cash flow (operating CF - capex) and net change in cash.
///
/// Examples:
///   fmp statements cash-flow --symbol AAPL
///   fmp statements cash-flow --symbol AAPL --period annual --limit 5
///   fmp statements cash-flow --symbol AAPL --period quarterly --limit 8
#[derive(Args, Debug, Clone)]
pub struct CashFlowArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: \"annual\" (10-K) or \"quarterly\" (10-Q)")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return (most recent first)")]
    pub limit: Option<u32>,
}

impl CashFlowArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.cash_flow_statement(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct IncomeTtmArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl IncomeTtmArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.limit {
            Some(limit) => fmp::types::statements::StatementLimitParams::builder()
                .symbol(&self.symbol)
                .limit(limit)
                .build(),
            None => fmp::types::statements::StatementLimitParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.income_statement_ttm(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct IncomeGrowthArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: annual or quarterly")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return")]
    pub limit: Option<u32>,
}

impl IncomeGrowthArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.income_statement_growth(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct BalanceSheetTtmArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl BalanceSheetTtmArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.limit {
            Some(limit) => fmp::types::statements::StatementLimitParams::builder()
                .symbol(&self.symbol)
                .limit(limit)
                .build(),
            None => fmp::types::statements::StatementLimitParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.balance_sheet_statement_ttm(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct BalanceSheetGrowthArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: annual or quarterly")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return")]
    pub limit: Option<u32>,
}

impl BalanceSheetGrowthArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.balance_sheet_statement_growth(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct BalanceSheetAsReportedArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: annual or quarterly")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return")]
    pub limit: Option<u32>,
}

impl BalanceSheetAsReportedArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.balance_sheet_statement_as_reported(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CashFlowTtmArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return")]
    pub limit: Option<u32>,
}

impl CashFlowTtmArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.limit {
            Some(limit) => fmp::types::statements::StatementLimitParams::builder()
                .symbol(&self.symbol)
                .limit(limit)
                .build(),
            None => fmp::types::statements::StatementLimitParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.cash_flow_statement_ttm(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CashFlowGrowthArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: annual or quarterly")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return")]
    pub limit: Option<u32>,
}

impl CashFlowGrowthArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.cash_flow_statement_growth(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct CashFlowAsReportedArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: annual or quarterly")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return")]
    pub limit: Option<u32>,
}

impl CashFlowAsReportedArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.cash_flow_statement_as_reported(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct FinancialGrowthArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: annual or quarterly")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return")]
    pub limit: Option<u32>,
}

impl FinancialGrowthArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.financial_growth(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ReportDatesArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl ReportDatesArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::statements::SymbolParam::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.financial_reports_dates(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct ReportJsonArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, required = true, help = "Fiscal year (e.g., 2023)")]
    pub year: i32,

    #[arg(long, required = true, value_parser = ["Q1", "Q2", "Q3", "Q4", "FY"], help = "Fiscal period: Q1, Q2, Q3, Q4, or FY")]
    pub period: String,
}

impl ReportJsonArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::statements::FinancialReportParams::builder()
            .symbol(&self.symbol)
            .year(self.year)
            .period(&self.period)
            .build();
        let data = ctx.client.financial_reports_json(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct RevenueProductArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Reporting period (e.g., annual, quarterly)")]
    pub period: Option<String>,

    #[arg(long, help = "Data structure format (flat or hierarchical)")]
    pub structure: Option<String>,
}

impl RevenueProductArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.structure) {
            (Some(period), Some(structure)) => fmp::types::statements::SegmentationParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .structure(structure)
                .build(),
            (Some(period), None) => fmp::types::statements::SegmentationParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(structure)) => fmp::types::statements::SegmentationParams::builder()
                .symbol(&self.symbol)
                .structure(structure)
                .build(),
            (None, None) => fmp::types::statements::SegmentationParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.revenue_product_segmentation(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct RevenueGeographicArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Reporting period (e.g., annual, quarterly)")]
    pub period: Option<String>,

    #[arg(long, help = "Data structure format (flat or hierarchical)")]
    pub structure: Option<String>,
}

impl RevenueGeographicArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.structure) {
            (Some(period), Some(structure)) => fmp::types::statements::SegmentationParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .structure(structure)
                .build(),
            (Some(period), None) => fmp::types::statements::SegmentationParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(structure)) => fmp::types::statements::SegmentationParams::builder()
                .symbol(&self.symbol)
                .structure(structure)
                .build(),
            (None, None) => fmp::types::statements::SegmentationParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.revenue_geographic_segmentation(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch key financial metrics per reporting period.
///
/// Returns a comprehensive set of per-share and ratio metrics calculated from
/// financial statements, including:
///   Valuation: P/E ratio, P/B ratio, EV/EBITDA, EV/Revenue, P/FCF
///   Profitability: ROE, ROA, ROIC, Net margin, Operating margin
///   Leverage: Debt/Equity, Debt/Assets, Interest coverage
///   Efficiency: Asset turnover, Inventory turnover, Days payable
///   Per-share: Revenue/share, FCF/share, Book value/share, Tangible BV/share
///   Yield: Dividend yield, FCF yield, Earnings yield
///
/// Examples:
///   fmp statements key-metrics --symbol AAPL
///   fmp statements key-metrics --symbol AAPL --period annual --limit 10
///   fmp statements key-metrics --symbol AAPL --period quarterly
#[derive(Args, Debug, Clone)]
pub struct KeyMetricsArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: \"annual\" (10-K) or \"quarterly\" (10-Q)")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return (most recent first)")]
    pub limit: Option<u32>,
}

impl KeyMetricsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.key_metrics(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch financial ratios per reporting period.
///
/// Returns a comprehensive set of financial ratios calculated from statements:
///   Liquidity: Current ratio, Quick ratio, Cash ratio
///   Profitability: Gross/Net/Operating profit margins, Return on assets/equity
///   Debt: Debt-to-equity, Debt-to-assets, Interest coverage, Debt service coverage
///   Efficiency: Asset turnover, Inventory turnover, Days sales outstanding
///   Valuation: P/E, P/B, P/S, EV/EBITDA, Price/FCF, Dividend yield
///   Per-share: EPS (basic + diluted), Book value/share, Operating CF/share
///
/// Examples:
///   fmp statements ratios --symbol AAPL
///   fmp statements ratios --symbol AAPL --period annual --limit 5
///   fmp statements ratios --symbol AAPL --period quarterly
#[derive(Args, Debug, Clone)]
pub struct RatiosArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: \"annual\" (10-K) or \"quarterly\" (10-Q)")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return (most recent first)")]
    pub limit: Option<u32>,
}

impl RatiosArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.ratios(params).await?;
        crate::output::output_json(&data)
    }
}

/// Fetch composite financial health scores for a company.
///
/// Returns multiple scoring models per period:
///   Altman Z-Score: bankruptcy prediction (>3.0 safe, <1.8 distress zone)
///   Piotroski F-Score: 9-point financial strength (0–3 weak, 7–9 strong)
///   Beneish M-Score: earnings manipulation detection (<-1.78 likely not manipulating)
///   Intrinsic Value (DCF-based): estimated fair value vs. current price
///
/// Example:
///   fmp statements scores --symbol AAPL
///   fmp statements scores --symbol AAPL --limit 5
#[derive(Args, Debug, Clone)]
pub struct ScoresArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, help = "Maximum number of records to return (most recent first)")]
    pub limit: Option<u32>,
}

impl ScoresArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match self.limit {
            Some(limit) => fmp::types::statements::FinancialScoresParams::builder()
                .symbol(&self.symbol)
                .limit(limit)
                .build(),
            None => fmp::types::statements::FinancialScoresParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.financial_scores(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct IncomeAsReportedArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: annual or quarterly")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return")]
    pub limit: Option<u32>,
}

impl IncomeAsReportedArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.income_statement_as_reported(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct FullAsReportedArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"], help = "Reporting period: annual or quarterly")]
    pub period: Option<String>,

    #[arg(long, help = "Maximum number of periods to return")]
    pub limit: Option<u32>,
}

impl FullAsReportedArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.period, &self.limit) {
            (Some(period), Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .limit(*limit)
                .build(),
            (Some(period), None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .period(period)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementCommonParams::builder()
                .symbol(&self.symbol)
                .build(),
        };
        let data = ctx.client.financial_statement_full_as_reported(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct LatestArgs {
    #[arg(long, help = "Page number for pagination (0-indexed)")]
    pub page: Option<u32>,

    #[arg(long, help = "Number of results per page")]
    pub limit: Option<u32>,
}

impl LatestArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = match (&self.page, &self.limit) {
            (Some(page), Some(limit)) => fmp::types::statements::StatementPaginationParams::builder()
                .page(*page)
                .limit(*limit)
                .build(),
            (Some(page), None) => fmp::types::statements::StatementPaginationParams::builder()
                .page(*page)
                .build(),
            (None, Some(limit)) => fmp::types::statements::StatementPaginationParams::builder()
                .limit(*limit)
                .build(),
            (None, None) => fmp::types::statements::StatementPaginationParams::builder().build(),
        };
        let data = ctx.client.latest_financial_statements(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct KeyMetricsTtmArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl KeyMetricsTtmArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::statements::SymbolParam::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.key_metrics_ttm(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct RatiosTtmArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl RatiosTtmArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::statements::SymbolParam::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.ratios_ttm(params).await?;
        crate::output::output_json(&data)
    }
}

#[derive(Args, Debug, Clone)]
pub struct OwnerEarningsArgs {
    #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
    pub symbol: String,
}

impl OwnerEarningsArgs {
    pub async fn handle(&self, ctx: &Context) -> Result<()> {
        let params = fmp::types::statements::SymbolParam::builder()
            .symbol(&self.symbol)
            .build();
        let data = ctx.client.owner_earnings(params).await?;
        crate::output::output_json(&data)
    }
}
