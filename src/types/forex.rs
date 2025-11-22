use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexPair {
  pub symbol: String,
  pub from_currency: String,
  pub to_currency: String,
  pub from_name: String,
  pub to_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexQuote {
  pub symbol: String,
  pub name: String,
  pub price: f64,
  pub change_percentage: f64,
  pub change: f64,
  pub volume: f64,
  pub day_low: f64,
  pub day_high: f64,
  pub year_high: f64,
  pub year_low: f64,
  #[serde(default, alias = "marketCap")]
  pub market_cap: Option<f64>,
  #[serde(alias = "priceAvg50")]
  pub price_avg50: f64,
  #[serde(alias = "priceAvg200")]
  pub price_avg200: f64,
  pub exchange: String,
  pub open: f64,
  pub previous_close: f64,
  pub timestamp: FmpDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexShortQuote {
  pub symbol: String,
  pub price: f64,
  pub change: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexLightChart {
  pub symbol: String,
  pub date: FmpDateTime,
  pub price: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexHistoricalChart {
  pub symbol: String,
  pub date: FmpDateTime,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub close: f64,
  pub volume: f64,
  pub change: f64,
  pub change_percent: f64,
  pub vwap: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexIntradayChart {
  pub date: FmpDateTime,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub close: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ForexSymbolParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct ForexBatchParams {
  pub short: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ForexHistoryParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}
