use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialDisclosure {
  pub symbol: String,
  pub disclosure_date: FmpDate,
  pub transaction_date: FmpDate,
  pub first_name: String,
  pub last_name: String,
  pub office: String,
  pub district: String,
  pub owner: String,
  pub asset_description: String,
  pub asset_type: String,
  #[serde(rename = "type")]
  pub trade_type: String,
  pub amount: String,
  #[serde(default)]
  pub capital_gains_over200_usd: Option<String>,
  #[serde(default)]
  pub comment: Option<String>,
  pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct NameParams {
  pub name: String,
}

#[cfg(test)]
mod tests {
  use super::FinancialDisclosure;

  #[test]
  fn senate_trade_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/senate_trade.json").unwrap();
    let _: Vec<FinancialDisclosure> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn house_trade_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/house_trade.json").unwrap();
    let _: Vec<FinancialDisclosure> = serde_json::from_slice(&bytes).unwrap();
  }
}
