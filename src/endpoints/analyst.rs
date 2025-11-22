use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::analyst::{
  AnalystEstimate, AnalystEstimatesParams, HistoricalRating, HistoricalStockGrade, PageParams, PriceTargetConsensus,
  PriceTargetNews, PriceTargetSummary, RatingsSnapshot, StockGrade, StockGradeNews, StockGradeSummary,
  SymbolLimitParams, SymbolPageParams,
};

pub async fn analyst_estimates(
  http: &FmpHttpClient,
  params: AnalystEstimatesParams,
) -> FmpResult<Vec<AnalystEstimate>> {
  http.get_json("/analyst-estimates", &params).await
}

pub async fn ratings_snapshot(http: &FmpHttpClient, params: SymbolLimitParams) -> FmpResult<Vec<RatingsSnapshot>> {
  http.get_json("/ratings-snapshot", &params).await
}

pub async fn ratings_historical(http: &FmpHttpClient, params: SymbolLimitParams) -> FmpResult<Vec<HistoricalRating>> {
  http.get_json("/ratings-historical", &params).await
}

pub async fn price_target_summary(
  http: &FmpHttpClient,
  params: SymbolLimitParams,
) -> FmpResult<Vec<PriceTargetSummary>> {
  http.get_json("/price-target-summary", &params).await
}

pub async fn price_target_consensus(
  http: &FmpHttpClient,
  params: SymbolLimitParams,
) -> FmpResult<Vec<PriceTargetConsensus>> {
  http.get_json("/price-target-consensus", &params).await
}

pub async fn price_target_news(http: &FmpHttpClient, params: SymbolPageParams) -> FmpResult<Vec<PriceTargetNews>> {
  http.get_json("/price-target-news", &params).await
}

pub async fn price_target_latest_news(http: &FmpHttpClient, params: PageParams) -> FmpResult<Vec<PriceTargetNews>> {
  http.get_json("/price-target-latest-news", &params).await
}

pub async fn grades(http: &FmpHttpClient, params: SymbolLimitParams) -> FmpResult<Vec<StockGrade>> {
  http.get_json("/grades", &params).await
}

pub async fn grades_historical(
  http: &FmpHttpClient,
  params: SymbolLimitParams,
) -> FmpResult<Vec<HistoricalStockGrade>> {
  http.get_json("/grades-historical", &params).await
}

pub async fn grades_consensus(http: &FmpHttpClient, params: SymbolLimitParams) -> FmpResult<Vec<StockGradeSummary>> {
  http.get_json("/grades-consensus", &params).await
}

pub async fn grades_news(http: &FmpHttpClient, params: SymbolPageParams) -> FmpResult<Vec<StockGradeNews>> {
  http.get_json("/grades-news", &params).await
}

pub async fn grades_latest_news(http: &FmpHttpClient, params: PageParams) -> FmpResult<Vec<StockGradeNews>> {
  http.get_json("/grades-latest-news", &params).await
}
