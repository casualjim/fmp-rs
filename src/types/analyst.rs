use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalystEstimate {
  pub symbol: String,
  pub date: FmpDate,
  #[serde(default)]
  pub revenue_low: Option<f64>,
  #[serde(default)]
  pub revenue_high: Option<f64>,
  #[serde(default)]
  pub revenue_avg: Option<f64>,
  #[serde(default)]
  pub ebitda_low: Option<f64>,
  #[serde(default)]
  pub ebitda_high: Option<f64>,
  #[serde(default)]
  pub ebitda_avg: Option<f64>,
  #[serde(default)]
  pub ebit_low: Option<f64>,
  #[serde(default)]
  pub ebit_high: Option<f64>,
  #[serde(default)]
  pub ebit_avg: Option<f64>,
  #[serde(default)]
  pub net_income_low: Option<f64>,
  #[serde(default)]
  pub net_income_high: Option<f64>,
  #[serde(default)]
  pub net_income_avg: Option<f64>,
  #[serde(default)]
  pub sga_expense_low: Option<f64>,
  #[serde(default)]
  pub sga_expense_high: Option<f64>,
  #[serde(default)]
  pub sga_expense_avg: Option<f64>,
  #[serde(default)]
  pub eps_avg: Option<f64>,
  #[serde(default)]
  pub eps_high: Option<f64>,
  #[serde(default)]
  pub eps_low: Option<f64>,
  #[serde(default)]
  pub num_analysts_revenue: Option<f64>,
  #[serde(default)]
  pub num_analysts_eps: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingsSnapshot {
  pub symbol: String,
  pub rating: String,
  pub overall_score: f64,
  pub discounted_cash_flow_score: f64,
  pub return_on_equity_score: f64,
  pub return_on_assets_score: f64,
  pub debt_to_equity_score: f64,
  pub price_to_earnings_score: f64,
  pub price_to_book_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalRating {
  #[serde(flatten)]
  pub snapshot: RatingsSnapshot,
  pub date: FmpDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTargetSummary {
  pub symbol: String,
  #[serde(default)]
  pub last_month_count: Option<f64>,
  #[serde(default)]
  pub last_month_avg_price_target: Option<f64>,
  #[serde(default)]
  pub last_quarter_count: Option<f64>,
  #[serde(default)]
  pub last_quarter_avg_price_target: Option<f64>,
  #[serde(default)]
  pub last_year_count: Option<f64>,
  #[serde(default)]
  pub last_year_avg_price_target: Option<f64>,
  #[serde(default)]
  pub all_time_count: Option<f64>,
  #[serde(default)]
  pub all_time_avg_price_target: Option<f64>,
  pub publishers: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTargetConsensus {
  pub symbol: String,
  #[serde(default)]
  pub target_high: Option<f64>,
  #[serde(default)]
  pub target_low: Option<f64>,
  #[serde(default)]
  pub target_consensus: Option<f64>,
  #[serde(default)]
  pub target_median: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTargetNews {
  pub symbol: String,
  pub published_date: FmpDate,
  #[serde(rename = "newsURL")]
  pub news_url: String,
  pub news_title: String,
  pub analyst_name: String,
  #[serde(default)]
  pub price_target: Option<f64>,
  #[serde(default)]
  pub adj_price_target: Option<f64>,
  #[serde(default)]
  pub price_when_posted: Option<f64>,
  pub news_publisher: String,
  #[serde(rename = "newsBaseURL")]
  pub news_base_url: String,
  pub analyst_company: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockGrade {
  pub symbol: String,
  pub date: FmpDate,
  pub grading_company: String,
  pub previous_grade: String,
  pub new_grade: String,
  pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalStockGrade {
  pub symbol: String,
  pub date: FmpDate,
  pub analyst_ratings_buy: f64,
  pub analyst_ratings_hold: f64,
  pub analyst_ratings_sell: f64,
  pub analyst_ratings_strong_sell: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockGradeSummary {
  pub symbol: String,
  #[serde(default)]
  pub strong_buy: Option<f64>,
  #[serde(default)]
  pub buy: Option<f64>,
  #[serde(default)]
  pub hold: Option<f64>,
  #[serde(default)]
  pub sell: Option<f64>,
  #[serde(default)]
  pub strong_sell: Option<f64>,
  pub consensus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockGradeNews {
  pub symbol: String,
  pub published_date: FmpDate,
  #[serde(rename = "newsURL")]
  pub news_url: String,
  pub news_title: String,
  #[serde(rename = "newsBaseURL")]
  pub news_base_url: String,
  pub news_publisher: String,
  pub new_grade: String,
  pub previous_grade: String,
  pub grading_company: String,
  pub action: String,
  pub price_when_posted: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct AnalystEstimatesParams {
  pub symbol: String,
  pub period: String,
  #[builder(default, setter(strip_option))]
  pub page: Option<u32>,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolLimitParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolPageParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub page: Option<u32>,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct PageParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[cfg(test)]
mod tests {
  use super::{
    AnalystEstimate, HistoricalRating, HistoricalStockGrade, PriceTargetConsensus, PriceTargetNews, PriceTargetSummary,
    RatingsSnapshot, StockGrade, StockGradeNews, StockGradeSummary,
  };

  #[test]
  fn analyst_estimate_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/analyst_estimate.json").unwrap();
    let _: Vec<AnalystEstimate> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn ratings_snapshot_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/ratings_snapshot.json").unwrap();
    let _: Vec<RatingsSnapshot> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn ratings_historical_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/ratings_historical.json").unwrap();
    let _: Vec<HistoricalRating> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn price_target_summary_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/price_target_summary.json").unwrap();
    let _: Vec<PriceTargetSummary> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn price_target_consensus_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/price_target_consensus.json").unwrap();
    let _: Vec<PriceTargetConsensus> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn price_target_news_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/price_target_news.json").unwrap();
    let _: Vec<PriceTargetNews> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stock_grade_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/stock_grade.json").unwrap();
    let _: Vec<StockGrade> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stock_grade_historical_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/stock_grade_historical.json").unwrap();
    let _: Vec<HistoricalStockGrade> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stock_grade_summary_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/stock_grade_summary.json").unwrap();
    let _: Vec<StockGradeSummary> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stock_grade_news_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/stock_grade_news.json").unwrap();
    let _: Vec<StockGradeNews> = serde_json::from_slice(&bytes).unwrap();
  }
}
