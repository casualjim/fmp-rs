use crate::macros::define_api_trait;
use crate::types::fund::{
  FundAssetExposure, FundCountryAllocation, FundDisclosure, FundDisclosureDate, FundDisclosureDatesParams,
  FundDisclosureHolder, FundDisclosureParams, FundDisclosureSearch, FundDisclosureSearchParams, FundHolding, FundInfo,
  FundSectorWeighting, FundSymbolParams,
};

define_api_trait!(
  /// API endpoints for fund data (ETFs and mutual funds).
  FundApi,
  etf_holdings -> "/etf/holdings" -> FundSymbolParams  -> Vec<FundHolding>,
  etf_info -> "/etf/info" -> FundSymbolParams  -> FundInfo,
  etf_country_weightings -> "/etf/country-weightings" -> FundSymbolParams  -> Vec<FundCountryAllocation>,
  etf_asset_exposure -> "/etf/asset-exposure" -> FundSymbolParams  -> Vec<FundAssetExposure>,
  etf_sector_weightings -> "/etf/sector-weightings" -> FundSymbolParams  -> Vec<FundSectorWeighting>,
  funds_disclosure_holders_latest -> "/funds/disclosure-holders-latest" -> FundSymbolParams  -> Vec<FundDisclosureHolder>,
  funds_disclosure_holders_search -> "/funds/disclosure-holders-search" -> FundDisclosureSearchParams  -> Vec<FundDisclosureSearch>,
  funds_disclosure_dates -> "/funds/disclosure-dates" -> FundDisclosureDatesParams  -> Vec<FundDisclosureDate>,
  funds_disclosure -> "/funds/disclosure" -> FundDisclosureParams  -> Vec<FundDisclosure>,
);
