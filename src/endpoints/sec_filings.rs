use crate::macros::define_api_trait;
use crate::types::sec_filings::{
  AllIndustryClassificationParams, CikParams, CompanyCikSearchParams, CompanyNameSearchParams, CompanyProfileParams,
  CompanySearchResult, CompanySymbolSearchParams, DateRangeParams, Form8kParams, FormTypeParams,
  IndustryClassification, IndustryClassificationSearchParams, IndustrySearchParams, SecCompanyProfile, SecFiling,
  SymbolParams,
};

define_api_trait!(
  /// API endpoints for sec_filings.
  SecFilingsApi,
  latest_8k_filings -> "/sec-filings-8k" -> Form8kParams  -> Vec<SecFiling>,
  latest_financial_filings -> "/sec-filings-financials" -> DateRangeParams  -> Vec<SecFiling>,
  filings_by_form_type -> "/sec-filings-search/form-type" -> FormTypeParams  -> Vec<SecFiling>,
  filings_by_symbol -> "/sec-filings-search/symbol" -> SymbolParams  -> Vec<SecFiling>,
  filings_by_cik -> "/sec-filings-search/cik" -> CikParams  -> Vec<SecFiling>,
  company_search_by_name -> "/sec-filings-company-search/name" -> CompanyNameSearchParams  -> Vec<CompanySearchResult>,
  company_search_by_symbol -> "/sec-filings-company-search/symbol" -> CompanySymbolSearchParams  -> Vec<CompanySearchResult>,
  company_search_by_cik -> "/sec-filings-company-search/cik" -> CompanyCikSearchParams  -> Vec<CompanySearchResult>,
  sec_profile -> "/sec-profile" -> CompanyProfileParams  -> Vec<SecCompanyProfile>,
  industry_classification_list -> "/standard-industrial-classification-list" -> IndustrySearchParams  -> Vec<IndustryClassification>,
  industry_classification_search -> "/industry-classification-search" -> IndustryClassificationSearchParams  -> Vec<CompanySearchResult>,
  all_industry_classification -> "/all-industry-classification" -> AllIndustryClassificationParams  -> Vec<CompanySearchResult>,
);
