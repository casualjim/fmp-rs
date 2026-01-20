use crate::macros::define_api_trait;
use crate::types::fundraisers::{
  CrowdfundingCampaign, CrowdfundingCikParams, CrowdfundingLatestParams, CrowdfundingSearchParams,
  CrowdfundingSearchResult, EquityCikParams, EquityLatestParams, EquityOffering, EquityOfferingSearchResult,
  EquitySearchParams,
};

define_api_trait!(
  /// API endpoints for fundraisers.
  FundraisersApi,
  crowdfunding_offerings_latest -> "/crowdfunding-offerings-latest" -> CrowdfundingLatestParams  -> Vec<CrowdfundingCampaign>,
  crowdfunding_offerings_search -> "/crowdfunding-offerings-search" -> CrowdfundingSearchParams  -> Vec<CrowdfundingSearchResult>,
  crowdfunding_offerings -> "/crowdfunding-offerings" -> CrowdfundingCikParams  -> Vec<CrowdfundingCampaign>,
  fundraising_latest -> "/fundraising-latest" -> EquityLatestParams  -> Vec<EquityOffering>,
  fundraising_search -> "/fundraising-search" -> EquitySearchParams  -> Vec<EquityOfferingSearchResult>,
  fundraising -> "/fundraising" -> EquityCikParams  -> Vec<EquityOffering>,
);
