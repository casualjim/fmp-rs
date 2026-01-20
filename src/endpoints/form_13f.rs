use crate::macros::define_api_trait;
use crate::types::form_13f::{
  FilingDatesParams, FilingExtractAnalytics, FilingExtractAnalyticsByHolderParams, FilingExtractParams,
  Form13fFilingDate, HolderIndustryBreakdown, HolderIndustryBreakdownParams, HolderPerformanceParams,
  HolderPerformanceSummary, IndustryPerformanceParams, IndustryPerformanceSummary, InstitutionalOwnershipFiling,
  PaginationParams, PositionsSummary, PositionsSummaryParams, SecFilingExtract,
};

define_api_trait!(
  /// API endpoints for Form 13F institutional ownership data.
  Form13FApi,
  institutional_ownership_latest -> "/institutional-ownership/latest" -> PaginationParams  -> Vec<InstitutionalOwnershipFiling>,
  institutional_ownership_extract -> "/institutional-ownership/extract" -> FilingExtractParams  -> Vec<SecFilingExtract>,
  institutional_ownership_dates -> "/institutional-ownership/dates" -> FilingDatesParams  -> Vec<Form13fFilingDate>,
  institutional_ownership_extract_analytics_holder -> "/institutional-ownership/extract-analytics/holder" -> FilingExtractAnalyticsByHolderParams  -> Vec<FilingExtractAnalytics>,
  institutional_ownership_holder_performance_summary -> "/institutional-ownership/holder-performance-summary" -> HolderPerformanceParams  -> Vec<HolderPerformanceSummary>,
  institutional_ownership_holder_industry_breakdown -> "/institutional-ownership/holder-industry-breakdown" -> HolderIndustryBreakdownParams  -> Vec<HolderIndustryBreakdown>,
  institutional_ownership_symbol_positions_summary -> "/institutional-ownership/symbol-positions-summary" -> PositionsSummaryParams  -> Vec<PositionsSummary>,
  institutional_ownership_industry_summary -> "/institutional-ownership/industry-summary" -> IndustryPerformanceParams  -> Vec<IndustryPerformanceSummary>,
);
