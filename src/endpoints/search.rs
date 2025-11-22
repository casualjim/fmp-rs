use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::search::{
  CikSearchParams, CusipSearchParams, ExchangeVariantResult, ExchangeVariantSearchParams, IsinSearchParams,
  NameSearchParams, StockScreenerParams, StockScreenerResult, SymbolSearchParams,
};

pub async fn search_symbol(
  http: &FmpHttpClient,
  params: SymbolSearchParams,
) -> FmpResult<Vec<crate::types::search::SymbolSearchResult>> {
  http.get_json("/search-symbol", &params).await
}

pub async fn search_name(
  http: &FmpHttpClient,
  params: NameSearchParams,
) -> FmpResult<Vec<crate::types::search::NameSearchResult>> {
  http.get_json("/search-name", &params).await
}

pub async fn search_cik(
  http: &FmpHttpClient,
  params: CikSearchParams,
) -> FmpResult<Vec<crate::types::search::CikSearchResult>> {
  http.get_json("/search-cik", &params).await
}

pub async fn search_cusip(
  http: &FmpHttpClient,
  params: CusipSearchParams,
) -> FmpResult<Vec<crate::types::search::CusipSearchResult>> {
  http.get_json("/search-cusip", &params).await
}

pub async fn search_isin(
  http: &FmpHttpClient,
  params: IsinSearchParams,
) -> FmpResult<Vec<crate::types::search::IsinSearchResult>> {
  http.get_json("/search-isin", &params).await
}

pub async fn company_screener(
  http: &FmpHttpClient,
  params: StockScreenerParams,
) -> FmpResult<Vec<StockScreenerResult>> {
  http.get_json("/company-screener", &params).await
}

pub async fn search_exchange_variants(
  http: &FmpHttpClient,
  params: ExchangeVariantSearchParams,
) -> FmpResult<Vec<ExchangeVariantResult>> {
  http.get_json("/search-exchange-variants", &params).await
}
