use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderTrading {
  pub symbol: String,
  pub filing_date: FmpDate,
  pub transaction_date: FmpDate,
  pub reporting_cik: String,
  pub company_cik: String,
  pub transaction_type: String,
  #[serde(default)]
  pub securities_owned: Option<f64>,
  pub reporting_name: String,
  pub type_of_owner: String,
  pub acquisition_or_disposition: String,
  pub direct_or_indirect: String,
  pub form_type: String,
  #[serde(default)]
  pub securities_transacted: Option<f64>,
  #[serde(default)]
  pub price: Option<f64>,
  pub security_name: String,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderReportingName {
  pub reporting_cik: String,
  pub reporting_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderTransactionType {
  pub transaction_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderTradeStatistics {
  pub symbol: String,
  pub cik: String,
  pub year: i32,
  pub quarter: i32,
  pub acquired_transactions: f64,
  pub disposed_transactions: f64,
  pub acquired_disposed_ratio: f64,
  pub total_acquired: f64,
  pub total_disposed: f64,
  pub average_acquired: f64,
  pub average_disposed: f64,
  pub total_purchases: f64,
  pub total_sales: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcquisitionOwnership {
  pub cik: String,
  pub symbol: String,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub cusip: String,
  pub name_of_reporting_person: String,
  pub citizenship_or_place_of_organization: String,
  pub sole_voting_power: String,
  pub shared_voting_power: String,
  pub sole_dispositive_power: String,
  pub shared_dispositive_power: String,
  pub amount_beneficially_owned: String,
  pub percent_of_class: String,
  pub type_of_reporting_person: String,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct InsiderLatestParams {
  pub date: Option<FmpDate>,
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct InsiderSearchParams {
  pub symbol: Option<String>,
  pub page: Option<u32>,
  pub limit: Option<u32>,
  pub reporting_cik: Option<String>,
  pub company_cik: Option<String>,
  pub transaction_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ReportingNameParams {
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct InsiderSymbolParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct AcquisitionOwnershipParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[cfg(test)]
mod tests {
  use super::{
    AcquisitionOwnership, InsiderReportingName, InsiderTradeStatistics, InsiderTrading, InsiderTransactionType,
  };

  #[test]
  fn insider_trading_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/insider_trading.json").unwrap();
    let _: Vec<InsiderTrading> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn insider_reporting_name_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/insider_reporting_name.json").unwrap();
    let _: Vec<InsiderReportingName> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn insider_transaction_type_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/insider_transaction_type.json").unwrap();
    let _: Vec<InsiderTransactionType> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn insider_statistics_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/insider_statistics.json").unwrap();
    let _: Vec<InsiderTradeStatistics> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn acquisition_ownership_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/acquisition_ownership.json").unwrap();
    let _: Vec<AcquisitionOwnership> = serde_json::from_slice(&bytes).unwrap();
  }
}
