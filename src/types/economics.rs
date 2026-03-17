use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasuryRate {
  pub date: FmpDate,
  pub month1: f64,
  pub month2: f64,
  pub month3: f64,
  pub month6: f64,
  pub year1: f64,
  pub year2: f64,
  pub year3: f64,
  pub year5: f64,
  pub year7: f64,
  pub year10: f64,
  pub year20: f64,
  pub year30: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconomicIndicator {
  pub date: FmpDate,
  pub name: String,
  pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconomicCalendar {
  pub date: FmpDate,
  pub country: String,
  pub event: String,
  pub currency: String,
  #[serde(default)]
  pub previous: Option<f64>,
  #[serde(default)]
  pub estimate: Option<f64>,
  #[serde(default)]
  pub actual: Option<f64>,
  #[serde(default)]
  pub change: Option<f64>,
  pub impact: String,
  #[serde(default)]
  pub change_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketRiskPremium {
  pub country: String,
  pub continent: String,
  pub country_risk_premium: f64,
  pub total_equity_risk_premium: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct DateRangeParams {
  pub from: Option<FmpDate>,
  pub to: Option<FmpDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct EconomicIndicatorParams {
  pub name: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct EmptyParams {}

#[cfg(test)]
mod tests {
  use super::{EconomicCalendar, EconomicIndicator, MarketRiskPremium, TreasuryRate};

  #[test]
  fn treasury_rate_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/treasury_rate.json").unwrap();
    let _: Vec<TreasuryRate> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn economic_indicator_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/economic_indicator.json").unwrap();
    let _: Vec<EconomicIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn economic_calendar_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/economic_calendar.json").unwrap();
    let _: Vec<EconomicCalendar> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn market_risk_premium_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/market_risk_premium.json").unwrap();
    let _: Vec<MarketRiskPremium> = serde_json::from_slice(&bytes).unwrap();
  }
}
