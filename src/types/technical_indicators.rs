use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalIndicatorBase {
  pub date: FmpDateTime,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub close: f64,
  #[serde(default)]
  pub volume: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub sma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub ema: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WmaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub wma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub dema: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub tema: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RsiIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub rsi: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardDeviationIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub standard_deviation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WilliamsIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub williams: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdxIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub adx: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct TechnicalIndicatorParams {
  pub symbol: String,
  pub period_length: u32,
  pub timeframe: String,
  #[builder(setter(strip_option))]
  pub from: Option<FmpDateTime>,
  #[builder(setter(strip_option))]
  pub to: Option<FmpDateTime>,
}
