use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::market_performance::{
  IndustryHistoryParams, IndustryPe, IndustryPerformance, IndustrySnapshotParams, SectorHistoryParams, SectorPe,
  SectorPerformance, SectorSnapshotParams, StockMovement,
};

pub async fn sector_performance_snapshot(
  http: &FmpHttpClient,
  params: SectorSnapshotParams,
) -> FmpResult<Vec<SectorPerformance>> {
  http.get_json("/sector-performance-snapshot", &params).await
}

pub async fn industry_performance_snapshot(
  http: &FmpHttpClient,
  params: IndustrySnapshotParams,
) -> FmpResult<Vec<IndustryPerformance>> {
  http.get_json("/industry-performance-snapshot", &params).await
}

pub async fn historical_sector_performance(
  http: &FmpHttpClient,
  params: SectorHistoryParams,
) -> FmpResult<Vec<SectorPerformance>> {
  http.get_json("/historical-sector-performance", &params).await
}

pub async fn historical_industry_performance(
  http: &FmpHttpClient,
  params: IndustryHistoryParams,
) -> FmpResult<Vec<IndustryPerformance>> {
  http.get_json("/historical-industry-performance", &params).await
}

pub async fn sector_pe_snapshot(http: &FmpHttpClient, params: SectorSnapshotParams) -> FmpResult<Vec<SectorPe>> {
  http.get_json("/sector-pe-snapshot", &params).await
}

pub async fn industry_pe_snapshot(http: &FmpHttpClient, params: IndustrySnapshotParams) -> FmpResult<Vec<IndustryPe>> {
  http.get_json("/industry-pe-snapshot", &params).await
}

pub async fn historical_sector_pe(http: &FmpHttpClient, params: SectorHistoryParams) -> FmpResult<Vec<SectorPe>> {
  http.get_json("/historical-sector-pe", &params).await
}

pub async fn historical_industry_pe(http: &FmpHttpClient, params: IndustryHistoryParams) -> FmpResult<Vec<IndustryPe>> {
  http.get_json("/historical-industry-pe", &params).await
}

pub async fn biggest_gainers(http: &FmpHttpClient) -> FmpResult<Vec<StockMovement>> {
  http.get_json("/biggest-gainers", &()).await
}

pub async fn biggest_losers(http: &FmpHttpClient) -> FmpResult<Vec<StockMovement>> {
  http.get_json("/biggest-losers", &()).await
}

pub async fn most_actives(http: &FmpHttpClient) -> FmpResult<Vec<StockMovement>> {
  http.get_json("/most-actives", &()).await
}
