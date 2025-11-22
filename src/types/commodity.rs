use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commodity {
  pub symbol: String,
  pub name: String,
  pub exchange: String,
  #[serde(default)]
  pub trade_month: Option<String>,
  pub currency: String,
}
