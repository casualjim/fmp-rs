use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[allow(clippy::too_many_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CotReport {
  pub symbol: String,
  pub date: FmpDate,
  pub name: String,
  pub sector: String,
  #[serde(default)]
  pub exchange: Option<String>,
  pub market_and_exchange_names: String,
  pub cftc_contract_market_code: String,
  pub cftc_market_code: String,
  pub cftc_region_code: String,
  pub cftc_commodity_code: String,
  pub open_interest_all: f64,
  pub noncomm_positions_long_all: f64,
  pub noncomm_positions_short_all: f64,
  pub noncomm_positions_spread_all: f64,
  pub comm_positions_long_all: f64,
  pub comm_positions_short_all: f64,
  pub tot_rept_positions_long_all: f64,
  pub tot_rept_positions_short_all: f64,
  pub nonrept_positions_long_all: f64,
  pub nonrept_positions_short_all: f64,
  pub open_interest_old: f64,
  pub noncomm_positions_long_old: f64,
  pub noncomm_positions_short_old: f64,
  pub noncomm_positions_spread_old: f64,
  pub comm_positions_long_old: f64,
  pub comm_positions_short_old: f64,
  pub tot_rept_positions_long_old: f64,
  pub tot_rept_positions_short_old: f64,
  pub nonrept_positions_long_old: f64,
  pub nonrept_positions_short_old: f64,
  pub open_interest_other: f64,
  pub noncomm_positions_long_other: f64,
  pub noncomm_positions_short_other: f64,
  pub noncomm_positions_spread_other: f64,
  pub comm_positions_long_other: f64,
  pub comm_positions_short_other: f64,
  pub tot_rept_positions_long_other: f64,
  pub tot_rept_positions_short_other: f64,
  pub nonrept_positions_long_other: f64,
  pub nonrept_positions_short_other: f64,
  pub change_in_open_interest_all: f64,
  pub change_in_noncomm_long_all: f64,
  pub change_in_noncomm_short_all: f64,
  pub change_in_noncomm_spead_all: f64,
  pub change_in_comm_long_all: f64,
  pub change_in_comm_short_all: f64,
  pub change_in_tot_rept_long_all: f64,
  pub change_in_tot_rept_short_all: f64,
  pub change_in_nonrept_long_all: f64,
  pub change_in_nonrept_short_all: f64,
  pub pct_of_open_interest_all: f64,
  pub pct_of_oi_noncomm_long_all: f64,
  pub pct_of_oi_noncomm_short_all: f64,
  pub pct_of_oi_noncomm_spread_all: f64,
  pub pct_of_oi_comm_long_all: f64,
  pub pct_of_oi_comm_short_all: f64,
  pub pct_of_oi_tot_rept_long_all: f64,
  pub pct_of_oi_tot_rept_short_all: f64,
  pub pct_of_oi_nonrept_long_all: f64,
  pub pct_of_oi_nonrept_short_all: f64,
  pub pct_of_open_interest_ol: f64,
  pub pct_of_oi_noncomm_long_ol: f64,
  pub pct_of_oi_noncomm_short_ol: f64,
  pub pct_of_oi_noncomm_spread_ol: f64,
  pub pct_of_oi_comm_long_ol: f64,
  pub pct_of_oi_comm_short_ol: f64,
  pub pct_of_oi_tot_rept_long_ol: f64,
  pub pct_of_oi_tot_rept_short_ol: f64,
  pub pct_of_oi_nonrept_long_ol: f64,
  pub pct_of_oi_nonrept_short_ol: f64,
  pub pct_of_open_interest_other: f64,
  pub pct_of_oi_noncomm_long_other: f64,
  pub pct_of_oi_noncomm_short_other: f64,
  pub pct_of_oi_noncomm_spread_other: f64,
  pub pct_of_oi_comm_long_other: f64,
  pub pct_of_oi_comm_short_other: f64,
  pub pct_of_oi_tot_rept_long_other: f64,
  pub pct_of_oi_tot_rept_short_other: f64,
  pub pct_of_oi_nonrept_long_other: f64,
  pub pct_of_oi_nonrept_short_other: f64,
  pub traders_tot_all: f64,
  pub traders_noncomm_long_all: f64,
  pub traders_noncomm_short_all: f64,
  pub traders_noncomm_spread_all: f64,
  pub traders_comm_long_all: f64,
  pub traders_comm_short_all: f64,
  pub traders_tot_rept_long_all: f64,
  pub traders_tot_rept_short_all: f64,
  pub traders_tot_ol: f64,
  pub traders_noncomm_long_ol: f64,
  pub traders_noncomm_short_ol: f64,
  pub traders_noncomm_spead_ol: f64,
  pub traders_comm_long_ol: f64,
  pub traders_comm_short_ol: f64,
  pub traders_tot_rept_long_ol: f64,
  pub traders_tot_rept_short_ol: f64,
  pub traders_tot_other: f64,
  pub traders_noncomm_long_other: f64,
  pub traders_noncomm_short_other: f64,
  pub traders_noncomm_spread_other: f64,
  pub traders_comm_long_other: f64,
  pub traders_comm_short_other: f64,
  pub traders_tot_rept_long_other: f64,
  pub traders_tot_rept_short_other: f64,
  pub conc_gross_le4_tdr_long_all: f64,
  pub conc_gross_le4_tdr_short_all: f64,
  pub conc_gross_le8_tdr_long_all: f64,
  pub conc_gross_le8_tdr_short_all: f64,
  pub conc_net_le4_tdr_long_all: f64,
  pub conc_net_le4_tdr_short_all: f64,
  pub conc_net_le8_tdr_long_all: f64,
  pub conc_net_le8_tdr_short_all: f64,
  pub conc_gross_le4_tdr_long_ol: f64,
  pub conc_gross_le4_tdr_short_ol: f64,
  pub conc_gross_le8_tdr_long_ol: f64,
  pub conc_gross_le8_tdr_short_ol: f64,
  pub conc_net_le4_tdr_long_ol: f64,
  pub conc_net_le4_tdr_short_ol: f64,
  pub conc_net_le8_tdr_long_ol: f64,
  pub conc_net_le8_tdr_short_ol: f64,
  pub conc_gross_le4_tdr_long_other: f64,
  pub conc_gross_le4_tdr_short_other: f64,
  pub conc_gross_le8_tdr_long_other: f64,
  pub conc_gross_le8_tdr_short_other: f64,
  pub conc_net_le4_tdr_long_other: f64,
  pub conc_net_le4_tdr_short_other: f64,
  pub conc_net_le8_tdr_long_other: f64,
  pub conc_net_le8_tdr_short_other: f64,
  pub contract_units: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CotAnalysis {
  pub symbol: String,
  pub date: FmpDate,
  pub name: String,
  pub sector: String,
  pub exchange: String,
  pub current_long_market_situation: f64,
  pub current_short_market_situation: f64,
  pub market_situation: String,
  pub previous_long_market_situation: f64,
  pub previous_short_market_situation: f64,
  pub previous_market_situation: String,
  pub net_postion: f64,
  pub previous_net_position: f64,
  pub change_in_net_position: f64,
  pub market_sentiment: String,
  pub reversal_trend: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CotList {
  pub symbol: String,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct CotRangeParams {
  pub symbol: Option<String>,
  pub from: Option<FmpDate>,
  pub to: Option<FmpDate>,
}

#[cfg(test)]
mod tests {
  use super::{CotAnalysis, CotReport};

  #[test]
  fn cot_report_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/cot_report.json").unwrap();
    let _: Vec<CotReport> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn cot_analysis_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/cot_analysis.json").unwrap();
    let _: Vec<CotAnalysis> = serde_json::from_slice(&bytes).unwrap();
  }
}
