use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalIndicatorBase {
  pub date: FmpDateTime,
  pub open: f64,
  pub high: f64,
  pub low: f64,
  pub close: f64,
  #[serde(default)]
  pub volume: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub sma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub ema: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WmaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub wma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub dema: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemaIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub tema: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RsiIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub rsi: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardDeviationIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub standard_deviation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WilliamsIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub williams: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdxIndicator {
  #[serde(flatten)]
  pub base: TechnicalIndicatorBase,
  pub adx: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct TechnicalIndicatorParams {
  pub symbol: String,
  pub period_length: u32,
  pub timeframe: String,
  #[builder(setter(strip_option))]
  pub from: Option<FmpDateTime>,
  #[builder(setter(strip_option))]
  pub to: Option<FmpDateTime>,
}

#[cfg(test)]
mod tests {
  use super::{
    AdxIndicator, DemaIndicator, EmaIndicator, RsiIndicator, SmaIndicator, StandardDeviationIndicator, TemaIndicator,
    WilliamsIndicator, WmaIndicator,
  };

  #[test]
  fn sma_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/sma_indicator.json").unwrap();
    let _: Vec<SmaIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn ema_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/ema_indicator.json").unwrap();
    let _: Vec<EmaIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn wma_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/wma_indicator.json").unwrap();
    let _: Vec<WmaIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn dema_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/dema_indicator.json").unwrap();
    let _: Vec<DemaIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn tema_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/tema_indicator.json").unwrap();
    let _: Vec<TemaIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn rsi_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/rsi_indicator.json").unwrap();
    let _: Vec<RsiIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stddev_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/stddev_indicator.json").unwrap();
    let _: Vec<StandardDeviationIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn williams_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/williams_indicator.json").unwrap();
    let _: Vec<WilliamsIndicator> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn adx_indicator_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/adx_indicator.json").unwrap();
    let _: Vec<AdxIndicator> = serde_json::from_slice(&bytes).unwrap();
  }
}
