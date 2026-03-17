use crate::macros::define_api_trait;
use crate::types::statements::{
  AsReportedBalanceSheet, AsReportedCashFlowStatement, AsReportedFinancialStatement, AsReportedIncomeStatement,
  BalanceSheetStatement, BalanceSheetStatementGrowth, CashFlowStatement, CashFlowStatementGrowth, FinancialReport10K,
  FinancialReportDate, FinancialReportParams, FinancialScores, FinancialScoresParams, FinancialStatementGrowth,
  IncomeStatement, IncomeStatementGrowth, KeyMetrics, KeyMetricsTtm, LatestFinancialStatement, OwnerEarnings, Ratios,
  RevenueGeographicSegmentation, RevenueProductSegmentation, SegmentationParams, StatementCommonParams,
  StatementLimitParams, StatementPaginationParams, SymbolParam,
};

define_api_trait!(
  /// API endpoints for statements.
  StatementsApi,
  income_statement -> "/income-statement" -> StatementCommonParams  -> Vec<IncomeStatement>,
  balance_sheet_statement -> "/balance-sheet-statement" -> StatementCommonParams  -> Vec<BalanceSheetStatement>,
  cash_flow_statement -> "/cash-flow-statement" -> StatementCommonParams  -> Vec<CashFlowStatement>,
  latest_financial_statements -> "/latest-financial-statements" -> StatementPaginationParams  -> Vec<LatestFinancialStatement>,
  income_statement_ttm -> "/income-statement-ttm" -> StatementLimitParams  -> Vec<IncomeStatement>,
  balance_sheet_statement_ttm -> "/balance-sheet-statement-ttm" -> StatementLimitParams  -> Vec<BalanceSheetStatement>,
  cash_flow_statement_ttm -> "/cash-flow-statement-ttm" -> StatementLimitParams  -> Vec<CashFlowStatement>,
  income_statement_growth -> "/income-statement-growth" -> StatementCommonParams  -> Vec<IncomeStatementGrowth>,
  balance_sheet_statement_growth -> "/balance-sheet-statement-growth" -> StatementCommonParams  -> Vec<BalanceSheetStatementGrowth>,
  cash_flow_statement_growth -> "/cash-flow-statement-growth" -> StatementCommonParams  -> Vec<CashFlowStatementGrowth>,
  financial_growth -> "/financial-growth" -> StatementCommonParams  -> Vec<FinancialStatementGrowth>,
  financial_reports_dates -> "/financial-reports-dates" -> SymbolParam  -> Vec<FinancialReportDate>,
  financial_reports_json -> "/financial-reports-json" -> FinancialReportParams  -> Vec<FinancialReport10K>,
  revenue_product_segmentation -> "/revenue-product-segmentation" -> SegmentationParams  -> Vec<RevenueProductSegmentation>,
  revenue_geographic_segmentation -> "/revenue-geographic-segmentation" -> SegmentationParams  -> Vec<RevenueGeographicSegmentation>,
  income_statement_as_reported -> "/income-statement-as-reported" -> StatementCommonParams  -> Vec<AsReportedIncomeStatement>,
  balance_sheet_statement_as_reported -> "/balance-sheet-statement-as-reported" -> StatementCommonParams  -> Vec<AsReportedBalanceSheet>,
  cash_flow_statement_as_reported -> "/cash-flow-statement-as-reported" -> StatementCommonParams  -> Vec<AsReportedCashFlowStatement>,
  financial_statement_full_as_reported -> "/financial-statement-full-as-reported" -> StatementCommonParams  -> Vec<AsReportedFinancialStatement>,
  key_metrics -> "/key-metrics" -> StatementCommonParams  -> Vec<KeyMetrics>,
  ratios -> "/ratios" -> StatementCommonParams  -> Vec<Ratios>,
  key_metrics_ttm -> "/key-metrics-ttm" -> SymbolParam  -> Vec<KeyMetricsTtm>,
  ratios_ttm -> "/ratios-ttm" -> SymbolParam  -> Vec<Ratios>,
  financial_scores -> "/financial-scores" -> FinancialScoresParams  -> Vec<FinancialScores>,
  owner_earnings -> "/owner-earnings" -> SymbolParam  -> Vec<OwnerEarnings>,
);
