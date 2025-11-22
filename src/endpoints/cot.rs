use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::cot::{CotAnalysis, CotList, CotRangeParams, CotReport};

pub async fn commitment_of_traders_report(http: &FmpHttpClient, params: CotRangeParams) -> FmpResult<Vec<CotReport>> {
  http.get_json("/commitment-of-traders-report", &params).await
}

pub async fn commitment_of_traders_analysis(
  http: &FmpHttpClient,
  params: CotRangeParams,
) -> FmpResult<Vec<CotAnalysis>> {
  http.get_json("/commitment-of-traders-analysis", &params).await
}

pub async fn commitment_of_traders_list(http: &FmpHttpClient) -> FmpResult<Vec<CotList>> {
  http.get_json("/commitment-of-traders-list", &()).await
}
