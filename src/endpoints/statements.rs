use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::statements::{
  AsReportedBalanceSheet, AsReportedCashFlowStatement, AsReportedFinancialStatement, AsReportedIncomeStatement,
  BalanceSheetStatement, BalanceSheetStatementGrowth, CashFlowStatement, CashFlowStatementGrowth, FinancialReport10K,
  FinancialReportDate, FinancialReportParams, FinancialScores, FinancialScoresParams, FinancialStatementGrowth,
  IncomeStatement, IncomeStatementGrowth, KeyMetrics, LatestFinancialStatement, OwnerEarnings, Ratios,
  RevenueGeographicSegmentation, RevenueProductSegmentation, SegmentationParams, StatementCommonParams,
  StatementLimitParams, StatementPaginationParams, SymbolParam,
};

pub async fn income_statement(http: &FmpHttpClient, params: StatementCommonParams) -> FmpResult<Vec<IncomeStatement>> {
  http.get_json("/income-statement", &params).await
}

pub async fn balance_sheet_statement(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<BalanceSheetStatement>> {
  http.get_json("/balance-sheet-statement", &params).await
}

pub async fn cash_flow_statement(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<CashFlowStatement>> {
  http.get_json("/cash-flow-statement", &params).await
}

pub async fn latest_financial_statements(
  http: &FmpHttpClient,
  params: StatementPaginationParams,
) -> FmpResult<Vec<LatestFinancialStatement>> {
  http.get_json("/latest-financial-statements", &params).await
}

pub async fn income_statement_ttm(
  http: &FmpHttpClient,
  params: StatementLimitParams,
) -> FmpResult<Vec<IncomeStatement>> {
  http.get_json("/income-statement-ttm", &params).await
}

pub async fn balance_sheet_statement_ttm(
  http: &FmpHttpClient,
  params: StatementLimitParams,
) -> FmpResult<Vec<BalanceSheetStatement>> {
  http.get_json("/balance-sheet-statement-ttm", &params).await
}

pub async fn cash_flow_statement_ttm(
  http: &FmpHttpClient,
  params: StatementLimitParams,
) -> FmpResult<Vec<CashFlowStatement>> {
  http.get_json("/cash-flow-statement-ttm", &params).await
}

pub async fn income_statement_growth(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<IncomeStatementGrowth>> {
  http.get_json("/income-statement-growth", &params).await
}

pub async fn balance_sheet_statement_growth(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<BalanceSheetStatementGrowth>> {
  http.get_json("/balance-sheet-statement-growth", &params).await
}

pub async fn cash_flow_statement_growth(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<CashFlowStatementGrowth>> {
  http.get_json("/cash-flow-statement-growth", &params).await
}

pub async fn financial_growth(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<FinancialStatementGrowth>> {
  http.get_json("/financial-growth", &params).await
}

pub async fn financial_reports_dates(http: &FmpHttpClient, params: SymbolParam) -> FmpResult<Vec<FinancialReportDate>> {
  http.get_json("/financial-reports-dates", &params).await
}

pub async fn financial_reports_json(
  http: &FmpHttpClient,
  params: FinancialReportParams,
) -> FmpResult<Vec<FinancialReport10K>> {
  http.get_json("/financial-reports-json", &params).await
}

pub async fn revenue_product_segmentation(
  http: &FmpHttpClient,
  params: SegmentationParams,
) -> FmpResult<Vec<RevenueProductSegmentation>> {
  http.get_json("/revenue-product-segmentation", &params).await
}

pub async fn revenue_geographic_segmentation(
  http: &FmpHttpClient,
  params: SegmentationParams,
) -> FmpResult<Vec<RevenueGeographicSegmentation>> {
  http.get_json("/revenue-geographic-segmentation", &params).await
}

pub async fn income_statement_as_reported(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<AsReportedIncomeStatement>> {
  http.get_json("/income-statement-as-reported", &params).await
}

pub async fn balance_sheet_statement_as_reported(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<AsReportedBalanceSheet>> {
  http.get_json("/balance-sheet-statement-as-reported", &params).await
}

pub async fn cash_flow_statement_as_reported(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<AsReportedCashFlowStatement>> {
  http.get_json("/cash-flow-statement-as-reported", &params).await
}

pub async fn financial_statement_full_as_reported(
  http: &FmpHttpClient,
  params: StatementCommonParams,
) -> FmpResult<Vec<AsReportedFinancialStatement>> {
  http.get_json("/financial-statement-full-as-reported", &params).await
}

pub async fn key_metrics(http: &FmpHttpClient, params: StatementCommonParams) -> FmpResult<Vec<KeyMetrics>> {
  http.get_json("/key-metrics", &params).await
}

pub async fn ratios(http: &FmpHttpClient, params: StatementCommonParams) -> FmpResult<Vec<Ratios>> {
  http.get_json("/ratios", &params).await
}

pub async fn key_metrics_ttm(http: &FmpHttpClient, params: SymbolParam) -> FmpResult<Vec<KeyMetrics>> {
  http.get_json("/key-metrics-ttm", &params).await
}

pub async fn ratios_ttm(http: &FmpHttpClient, params: SymbolParam) -> FmpResult<Vec<Ratios>> {
  http.get_json("/ratios-ttm", &params).await
}

pub async fn financial_scores(http: &FmpHttpClient, params: FinancialScoresParams) -> FmpResult<Vec<FinancialScores>> {
  http.get_json("/financial-scores", &params).await
}

pub async fn owner_earnings(http: &FmpHttpClient, params: SymbolParam) -> FmpResult<Vec<OwnerEarnings>> {
  http.get_json("/owner-earnings", &params).await
}
