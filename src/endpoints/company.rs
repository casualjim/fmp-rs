use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::company::{
  CikParams, CompanyExecutive, CompanyNote, CompanyProfile, DelistedCompany, EmployeeCount, ExecutiveCompensation,
  ExecutiveCompensationBenchmark, ExecutivesParams, MarketCap, MarketCapHistoryParams, MergerAcquisition,
  MnaSearchParams, PaginationParams, ShareFloat, ShareFloatAllParams, StockPeer, SymbolLimitParams, SymbolParams,
  SymbolsParams,
};

pub async fn profile(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<CompanyProfile>> {
  http.get_json("/profile", &params).await
}

pub async fn profile_cik(http: &FmpHttpClient, params: CikParams) -> FmpResult<Vec<CompanyProfile>> {
  http.get_json("/profile-cik", &params).await
}

pub async fn company_notes(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<CompanyNote>> {
  http.get_json("/company-notes", &params).await
}

pub async fn stock_peers(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<StockPeer>> {
  http.get_json("/stock-peers", &params).await
}

pub async fn delisted_companies(http: &FmpHttpClient, params: PaginationParams) -> FmpResult<Vec<DelistedCompany>> {
  http.get_json("/delisted-companies", &params).await
}

pub async fn employee_count(http: &FmpHttpClient, params: SymbolLimitParams) -> FmpResult<Vec<EmployeeCount>> {
  http.get_json("/employee-count", &params).await
}

pub async fn historical_employee_count(
  http: &FmpHttpClient,
  params: SymbolLimitParams,
) -> FmpResult<Vec<EmployeeCount>> {
  http.get_json("/historical-employee-count", &params).await
}

pub async fn market_capitalization(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<MarketCap>> {
  http.get_json("/market-capitalization", &params).await
}

pub async fn market_capitalization_batch(http: &FmpHttpClient, params: SymbolsParams) -> FmpResult<Vec<MarketCap>> {
  http.get_json("/market-capitalization-batch", &params).await
}

pub async fn historical_market_capitalization(
  http: &FmpHttpClient,
  params: MarketCapHistoryParams,
) -> FmpResult<Vec<MarketCap>> {
  http.get_json("/historical-market-capitalization", &params).await
}

pub async fn shares_float(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<ShareFloat>> {
  http.get_json("/shares-float", &params).await
}

pub async fn shares_float_all(http: &FmpHttpClient, params: ShareFloatAllParams) -> FmpResult<Vec<ShareFloat>> {
  http.get_json("/shares-float-all", &params).await
}

pub async fn mergers_acquisitions_latest(
  http: &FmpHttpClient,
  params: PaginationParams,
) -> FmpResult<Vec<MergerAcquisition>> {
  http.get_json("/mergers-acquisitions-latest", &params).await
}

pub async fn mergers_acquisitions_search(
  http: &FmpHttpClient,
  params: MnaSearchParams,
) -> FmpResult<Vec<MergerAcquisition>> {
  http.get_json("/mergers-acquisitions-search", &params).await
}

pub async fn key_executives(http: &FmpHttpClient, params: ExecutivesParams) -> FmpResult<Vec<CompanyExecutive>> {
  http.get_json("/key-executives", &params).await
}

pub async fn governance_executive_compensation(
  http: &FmpHttpClient,
  params: SymbolParams,
) -> FmpResult<Vec<ExecutiveCompensation>> {
  http.get_json("/governance-executive-compensation", &params).await
}

pub async fn executive_compensation_benchmark(
  http: &FmpHttpClient,
  params: SymbolParams,
) -> FmpResult<Vec<ExecutiveCompensationBenchmark>> {
  http.get_json("/executive-compensation-benchmark", &params).await
}
