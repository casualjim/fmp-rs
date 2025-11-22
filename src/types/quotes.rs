use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockQuote {
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
  #[serde(alias = "mktCap")]
  pub market_cap: f64,
  #[serde(alias = "priceAvg50")]
  pub price_avg50: f64,
  #[serde(alias = "priceAvg200")]
  pub price_avg200: f64,
  pub exchange: String,
  pub open: f64,
  pub previous_close: f64,
  pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockQuoteShort {
  pub symbol: String,
  pub price: f64,
  pub change: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AftermarketTrade {
  pub symbol: String,
  pub price: f64,
  pub trade_size: f64,
  pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AftermarketQuote {
  pub symbol: String,
  pub bid_size: f64,
  pub bid_price: f64,
  pub ask_size: f64,
  pub ask_price: f64,
  pub volume: f64,
  pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPriceChange {
  pub symbol: String,
  #[serde(rename = "1D")]
  pub one_day: f64,
  #[serde(rename = "5D")]
  pub five_day: f64,
  #[serde(rename = "1M")]
  pub one_month: f64,
  #[serde(rename = "3M")]
  pub three_month: f64,
  #[serde(rename = "6M")]
  pub six_month: f64,
  pub ytd: f64,
  #[serde(rename = "1Y")]
  pub one_year: f64,
  #[serde(rename = "3Y")]
  pub three_year: f64,
  #[serde(rename = "5Y")]
  pub five_year: f64,
  #[serde(rename = "10Y")]
  pub ten_year: f64,
  pub max: f64,
}

// Parameter bags for request builders
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct QuoteParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct BatchQuoteParams {
  pub symbols: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ExchangeQuoteParams {
  pub exchange: String,
  #[serde(default)]
  #[builder(default, setter(strip_option))]
  pub short: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct ShortParams {
  pub short: Option<bool>,
}
