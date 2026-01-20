use crate::macros::define_api_trait;
use crate::types::market_performance::{
  IndustryHistoryParams, IndustryPe, IndustryPerformance, IndustrySnapshotParams, SectorHistoryParams, SectorPe,
  SectorPerformance, SectorSnapshotParams, StockMovement,
};

define_api_trait!(
  /// API endpoints for market_performance.
  MarketPerformanceApi,
  sector_performance_snapshot -> "/sector-performance-snapshot" -> SectorSnapshotParams  -> Vec<SectorPerformance>,
  industry_performance_snapshot -> "/industry-performance-snapshot" -> IndustrySnapshotParams  -> Vec<IndustryPerformance>,
  historical_sector_performance -> "/historical-sector-performance" -> SectorHistoryParams  -> Vec<SectorPerformance>,
  historical_industry_performance -> "/historical-industry-performance" -> IndustryHistoryParams  -> Vec<IndustryPerformance>,
  sector_pe_snapshot -> "/sector-pe-snapshot" -> SectorSnapshotParams  -> Vec<SectorPe>,
  industry_pe_snapshot -> "/industry-pe-snapshot" -> IndustrySnapshotParams  -> Vec<IndustryPe>,
  historical_sector_pe -> "/historical-sector-pe" -> SectorHistoryParams  -> Vec<SectorPe>,
  historical_industry_pe -> "/historical-industry-pe" -> IndustryHistoryParams  -> Vec<IndustryPe>,
  biggest_gainers -> "/biggest-gainers" -> () -> Vec<StockMovement>,
  biggest_losers -> "/biggest-losers" -> () -> Vec<StockMovement>,
  most_actives -> "/most-actives" -> () -> Vec<StockMovement>,
);
