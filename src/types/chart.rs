use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

/// Full OHLCV bar with split- and dividend-adjusted prices.
///
/// Returned by the `historical-price-eod/full` endpoint. All price fields
/// are adjusted for both stock splits and dividend distributions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
  /// Ticker symbol.
  pub symbol: String,
  /// Date of the bar (calendar date for EOD data).
  pub date: FmpDate,
  /// Adjusted opening price.
  pub open: f64,
  /// Adjusted intraday high price.
  pub high: f64,
  /// Adjusted intraday low price.
  pub low: f64,
  /// Adjusted closing price.
  pub close: f64,
  /// Trading volume for the period; absent for some non-equity instruments.
  #[serde(default)]
  pub volume: Option<f64>,
  /// Absolute change in closing price from the previous bar's close.
  #[serde(default)]
  pub change: Option<f64>,
  /// Percentage change in closing price from the previous bar's close.
  #[serde(default)]
  pub change_percent: Option<f64>,
  /// Volume-weighted average price for the bar (close × volume / total volume).
  #[serde(default)]
  pub vwap: Option<f64>,
}

/// Lightweight EOD bar containing only closing price and volume.
///
/// Returned by the `historical-price-eod/light` endpoint. Prices are
/// adjusted for stock splits and dividends. Use this when you only need
/// close prices and volume to minimise response payload size.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightChartData {
  /// Ticker symbol.
  pub symbol: String,
  /// Date of the bar (calendar date for EOD data).
  pub date: FmpDate,
  /// Closing price (field is named `price` in the API response).
  pub price: f64,
  /// Trading volume; absent for some non-equity instruments.
  #[serde(default)]
  pub volume: Option<f64>,
}

/// EOD OHLCV bar with prices **not** adjusted for stock splits.
///
/// Returned by the `historical-price-eod/non-split-adjusted` endpoint.
/// Prices reflect the raw traded price without any split factor applied,
/// which preserves the true historical transaction prices.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnadjustedChartData {
  /// Ticker symbol.
  pub symbol: String,
  /// Date of the bar (calendar date for EOD data).
  pub date: FmpDate,
  /// Unadjusted opening price (no split factor applied).
  pub adj_open: f64,
  /// Unadjusted intraday high price.
  pub adj_high: f64,
  /// Unadjusted intraday low price.
  pub adj_low: f64,
  /// Unadjusted closing price.
  pub adj_close: f64,
  /// Trading volume; absent for some non-equity instruments.
  #[serde(default)]
  pub volume: Option<f64>,
}

/// Intraday OHLCV bar — same shape as [`UnadjustedChartData`] but without a symbol field.
///
/// FMP intraday endpoints omit the `symbol` field from each bar to reduce payload size.
pub type IntradayChartData = OmitSymbolUnadjusted;

/// Intraday price bar returned by the `/historical-chart/{interval}` endpoint.
///
/// The `symbol` field is intentionally absent (it is passed as a query parameter).
/// Field names are prefixed with `adj_` to match the API JSON keys, even though
/// intraday prices are not dividend-adjusted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OmitSymbolUnadjusted {
  /// Timestamp of the bar's open (space-separated "YYYY-MM-DD HH:MM:SS" format).
  pub date: FmpDateTime,
  /// Opening price for the interval.
  pub open: f64,
  /// Highest price reached during the interval.
  pub high: f64,
  /// Lowest price reached during the interval.
  pub low: f64,
  /// Closing price at the end of the interval.
  pub close: f64,
  /// Volume traded during the interval; absent for some instruments.
  #[serde(default)]
  pub volume: Option<f64>,
}

/// Time resolution for intraday chart data.
///
/// Used as a path parameter in the `/historical-chart/{interval}` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Interval {
  /// 1-minute bars — finest granularity; suitable for scalping and high-frequency analysis.
  #[serde(rename = "1min")]
  I1Min,
  /// 5-minute bars — commonly used for short-term intraday trading strategies.
  #[serde(rename = "5min")]
  I5Min,
  /// 15-minute bars — useful for swing entries within a single session.
  #[serde(rename = "15min")]
  I15Min,
  /// 30-minute bars — good balance between detail and noise reduction.
  #[serde(rename = "30min")]
  I30Min,
  /// 1-hour bars — popular for daily and multi-day swing analysis.
  #[serde(rename = "1hour")]
  I1Hour,
  /// 4-hour bars — bridges the gap between intraday and daily timeframes.
  #[serde(rename = "4hour")]
  I4Hour,
}

/// Parameters for EOD historical price endpoints with optional date range.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ChartHistoryParams {
  /// Ticker symbol (e.g., "AAPL").
  pub symbol: String,
  /// Earliest date to return (inclusive). Defaults to 5 years ago when omitted.
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  /// Latest date to return (inclusive). Defaults to today when omitted.
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}

/// Parameters for the intraday chart endpoint with optional date range.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ChartIntradayParams {
  /// Ticker symbol (e.g., "AAPL").
  pub symbol: String,
  /// Earliest date to return (inclusive).
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  /// Latest date to return (inclusive).
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}

#[cfg(test)]
mod tests {
  use super::{ChartData, IntradayChartData, LightChartData, UnadjustedChartData};

  #[test]
  fn chart_light_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/chart_light.json").unwrap();
    let _: Vec<LightChartData> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn chart_full_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/chart_full.json").unwrap();
    let _: Vec<ChartData> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn chart_unadjusted_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/chart_unadjusted.json").unwrap();
    let _: Vec<UnadjustedChartData> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn chart_intraday_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/chart_intraday.json").unwrap();
    let _: Vec<IntradayChartData> = serde_json::from_slice(&bytes).unwrap();
  }
}
