use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cryptocurrency {
  pub symbol: String,
  pub name: String,
  pub exchange: String,
  pub ico_date: FmpDateTime,
  pub circulating_supply: f64,
  pub total_supply: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptocurrencyQuote {
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
  #[serde(alias = "marketCap")]
  pub market_cap: f64,
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
pub struct CryptocurrencyShortQuote {
  pub symbol: String,
  pub price: f64,
  pub change: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptocurrencyLightChart {
  pub symbol: String,
  pub date: FmpDateTime,
  pub price: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptocurrencyHistoricalChart {
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
pub struct CryptocurrencyIntradayPrice {
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
pub struct CryptoSymbolParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct CryptoBatchParams {
  pub short: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CryptoHistoryParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}
