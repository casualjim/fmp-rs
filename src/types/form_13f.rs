use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalOwnershipFiling {
  pub cik: String,
  pub name: String,
  pub date: FmpDate,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub form_type: String,
  pub link: String,
  pub final_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecFilingExtract {
  pub date: FmpDate,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub cik: String,
  pub security_cusip: String,
  pub symbol: String,
  pub name_of_issuer: String,
  #[serde(default)]
  pub shares: Option<f64>,
  pub title_of_class: String,
  pub shares_type: String,
  pub put_call_share: String,
  #[serde(default)]
  pub value: Option<f64>,
  pub link: String,
  pub final_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form13fFilingDate {
  pub date: FmpDate,
  pub year: i32,
  pub quarter: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilingExtractAnalytics {
  pub date: FmpDate,
  pub cik: String,
  pub filing_date: FmpDate,
  pub investor_name: String,
  pub symbol: String,
  pub security_name: String,
  pub type_of_security: String,
  pub security_cusip: String,
  pub shares_type: String,
  pub put_call_share: String,
  pub investment_discretion: String,
  pub industry_title: String,
  #[serde(default)]
  pub weight: Option<f64>,
  #[serde(default)]
  pub last_weight: Option<f64>,
  #[serde(default)]
  pub change_in_weight: Option<f64>,
  #[serde(default)]
  pub change_in_weight_percentage: Option<f64>,
  #[serde(default)]
  pub market_value: Option<f64>,
  #[serde(default)]
  pub last_market_value: Option<f64>,
  #[serde(default)]
  pub change_in_market_value: Option<f64>,
  #[serde(default)]
  pub change_in_market_value_percentage: Option<f64>,
  #[serde(default)]
  pub shares_number: Option<f64>,
  #[serde(default)]
  pub last_shares_number: Option<f64>,
  #[serde(default)]
  pub change_in_shares_number: Option<f64>,
  #[serde(default)]
  pub change_in_shares_number_percentage: Option<f64>,
  #[serde(default)]
  pub quarter_end_price: Option<f64>,
  #[serde(default)]
  pub avg_price_paid: Option<f64>,
  pub is_new: bool,
  pub is_sold_out: bool,
  #[serde(default)]
  pub ownership: Option<f64>,
  #[serde(default)]
  pub last_ownership: Option<f64>,
  #[serde(default)]
  pub change_in_ownership: Option<f64>,
  #[serde(default)]
  pub change_in_ownership_percentage: Option<f64>,
  #[serde(default)]
  pub holding_period: Option<f64>,
  pub first_added: FmpDate,
  #[serde(default)]
  pub performance: Option<f64>,
  #[serde(default)]
  pub performance_percentage: Option<f64>,
  #[serde(default)]
  pub last_performance: Option<f64>,
  #[serde(default)]
  pub change_in_performance: Option<f64>,
  pub is_counted_for_performance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderPerformanceSummary {
  pub date: FmpDate,
  pub cik: String,
  pub investor_name: String,
  #[serde(default)]
  pub portfolio_size: Option<f64>,
  #[serde(default)]
  pub securities_added: Option<f64>,
  #[serde(default)]
  pub securities_removed: Option<f64>,
  #[serde(default)]
  pub market_value: Option<f64>,
  #[serde(default)]
  pub previous_market_value: Option<f64>,
  #[serde(default)]
  pub change_in_market_value: Option<f64>,
  #[serde(default)]
  pub change_in_market_value_percentage: Option<f64>,
  #[serde(default)]
  pub average_holding_period: Option<f64>,
  #[serde(default)]
  pub average_holding_period_top10: Option<f64>,
  #[serde(default)]
  pub average_holding_period_top20: Option<f64>,
  #[serde(default)]
  pub turnover: Option<f64>,
  #[serde(default)]
  pub turnover_alternate_sell: Option<f64>,
  #[serde(default)]
  pub turnover_alternate_buy: Option<f64>,
  #[serde(default)]
  pub performance: Option<f64>,
  #[serde(default)]
  pub performance_percentage: Option<f64>,
  #[serde(default)]
  pub last_performance: Option<f64>,
  #[serde(default)]
  pub change_in_performance: Option<f64>,
  #[serde(default)]
  pub performance1year: Option<f64>,
  #[serde(default)]
  pub performance_percentage1year: Option<f64>,
  #[serde(default)]
  pub performance3year: Option<f64>,
  #[serde(default)]
  pub performance_percentage3year: Option<f64>,
  #[serde(default)]
  pub performance5year: Option<f64>,
  #[serde(default)]
  pub performance_percentage5year: Option<f64>,
  #[serde(default)]
  pub performance_since_inception: Option<f64>,
  #[serde(default)]
  pub performance_since_inception_percentage: Option<f64>,
  #[serde(default)]
  pub performance_relative_to_sp500_percentage: Option<f64>,
  #[serde(default)]
  pub performance1year_relative_to_sp500_percentage: Option<f64>,
  #[serde(default)]
  pub performance3year_relative_to_sp500_percentage: Option<f64>,
  #[serde(default)]
  pub performance5year_relative_to_sp500_percentage: Option<f64>,
  #[serde(default)]
  pub performance_since_inception_relative_to_sp500_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolderIndustryBreakdown {
  pub date: FmpDate,
  pub cik: String,
  pub investor_name: String,
  pub industry_title: String,
  pub weight: f64,
  pub last_weight: f64,
  pub change_in_weight: f64,
  pub change_in_weight_percentage: f64,
  pub performance: f64,
  pub performance_percentage: f64,
  pub last_performance: f64,
  pub change_in_performance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsSummary {
  pub symbol: String,
  pub cik: String,
  pub date: FmpDate,
  pub investors_holding: f64,
  pub last_investors_holding: f64,
  pub investors_holding_change: f64,
  pub number_of13_fshares: f64,
  pub last_number_of13_fshares: f64,
  pub number_of13_fshares_change: f64,
  pub total_invested: f64,
  pub last_total_invested: f64,
  pub total_invested_change: f64,
  pub ownership_percent: f64,
  pub last_ownership_percent: f64,
  pub ownership_percent_change: f64,
  pub new_positions: f64,
  pub last_new_positions: f64,
  pub new_positions_change: f64,
  pub increased_positions: f64,
  pub last_increased_positions: f64,
  pub increased_positions_change: f64,
  pub closed_positions: f64,
  pub last_closed_positions: f64,
  pub closed_positions_change: f64,
  pub reduced_positions: f64,
  pub last_reduced_positions: f64,
  pub reduced_positions_change: f64,
  pub total_calls: f64,
  pub last_total_calls: f64,
  pub total_calls_change: f64,
  pub total_puts: f64,
  pub last_total_puts: f64,
  pub total_puts_change: f64,
  pub put_call_ratio: f64,
  pub last_put_call_ratio: f64,
  pub put_call_ratio_change: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryPerformanceSummary {
  pub industry_title: String,
  pub industry_value: f64,
  pub date: FmpDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FilingExtractParams {
  pub cik: String,
  pub year: i32,
  pub quarter: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FilingDatesParams {
  pub cik: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FilingExtractAnalyticsByHolderParams {
  pub symbol: String,
  pub year: i32,
  pub quarter: i32,
  #[builder(default, setter(strip_option))]
  pub page: Option<u32>,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct HolderPerformanceParams {
  pub cik: String,
  #[builder(default, setter(strip_option))]
  pub page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct HolderIndustryBreakdownParams {
  pub cik: String,
  pub year: i32,
  pub quarter: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct PositionsSummaryParams {
  pub symbol: String,
  pub year: i32,
  pub quarter: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct IndustryPerformanceParams {
  pub year: i32,
  pub quarter: i32,
}
