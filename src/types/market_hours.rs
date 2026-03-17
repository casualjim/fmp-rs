use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeMarketHours {
  pub exchange: String,
  pub name: String,
  pub opening_hour: String,
  pub closing_hour: String,
  pub timezone: String,
  pub is_market_open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolidayByExchange {
  pub exchange: String,
  pub date: FmpDate,
  pub name: String,
  #[serde(default)]
  pub is_closed: Option<bool>,
  #[serde(default, alias = "adjOpenTime")]
  pub adj_open_time: Option<String>,
  #[serde(default, alias = "adjCloseTime")]
  pub adj_close_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ExchangeParams {
  pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct HolidaysByExchangeParams {
  pub exchange: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}

#[cfg(test)]
mod tests {
  use super::{ExchangeMarketHours, HolidayByExchange};

  #[test]
  fn exchange_market_hours_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/exchange_market_hours.json").unwrap();
    let _: Vec<ExchangeMarketHours> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn holidays_by_exchange_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/holidays_by_exchange.json").unwrap();
    let _: Vec<HolidayByExchange> = serde_json::from_slice(&bytes).unwrap();
  }
}
