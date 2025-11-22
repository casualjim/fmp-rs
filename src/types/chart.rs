use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
  pub symbol: String,
  pub date: FmpDateTime,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub close: f64,
  #[serde(default)]
  pub volume: Option<f64>,
  #[serde(default)]
  pub change: Option<f64>,
  #[serde(default)]
  pub change_percent: Option<f64>,
  #[serde(default)]
  pub vwap: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightChartData {
  pub symbol: String,
  pub date: FmpDateTime,
  pub close: f64,
  #[serde(default)]
  pub volume: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnadjustedChartData {
  pub symbol: String,
  pub date: FmpDateTime,
  pub adj_open: f64,
  pub adj_high: f64,
  pub adj_low: f64,
  pub adj_close: f64,
  #[serde(default)]
  pub volume: Option<f64>,
}

pub type IntradayChartData = OmitSymbolUnadjusted;

/// Helper type to model intraday data that omits the symbol field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OmitSymbolUnadjusted {
  pub date: FmpDateTime,
  pub adj_open: f64,
  pub adj_high: f64,
  pub adj_low: f64,
  pub adj_close: f64,
  #[serde(default)]
  pub volume: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Interval {
  #[serde(rename = "1min")]
  I1Min,
  #[serde(rename = "5min")]
  I5Min,
  #[serde(rename = "15min")]
  I15Min,
  #[serde(rename = "30min")]
  I30Min,
  #[serde(rename = "1hour")]
  I1Hour,
  #[serde(rename = "4hour")]
  I4Hour,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ChartHistoryParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ChartIntradayParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}
