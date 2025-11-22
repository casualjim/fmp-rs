use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::fundraisers::{
  CrowdfundingCampaign, CrowdfundingCikParams, CrowdfundingLatestParams, CrowdfundingSearchParams,
  CrowdfundingSearchResult, EquityCikParams, EquityLatestParams, EquityOffering, EquityOfferingSearchResult,
  EquitySearchParams,
};

pub async fn crowdfunding_offerings_latest(
  http: &FmpHttpClient,
  params: CrowdfundingLatestParams,
) -> FmpResult<Vec<CrowdfundingCampaign>> {
  http.get_json("/crowdfunding-offerings-latest", &params).await
}

pub async fn crowdfunding_offerings_search(
  http: &FmpHttpClient,
  params: CrowdfundingSearchParams,
) -> FmpResult<Vec<CrowdfundingSearchResult>> {
  http.get_json("/crowdfunding-offerings-search", &params).await
}

pub async fn crowdfunding_offerings(
  http: &FmpHttpClient,
  params: CrowdfundingCikParams,
) -> FmpResult<Vec<CrowdfundingCampaign>> {
  http.get_json("/crowdfunding-offerings", &params).await
}

pub async fn fundraising_latest(http: &FmpHttpClient, params: EquityLatestParams) -> FmpResult<Vec<EquityOffering>> {
  http.get_json("/fundraising-latest", &params).await
}

pub async fn fundraising_search(
  http: &FmpHttpClient,
  params: EquitySearchParams,
) -> FmpResult<Vec<EquityOfferingSearchResult>> {
  http.get_json("/fundraising-search", &params).await
}

pub async fn fundraising(http: &FmpHttpClient, params: EquityCikParams) -> FmpResult<Vec<EquityOffering>> {
  http.get_json("/fundraising", &params).await
}
