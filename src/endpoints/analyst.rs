use crate::macros::define_api_trait;
use crate::types::analyst::{
  AnalystEstimate, AnalystEstimatesParams, HistoricalRating, HistoricalStockGrade, PageParams, PriceTargetConsensus,
  PriceTargetNews, PriceTargetSummary, RatingsSnapshot, StockGrade, StockGradeNews, StockGradeSummary,
  SymbolLimitParams, SymbolPageParams,
};

define_api_trait!(
  /// API endpoints for analyst.
  AnalystApi,
  analyst_estimates -> "/analyst-estimates" -> AnalystEstimatesParams  -> Vec<AnalystEstimate>,
  ratings_snapshot -> "/ratings-snapshot" -> SymbolLimitParams  -> Vec<RatingsSnapshot>,
  ratings_historical -> "/ratings-historical" -> SymbolLimitParams  -> Vec<HistoricalRating>,
  price_target_summary -> "/price-target-summary" -> SymbolLimitParams  -> Vec<PriceTargetSummary>,
  price_target_consensus -> "/price-target-consensus" -> SymbolLimitParams  -> Vec<PriceTargetConsensus>,
  price_target_news -> "/price-target-news" -> SymbolPageParams  -> Vec<PriceTargetNews>,
  price_target_latest_news -> "/price-target-latest-news" -> PageParams  -> Vec<PriceTargetNews>,
  grades -> "/grades" -> SymbolLimitParams  -> Vec<StockGrade>,
  grades_historical -> "/grades-historical" -> SymbolLimitParams  -> Vec<HistoricalStockGrade>,
  grades_consensus -> "/grades-consensus" -> SymbolLimitParams  -> Vec<StockGradeSummary>,
  grades_news -> "/grades-news" -> SymbolPageParams  -> Vec<StockGradeNews>,
  grades_latest_news -> "/grades-latest-news" -> PageParams  -> Vec<StockGradeNews>,
);
