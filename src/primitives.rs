use serde::{Deserialize, Serialize};

pub type FmpDate = jiff::civil::Date;
pub type FmpDateTime = jiff::Timestamp;

/// Server-provided error envelope used by FMP.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FmpErrorPayload {
  #[serde(rename = "Error Message")]
  pub message: String,
  #[serde(default)]
  pub code: Option<String>,
  #[serde(default)]
  pub status: Option<u16>,
}

/// Generic response wrapper; many FMP endpoints return raw arrays instead, so this is
/// opt-in for endpoints that expose the metadata structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FmpResponse<T> {
  pub data: T,
  #[serde(default)]
  pub error: Option<FmpErrorPayload>,
  pub timestamp: i64,
}
