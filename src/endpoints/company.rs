use crate::macros::define_api_trait;
use crate::types::company::{
  CikParams, CompanyExecutive, CompanyNote, CompanyProfile, DelistedCompany, EmployeeCount, ExecutiveCompensation,
  ExecutiveCompensationBenchmark, ExecutivesParams, MarketCap, MarketCapHistoryParams, MergerAcquisition,
  MnaSearchParams, PaginationParams, ShareFloat, ShareFloatAllParams, StockPeer, SymbolLimitParams, SymbolParams,
  SymbolsParams,
};

define_api_trait!(
  /// Company profile, market cap, employees, share float, executives, and M&A data.
  ///
  /// # Profile endpoints
  /// - `profile` — full company profile by ticker symbol (sector, industry, CEO, description, financials summary)
  /// - `profile_cik` — same profile lookup using SEC CIK number instead of ticker
  /// - `company_notes` — SEC-registered notes/bonds issued by the company
  /// - `stock_peers` — peer companies in the same sector and market-cap tier
  ///
  /// # Market capitalisation
  /// - `market_capitalization` — current market cap for a single symbol
  /// - `market_capitalization_batch` — current market cap for multiple symbols in one request
  /// - `historical_market_capitalization` — daily historical market cap with optional date range
  ///
  /// # Workforce
  /// - `employee_count` — current full-time employee count from the latest SEC filing
  /// - `historical_employee_count` — historical employee counts from prior filings
  ///
  /// # Ownership and float
  /// - `shares_float` — public float (tradeable shares) for a symbol
  /// - `shares_float_all` — share float for all companies (paginated)
  ///
  /// # Mergers and acquisitions
  /// - `mergers_acquisitions_latest` — most recent M&A announcements (paginated)
  /// - `mergers_acquisitions_search` — search M&A events by company name
  ///
  /// # Executive data
  /// - `key_executives` — current (and former) executives with titles and compensation
  /// - `governance_executive_compensation` — detailed NEO compensation from proxy statements
  /// - `executive_compensation_benchmark` — industry-average compensation benchmarks by year
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
