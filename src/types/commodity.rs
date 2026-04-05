use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commodity {
  pub symbol: String,
  pub name: String,
  #[serde(default)]
  pub exchange: Option<String>,
  #[serde(default)]
  pub trade_month: Option<String>,
  pub currency: String,
}

#[cfg(test)]
mod tests {
  use super::Commodity;

  #[test]
  fn commodity_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/commodity.json").unwrap();
    let _: Vec<Commodity> = serde_json::from_slice(&bytes).unwrap();
  }
}
