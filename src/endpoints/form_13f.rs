use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::form_13f::FilingExtractAnalytics;
use crate::types::form_13f::Form13fFilingDate;
use crate::types::form_13f::{
  FilingDatesParams, FilingExtractAnalyticsByHolderParams, FilingExtractParams, HolderIndustryBreakdown,
  HolderIndustryBreakdownParams, HolderPerformanceParams, HolderPerformanceSummary, IndustryPerformanceParams,
  IndustryPerformanceSummary, InstitutionalOwnershipFiling, PaginationParams, PositionsSummary, PositionsSummaryParams,
  SecFilingExtract,
};

pub async fn institutional_ownership_latest(
  http: &FmpHttpClient,
  params: PaginationParams,
) -> FmpResult<Vec<InstitutionalOwnershipFiling>> {
  http.get_json("/institutional-ownership/latest", &params).await
}

pub async fn institutional_ownership_extract(
  http: &FmpHttpClient,
  params: FilingExtractParams,
) -> FmpResult<Vec<SecFilingExtract>> {
  http.get_json("/institutional-ownership/extract", &params).await
}

pub async fn institutional_ownership_dates(
  http: &FmpHttpClient,
  params: FilingDatesParams,
) -> FmpResult<Vec<Form13fFilingDate>> {
  http.get_json("/institutional-ownership/dates", &params).await
}

pub async fn institutional_ownership_extract_analytics_holder(
  http: &FmpHttpClient,
  params: FilingExtractAnalyticsByHolderParams,
) -> FmpResult<Vec<FilingExtractAnalytics>> {
  http
    .get_json("/institutional-ownership/extract-analytics/holder", &params)
    .await
}

pub async fn institutional_ownership_holder_performance_summary(
  http: &FmpHttpClient,
  params: HolderPerformanceParams,
) -> FmpResult<Vec<HolderPerformanceSummary>> {
  http
    .get_json("/institutional-ownership/holder-performance-summary", &params)
    .await
}

pub async fn institutional_ownership_holder_industry_breakdown(
  http: &FmpHttpClient,
  params: HolderIndustryBreakdownParams,
) -> FmpResult<Vec<HolderIndustryBreakdown>> {
  http
    .get_json("/institutional-ownership/holder-industry-breakdown", &params)
    .await
}

pub async fn institutional_ownership_symbol_positions_summary(
  http: &FmpHttpClient,
  params: PositionsSummaryParams,
) -> FmpResult<Vec<PositionsSummary>> {
  http
    .get_json("/institutional-ownership/symbol-positions-summary", &params)
    .await
}

pub async fn institutional_ownership_industry_summary(
  http: &FmpHttpClient,
  params: IndustryPerformanceParams,
) -> FmpResult<Vec<IndustryPerformanceSummary>> {
  http
    .get_json("/institutional-ownership/industry-summary", &params)
    .await
}
