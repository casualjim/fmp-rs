use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexItem {
  pub symbol: String,
  pub name: String,
  pub exchange: String,
  pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexQuote {
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
pub struct IndexShortQuote {
  pub symbol: String,
  pub price: f64,
  pub change: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexLightChart {
  pub symbol: String,
  pub date: FmpDateTime,
  pub price: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexFullChart {
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
pub struct IndexIntradayData {
  pub date: FmpDateTime,
  pub open: f64,
  pub low: f64,
  pub high: f64,
  pub close: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexConstituent {
  pub symbol: String,
  pub name: String,
  pub sector: String,
  pub sub_sector: String,
  pub head_quarter: String,
  pub date_first_added: Option<FmpDate>,
  pub cik: String,
  pub founded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalIndexChange {
  pub date_added: FmpDate,
  pub added_security: String,
  pub removed_ticker: String,
  pub removed_security: String,
  pub date: FmpDate,
  pub symbol: String,
  pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct IndexSymbolParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct IndexBatchParams {
  pub short: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct IndexHistoryParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}

#[cfg(test)]
mod tests {
  use super::{IndexFullChart, IndexIntradayData, IndexItem, IndexLightChart, IndexQuote, IndexShortQuote};

  #[test]
  fn index_item_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/index_item.json").unwrap();
    let _: Vec<IndexItem> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn index_quote_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/index_quote.json").unwrap();
    let _: Vec<IndexQuote> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn index_quote_short_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/index_quote_short.json").unwrap();
    let _: Vec<IndexShortQuote> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn index_chart_light_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/index_chart_light.json").unwrap();
    let _: Vec<IndexLightChart> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn index_chart_full_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/index_chart_full.json").unwrap();
    let _: Vec<IndexFullChart> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn index_chart_intraday_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/index_chart_intraday.json").unwrap();
    let _: Vec<IndexIntradayData> = serde_json::from_slice(&bytes).unwrap();
  }
}
