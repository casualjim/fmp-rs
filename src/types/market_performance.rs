use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorPerformance {
  pub date: FmpDate,
  pub sector: String,
  pub exchange: String,
  pub average_change: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryPerformance {
  pub date: FmpDate,
  pub industry: String,
  pub exchange: String,
  pub average_change: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorPe {
  pub date: FmpDate,
  pub sector: String,
  pub exchange: String,
  pub pe: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryPe {
  pub date: FmpDate,
  pub industry: String,
  pub exchange: String,
  pub pe: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockMovement {
  pub symbol: String,
  pub price: f64,
  pub name: String,
  pub change: f64,
  pub changes_percentage: f64,
  pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SectorSnapshotParams {
  pub date: FmpDate,
  #[builder(default, setter(strip_option))]
  pub exchange: Option<String>,
  #[builder(default, setter(strip_option))]
  pub sector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct IndustrySnapshotParams {
  pub date: FmpDate,
  #[builder(default, setter(strip_option))]
  pub exchange: Option<String>,
  #[builder(default, setter(strip_option))]
  pub industry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SectorHistoryParams {
  pub sector: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub exchange: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct IndustryHistoryParams {
  pub industry: String,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub exchange: Option<String>,
}

#[cfg(test)]
mod tests {
  use super::{IndustryPe, IndustryPerformance, SectorPe, SectorPerformance, StockMovement};

  #[test]
  fn biggest_gainers_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/biggest_gainers.json").unwrap();
    let _: Vec<StockMovement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn biggest_losers_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/biggest_losers.json").unwrap();
    let _: Vec<StockMovement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn most_actives_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/most_actives.json").unwrap();
    let _: Vec<StockMovement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn sector_performance_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/sector_performance.json").unwrap();
    let _: Vec<SectorPerformance> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn industry_performance_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/industry_performance.json").unwrap();
    let _: Vec<IndustryPerformance> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn sector_pe_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/sector_pe.json").unwrap();
    let _: Vec<SectorPe> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn industry_pe_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/industry_pe.json").unwrap();
    let _: Vec<IndustryPe> = serde_json::from_slice(&bytes).unwrap();
  }
}
