use crate::macros::define_api_trait;
use crate::types::insider_trades::{
  AcquisitionOwnership, AcquisitionOwnershipParams, InsiderLatestParams, InsiderReportingName, InsiderSearchParams,
  InsiderSymbolParams, InsiderTradeStatistics, InsiderTrading, InsiderTransactionType, ReportingNameParams,
};

define_api_trait!(
  /// API endpoints for insider_trades.
  InsiderTradesApi,
  insider_trading_latest -> "/insider-trading/latest" -> InsiderLatestParams  -> Vec<InsiderTrading>,
  insider_trading_search -> "/insider-trading/search" -> InsiderSearchParams  -> Vec<InsiderTrading>,
  insider_trading_reporting_name -> "/insider-trading/reporting-name" -> ReportingNameParams  -> Vec<InsiderReportingName>,
  insider_trading_transaction_type -> "/insider-trading-transaction-type" -> InsiderSymbolParams  -> Vec<InsiderTransactionType>,
  insider_trading_statistics -> "/insider-trading/statistics" -> InsiderSymbolParams  -> Vec<InsiderTradeStatistics>,
  acquisition_of_beneficial_ownership -> "/acquisition-of-beneficial-ownership" -> AcquisitionOwnershipParams  -> Vec<AcquisitionOwnership>,
);
