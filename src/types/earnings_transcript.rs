use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestEarningTranscript {
  pub symbol: String,
  pub period: String,
  pub fiscal_year: i32,
  pub date: FmpDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningTranscript {
  pub symbol: String,
  pub period: String,
  pub year: i32,
  pub date: FmpDate,
  pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptDate {
  pub quarter: i32,
  pub fiscal_year: i32,
  pub date: FmpDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableTranscriptSymbol {
  pub symbol: String,
  pub company_name: String,
  pub no_of_transcripts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct LatestTranscriptsParams {
  pub limit: Option<u32>,
  pub page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct TranscriptParams {
  pub symbol: String,
  pub year: String,
  pub quarter: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct TranscriptDatesParams {
  pub symbol: String,
}

#[cfg(test)]
mod tests {
  use super::{EarningTranscript, LatestEarningTranscript, TranscriptDate};

  #[test]
  fn transcript_latest_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/transcript_latest.json").unwrap();
    let _: Vec<LatestEarningTranscript> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn transcript_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/transcript.json").unwrap();
    let _: Vec<EarningTranscript> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn transcript_dates_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/transcript_dates.json").unwrap();
    let _: Vec<TranscriptDate> = serde_json::from_slice(&bytes).unwrap();
  }
}
