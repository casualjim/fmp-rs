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
