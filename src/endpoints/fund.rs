use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::fund::{
  FundAssetExposure, FundCountryAllocation, FundDisclosure, FundDisclosureDate, FundDisclosureDatesParams,
  FundDisclosureHolder, FundDisclosureParams, FundDisclosureSearch, FundDisclosureSearchParams, FundHolding, FundInfo,
  FundSectorWeighting, FundSymbolParams,
};

pub async fn etf_holdings(http: &FmpHttpClient, params: FundSymbolParams) -> FmpResult<Vec<FundHolding>> {
  http.get_json("/etf/holdings", &params).await
}

pub async fn etf_info(http: &FmpHttpClient, params: FundSymbolParams) -> FmpResult<FundInfo> {
  http.get_json("/etf/info", &params).await
}

pub async fn etf_country_weightings(
  http: &FmpHttpClient,
  params: FundSymbolParams,
) -> FmpResult<Vec<FundCountryAllocation>> {
  http.get_json("/etf/country-weightings", &params).await
}

pub async fn etf_asset_exposure(http: &FmpHttpClient, params: FundSymbolParams) -> FmpResult<Vec<FundAssetExposure>> {
  http.get_json("/etf/asset-exposure", &params).await
}

pub async fn etf_sector_weightings(
  http: &FmpHttpClient,
  params: FundSymbolParams,
) -> FmpResult<Vec<FundSectorWeighting>> {
  http.get_json("/etf/sector-weightings", &params).await
}

pub async fn funds_disclosure_holders_latest(
  http: &FmpHttpClient,
  params: FundSymbolParams,
) -> FmpResult<Vec<FundDisclosureHolder>> {
  http.get_json("/funds/disclosure-holders-latest", &params).await
}

pub async fn funds_disclosure_holders_search(
  http: &FmpHttpClient,
  params: FundDisclosureSearchParams,
) -> FmpResult<Vec<FundDisclosureSearch>> {
  http.get_json("/funds/disclosure-holders-search", &params).await
}

pub async fn funds_disclosure_dates(
  http: &FmpHttpClient,
  params: FundDisclosureDatesParams,
) -> FmpResult<Vec<FundDisclosureDate>> {
  http.get_json("/funds/disclosure-dates", &params).await
}

pub async fn funds_disclosure(http: &FmpHttpClient, params: FundDisclosureParams) -> FmpResult<Vec<FundDisclosure>> {
  http.get_json("/funds/disclosure", &params).await
}
