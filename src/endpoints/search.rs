use crate::macros::define_api_trait;
use crate::types::search::{
  CikSearchParams, CusipSearchParams, ExchangeVariantResult, ExchangeVariantSearchParams, IsinSearchParams,
  NameSearchParams, StockScreenerParams, StockScreenerResult, SymbolSearchParams,
};

define_api_trait!(
  /// API endpoints for search.
  SearchApi,
  search_symbol -> "/search-symbol" -> SymbolSearchParams  -> Vec<crate::types::search::SymbolSearchResult>,
  search_name -> "/search-name" -> NameSearchParams  -> Vec<crate::types::search::NameSearchResult>,
  search_cik -> "/search-cik" -> CikSearchParams  -> Vec<crate::types::search::CikSearchResult>,
  search_cusip -> "/search-cusip" -> CusipSearchParams  -> Vec<crate::types::search::CusipSearchResult>,
  search_isin -> "/search-isin" -> IsinSearchParams  -> Vec<crate::types::search::IsinSearchResult>,
  company_screener -> "/company-screener" -> StockScreenerParams  -> Vec<StockScreenerResult>,
  search_exchange_variants -> "/search-exchange-variants" -> ExchangeVariantSearchParams  -> Vec<ExchangeVariantResult>,
);
