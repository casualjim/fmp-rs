use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dividend {
  pub symbol: String,
  pub date: FmpDate,
  pub record_date: FmpDate,
  pub payment_date: FmpDate,
  pub declaration_date: FmpDate,
  pub adj_dividend: f64,
  pub dividend: f64,
  #[serde(rename = "yield")]
  pub dividend_yield: f64,
  pub frequency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsReport {
  pub symbol: String,
  pub date: FmpDate,
  #[serde(default)]
  pub eps_actual: Option<f64>,
  #[serde(default)]
  pub eps_estimated: Option<f64>,
  #[serde(default)]
  pub revenue_actual: Option<f64>,
  #[serde(default)]
  pub revenue_estimated: Option<f64>,
  pub last_updated: FmpDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ipo {
  pub symbol: String,
  pub date: FmpDate,
  pub daa: String,
  pub company: String,
  pub exchange: String,
  pub actions: String,
  #[serde(default)]
  pub shares: Option<f64>,
  #[serde(default)]
  pub price_range: Option<String>,
  #[serde(default)]
  pub market_cap: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpoDisclosure {
  pub symbol: String,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub effectiveness_date: FmpDate,
  pub cik: String,
  pub form: String,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpoProspectus {
  pub symbol: String,
  pub accepted_date: FmpDateTime,
  pub filing_date: FmpDate,
  pub ipo_date: FmpDate,
  pub cik: String,
  pub price_public_per_share: f64,
  pub price_public_total: f64,
  pub discounts_and_commissions_per_share: f64,
  pub discounts_and_commissions_total: f64,
  pub proceeds_before_expenses_per_share: f64,
  pub proceeds_before_expenses_total: f64,
  pub form: String,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockSplit {
  pub symbol: String,
  pub date: FmpDate,
  pub numerator: i32,
  pub denominator: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolLimitParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct CalendarRangeParams {
  pub from: Option<FmpDate>,
  pub to: Option<FmpDate>,
}

#[cfg(test)]
mod tests {
  use super::{Dividend, EarningsReport, Ipo, IpoDisclosure, IpoProspectus, StockSplit};

  #[test]
  fn dividends_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/dividends.json").unwrap();
    let _: Vec<Dividend> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn earnings_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/earnings.json").unwrap();
    let _: Vec<EarningsReport> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn ipo_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/ipo.json").unwrap();
    let _: Vec<Ipo> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn ipo_disclosure_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/ipo_disclosure.json").unwrap();
    let _: Vec<IpoDisclosure> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn ipo_prospectus_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/ipo_prospectus.json").unwrap();
    let _: Vec<IpoProspectus> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stock_split_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/stock_split.json").unwrap();
    let _: Vec<StockSplit> = serde_json::from_slice(&bytes).unwrap();
  }
}
