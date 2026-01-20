use crate::macros::define_api_trait;
use crate::types::cot::{CotAnalysis, CotRangeParams, CotReport};

define_api_trait!(
  /// API endpoints for cot.
  CotApi,
  commitment_of_traders_report -> "/commitment-of-traders-report" -> CotRangeParams  -> Vec<CotReport>,
  commitment_of_traders_analysis -> "/commitment-of-traders-analysis" -> CotRangeParams  -> Vec<CotAnalysis>,
);
