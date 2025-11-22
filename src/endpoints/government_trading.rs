use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::government_trading::{FinancialDisclosure, NameParams, PaginationParams, SymbolParams};

pub async fn senate_latest(http: &FmpHttpClient, params: PaginationParams) -> FmpResult<Vec<FinancialDisclosure>> {
  http.get_json("/senate-latest", &params).await
}

pub async fn house_latest(http: &FmpHttpClient, params: PaginationParams) -> FmpResult<Vec<FinancialDisclosure>> {
  http.get_json("/house-latest", &params).await
}

pub async fn senate_trades(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<FinancialDisclosure>> {
  http.get_json("/senate-trades", &params).await
}

pub async fn senate_trades_by_name(http: &FmpHttpClient, params: NameParams) -> FmpResult<Vec<FinancialDisclosure>> {
  http.get_json("/senate-trades-by-name", &params).await
}

pub async fn house_trades(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<FinancialDisclosure>> {
  http.get_json("/house-trades", &params).await
}

pub async fn house_trades_by_name(http: &FmpHttpClient, params: NameParams) -> FmpResult<Vec<FinancialDisclosure>> {
  http.get_json("/house-trades-by-name", &params).await
}
