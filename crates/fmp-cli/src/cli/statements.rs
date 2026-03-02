use clap::{Args, Subcommand};
use eyre::Result;
use fmp::StatementsApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum StatementsArgs {
    Income(IncomeArgs),
    IncomeTtm(IncomeTtmArgs),
    IncomeGrowth(IncomeGrowthArgs),
    IncomeAsReported(IncomeAsReportedArgs),
    BalanceSheet(BalanceSheetArgs),
    BalanceSheetTtm(BalanceSheetTtmArgs),
    BalanceSheetGrowth(BalanceSheetGrowthArgs),
    BalanceSheetAsReported(BalanceSheetAsReportedArgs),
    CashFlow(CashFlowArgs),
    CashFlowTtm(CashFlowTtmArgs),
    CashFlowGrowth(CashFlowGrowthArgs),
    CashFlowAsReported(CashFlowAsReportedArgs),
    FullAsReported(FullAsReportedArgs),
    Latest(LatestArgs),
    FinancialGrowth(FinancialGrowthArgs),
    ReportDates(ReportDatesArgs),
    ReportJson(ReportJsonArgs),
    RevenueProduct(RevenueProductArgs),
    RevenueGeographic(RevenueGeographicArgs),
    KeyMetrics(KeyMetricsArgs),
    KeyMetricsTtm(KeyMetricsTtmArgs),
    Ratios(RatiosArgs),
    RatiosTtm(RatiosTtmArgs),
    Scores(ScoresArgs),
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

#[derive(Args, Debug, Clone)]
pub struct IncomeArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct BalanceSheetArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct CashFlowArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, required = true)]
    pub year: i32,

    #[arg(long, required = true, value_parser = ["Q1", "Q2", "Q3", "Q4", "FY"])]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
    pub period: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct KeyMetricsArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct RatiosArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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

#[derive(Args, Debug, Clone)]
pub struct ScoresArgs {
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long, required = true)]
    pub symbol: String,

    #[arg(long, value_parser = ["annual", "quarterly"])]
    pub period: Option<String>,

    #[arg(long)]
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
    #[arg(long)]
    pub page: Option<u32>,

    #[arg(long)]
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
    #[arg(long, required = true)]
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
    #[arg(long, required = true)]
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
    #[arg(long, required = true)]
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
