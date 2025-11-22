use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolSearchResult {
  pub symbol: String,
  pub name: String,
  pub currency: String,
  pub exchange_full_name: String,
  pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NameSearchResult {
  pub symbol: String,
  pub name: String,
  pub currency: String,
  pub exchange_full_name: String,
  pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CikSearchResult {
  pub symbol: String,
  pub company_name: String,
  pub cik: String,
  pub exchange_full_name: String,
  pub exchange: String,
  pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CusipSearchResult {
  pub symbol: String,
  pub company_name: String,
  pub cusip: String,
  pub market_cap: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsinSearchResult {
  pub symbol: String,
  pub name: String,
  pub isin: String,
  pub market_cap: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockScreenerResult {
  pub symbol: String,
  pub company_name: String,
  pub market_cap: f64,
  pub sector: String,
  pub industry: String,
  pub beta: f64,
  pub price: f64,
  pub last_annual_dividend: f64,
  pub volume: f64,
  pub exchange: String,
  pub exchange_short_name: String,
  pub country: String,
  pub is_etf: bool,
  pub is_fund: bool,
  pub is_actively_trading: bool,
}

/// Many screener endpoints accept a large set of optional filters; this builder keeps
/// the Rust API ergonomic without inventing a custom fluent interface.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct StockScreenerParams {
  pub market_cap_more_than: Option<f64>,
  pub market_cap_less_than: Option<f64>,
  pub price_more_than: Option<f64>,
  pub price_less_than: Option<f64>,
  pub beta_more_than: Option<f64>,
  pub beta_less_than: Option<f64>,
  pub volume_more_than: Option<f64>,
  pub volume_less_than: Option<f64>,
  pub dividend: Option<f64>,
  pub sector: Option<String>,
  pub industry: Option<String>,
  pub exchange: Option<String>,
  pub country: Option<String>,
  pub is_etf: Option<bool>,
  pub is_fund: Option<bool>,
  pub is_actively_trading: Option<bool>,
  pub limit: Option<u32>,
  pub page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolSearchParams {
  pub query: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
  #[builder(default, setter(strip_option))]
  pub exchange: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct NameSearchParams {
  pub query: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
  #[builder(default, setter(strip_option))]
  pub exchange: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CikSearchParams {
  pub cik: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CusipSearchParams {
  pub cusip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct IsinSearchParams {
  pub isin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ExchangeVariantSearchParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeVariantResult {
  pub symbol: String,
  pub price: f64,
  pub beta: f64,
  pub vol_avg: f64,
  pub mkt_cap: f64,
  pub last_div: f64,
  pub range: String,
  pub changes: f64,
  pub company_name: String,
  pub currency: String,
  pub cik: String,
  pub isin: String,
  pub cusip: String,
  pub exchange: String,
  pub exchange_short_name: String,
  pub industry: String,
  pub website: String,
  pub description: String,
  pub ceo: String,
  pub sector: String,
  pub country: String,
  pub full_time_employees: String,
  pub phone: String,
  pub address: String,
  pub city: String,
  pub state: String,
  pub zip: String,
  pub dcf_diff: f64,
  pub dcf: f64,
  pub image: String,
  pub ipo_date: FmpDate,
  pub default_image: bool,
  pub is_etf: bool,
  pub is_actively_trading: bool,
  pub is_adr: bool,
  pub is_fund: bool,
}
