use crate::macros::define_csv_api_trait;
use crate::types::bulk::{
  BalanceSheetGrowth, BalanceSheetStatement, CashFlowGrowth, CashFlowStatement, CompanyProfile, DcfValuation,
  EarningsSurprise, EarningsSurpriseParams, EodData, EodParams, EtfHolder, FinancialScore, IncomeStatement,
  IncomeStatementGrowth, KeyMetricsTtm, PartParams, PriceTargetSummary, RatiosTtm, StockPeer, StockRating,
  UpgradesDowngradesConsensus, YearPeriodParams,
};

define_csv_api_trait!(
  /// Bulk CSV streaming endpoints for full-market datasets.
  BulkApi,
  profile -> "/profile-bulk" -> PartParams -> CompanyProfile,
  etf_holder -> "/etf-holder-bulk" -> PartParams -> EtfHolder,
  eod -> "/eod-bulk" -> EodParams -> EodData,
  earnings_surprises -> "/earnings-surprises-bulk" -> EarningsSurpriseParams -> EarningsSurprise,
  income_statement -> "/income-statement-bulk" -> YearPeriodParams -> IncomeStatement,
  balance_sheet_statement -> "/balance-sheet-statement-bulk" -> YearPeriodParams -> BalanceSheetStatement,
  cash_flow_statement -> "/cash-flow-statement-bulk" -> YearPeriodParams -> CashFlowStatement,
  income_statement_growth -> "/income-statement-growth-bulk" -> YearPeriodParams -> IncomeStatementGrowth,
  balance_sheet_statement_growth -> "/balance-sheet-statement-growth-bulk" -> YearPeriodParams -> BalanceSheetGrowth,
  cash_flow_statement_growth -> "/cash-flow-statement-growth-bulk" -> YearPeriodParams -> CashFlowGrowth,
  key_metrics_ttm -> "/key-metrics-ttm-bulk" -> () -> KeyMetricsTtm,
  ratios_ttm -> "/ratios-ttm-bulk" -> () -> RatiosTtm,
  scores -> "/scores-bulk" -> () -> FinancialScore,
  rating -> "/rating-bulk" -> () -> StockRating,
  upgrades_downgrades_consensus -> "/upgrades-downgrades-consensus-bulk" -> () -> UpgradesDowngradesConsensus,
  price_target_summary -> "/price-target-summary-bulk" -> () -> PriceTargetSummary,
  peers -> "/peers-bulk" -> () -> StockPeer,
  dcf -> "/dcf-bulk" -> () -> DcfValuation,
);
