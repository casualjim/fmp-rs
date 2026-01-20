use crate::macros::define_api_trait;
use crate::types::company::{
  CikParams, CompanyExecutive, CompanyNote, CompanyProfile, DelistedCompany, EmployeeCount, ExecutiveCompensation,
  ExecutiveCompensationBenchmark, ExecutivesParams, MarketCap, MarketCapHistoryParams, MergerAcquisition,
  MnaSearchParams, PaginationParams, ShareFloat, ShareFloatAllParams, StockPeer, SymbolLimitParams, SymbolParams,
  SymbolsParams,
};

define_api_trait!(
  /// API endpoints for company.
  CompanyApi,
  profile -> "/profile" -> SymbolParams  -> Vec<CompanyProfile>,
  profile_cik -> "/profile-cik" -> CikParams  -> Vec<CompanyProfile>,
  company_notes -> "/company-notes" -> SymbolParams  -> Vec<CompanyNote>,
  stock_peers -> "/stock-peers" -> SymbolParams  -> Vec<StockPeer>,
  delisted_companies -> "/delisted-companies" -> PaginationParams  -> Vec<DelistedCompany>,
  employee_count -> "/employee-count" -> SymbolLimitParams  -> Vec<EmployeeCount>,
  historical_employee_count -> "/historical-employee-count" -> SymbolLimitParams  -> Vec<EmployeeCount>,
  market_capitalization -> "/market-capitalization" -> SymbolParams  -> Vec<MarketCap>,
  market_capitalization_batch -> "/market-capitalization-batch" -> SymbolsParams  -> Vec<MarketCap>,
  historical_market_capitalization -> "/historical-market-capitalization" -> MarketCapHistoryParams  -> Vec<MarketCap>,
  shares_float -> "/shares-float" -> SymbolParams  -> Vec<ShareFloat>,
  shares_float_all -> "/shares-float-all" -> ShareFloatAllParams  -> Vec<ShareFloat>,
  mergers_acquisitions_latest -> "/mergers-acquisitions-latest" -> PaginationParams  -> Vec<MergerAcquisition>,
  mergers_acquisitions_search -> "/mergers-acquisitions-search" -> MnaSearchParams  -> Vec<MergerAcquisition>,
  key_executives -> "/key-executives" -> ExecutivesParams  -> Vec<CompanyExecutive>,
  governance_executive_compensation -> "/governance-executive-compensation" -> SymbolParams  -> Vec<ExecutiveCompensation>,
  executive_compensation_benchmark -> "/executive-compensation-benchmark" -> SymbolParams  -> Vec<ExecutiveCompensationBenchmark>,
);
