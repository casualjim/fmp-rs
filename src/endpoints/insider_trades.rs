use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::insider_trades::{
  AcquisitionOwnership, AcquisitionOwnershipParams, InsiderLatestParams, InsiderReportingName, InsiderSearchParams,
  InsiderSymbolParams, InsiderTradeStatistics, InsiderTrading, InsiderTransactionType, ReportingNameParams,
};

pub async fn insider_trading_latest(
  http: &FmpHttpClient,
  params: InsiderLatestParams,
) -> FmpResult<Vec<InsiderTrading>> {
  http.get_json("/insider-trading/latest", &params).await
}

pub async fn insider_trading_search(
  http: &FmpHttpClient,
  params: InsiderSearchParams,
) -> FmpResult<Vec<InsiderTrading>> {
  http.get_json("/insider-trading/search", &params).await
}

pub async fn insider_trading_reporting_name(
  http: &FmpHttpClient,
  params: ReportingNameParams,
) -> FmpResult<Vec<InsiderReportingName>> {
  http.get_json("/insider-trading/reporting-name", &params).await
}

pub async fn insider_trading_transaction_type(http: &FmpHttpClient) -> FmpResult<Vec<InsiderTransactionType>> {
  http.get_json("/insider-trading-transaction-type", &()).await
}

pub async fn insider_trading_statistics(
  http: &FmpHttpClient,
  params: InsiderSymbolParams,
) -> FmpResult<Vec<InsiderTradeStatistics>> {
  http.get_json("/insider-trading/statistics", &params).await
}

pub async fn acquisition_of_beneficial_ownership(
  http: &FmpHttpClient,
  params: AcquisitionOwnershipParams,
) -> FmpResult<Vec<AcquisitionOwnership>> {
  http.get_json("/acquisition-of-beneficial-ownership", &params).await
}
