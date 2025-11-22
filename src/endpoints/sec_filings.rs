use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::sec_filings::{
  AllIndustryClassificationParams, CikParams, CompanyCikSearchParams, CompanyNameSearchParams, CompanyProfileParams,
  CompanySearchResult, CompanySymbolSearchParams, DateRangeParams, Form8kParams, FormTypeParams,
  IndustryClassification, IndustryClassificationSearchParams, IndustrySearchParams, SecCompanyProfile, SecFiling,
  SymbolParams,
};

pub async fn latest_8k_filings(http: &FmpHttpClient, params: Form8kParams) -> FmpResult<Vec<SecFiling>> {
  http.get_json("/sec-filings-8k", &params).await
}

pub async fn latest_financial_filings(http: &FmpHttpClient, params: DateRangeParams) -> FmpResult<Vec<SecFiling>> {
  http.get_json("/sec-filings-financials", &params).await
}

pub async fn filings_by_form_type(http: &FmpHttpClient, params: FormTypeParams) -> FmpResult<Vec<SecFiling>> {
  http.get_json("/sec-filings-search/form-type", &params).await
}

pub async fn filings_by_symbol(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<SecFiling>> {
  http.get_json("/sec-filings-search/symbol", &params).await
}

pub async fn filings_by_cik(http: &FmpHttpClient, params: CikParams) -> FmpResult<Vec<SecFiling>> {
  http.get_json("/sec-filings-search/cik", &params).await
}

pub async fn company_search_by_name(
  http: &FmpHttpClient,
  params: CompanyNameSearchParams,
) -> FmpResult<Vec<CompanySearchResult>> {
  http.get_json("/sec-filings-company-search/name", &params).await
}

pub async fn company_search_by_symbol(
  http: &FmpHttpClient,
  params: CompanySymbolSearchParams,
) -> FmpResult<Vec<CompanySearchResult>> {
  http.get_json("/sec-filings-company-search/symbol", &params).await
}

pub async fn company_search_by_cik(
  http: &FmpHttpClient,
  params: CompanyCikSearchParams,
) -> FmpResult<Vec<CompanySearchResult>> {
  http.get_json("/sec-filings-company-search/cik", &params).await
}

pub async fn sec_profile(http: &FmpHttpClient, params: CompanyProfileParams) -> FmpResult<Vec<SecCompanyProfile>> {
  http.get_json("/sec-profile", &params).await
}

pub async fn industry_classification_list(
  http: &FmpHttpClient,
  params: IndustrySearchParams,
) -> FmpResult<Vec<IndustryClassification>> {
  http.get_json("/standard-industrial-classification-list", &params).await
}

pub async fn industry_classification_search(
  http: &FmpHttpClient,
  params: IndustryClassificationSearchParams,
) -> FmpResult<Vec<CompanySearchResult>> {
  http.get_json("/industry-classification-search", &params).await
}

pub async fn all_industry_classification(
  http: &FmpHttpClient,
  params: AllIndustryClassificationParams,
) -> FmpResult<Vec<CompanySearchResult>> {
  http.get_json("/all-industry-classification", &params).await
}
