use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundHolding {
  pub symbol: String,
  pub asset: String,
  pub name: String,
  pub isin: String,
  pub security_cusip: String,
  pub shares_number: f64,
  pub weight_percentage: f64,
  pub market_value: f64,
  #[serde(default)]
  pub updated_at: Option<FmpDateTime>,
  #[serde(default)]
  pub updated: Option<FmpDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundSector {
  pub industry: String,
  pub exposure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundInfo {
  pub symbol: String,
  pub name: String,
  pub description: String,
  pub isin: String,
  pub asset_class: String,
  pub security_cusip: String,
  pub domicile: String,
  pub website: String,
  pub etf_company: String,
  pub expense_ratio: f64,
  pub assets_under_management: f64,
  pub avg_volume: f64,
  pub inception_date: FmpDate,
  pub nav: f64,
  pub nav_currency: String,
  pub holdings_count: f64,
  pub updated_at: FmpDateTime,
  pub sectors_list: Vec<FundSector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundCountryAllocation {
  pub country: String,
  pub weight_percentage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundAssetExposure {
  pub symbol: String,
  pub asset: String,
  pub shares_number: f64,
  pub weight_percentage: f64,
  pub market_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundSectorWeighting {
  pub symbol: String,
  pub sector: String,
  pub weight_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundDisclosureHolder {
  pub cik: String,
  pub holder: String,
  pub shares: f64,
  pub date_reported: FmpDate,
  pub change: f64,
  pub weight_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundDisclosureSearch {
  pub symbol: String,
  pub cik: String,
  pub class_id: String,
  pub series_id: String,
  pub entity_name: String,
  pub entity_org_type: String,
  pub series_name: String,
  pub class_name: String,
  pub reporting_file_number: String,
  pub address: String,
  pub city: String,
  pub zip_code: String,
  pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundDisclosureDate {
  pub date: FmpDate,
  pub year: i32,
  pub quarter: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundDisclosure {
  pub cik: String,
  pub date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub symbol: String,
  pub name: String,
  pub lei: String,
  pub title: String,
  pub cusip: String,
  pub isin: String,
  pub balance: f64,
  pub units: String,
  pub cur_cd: String,
  pub val_usd: f64,
  pub pct_val: f64,
  pub payoff_profile: String,
  pub asset_cat: String,
  pub issuer_cat: String,
  pub inv_country: String,
  pub is_restricted_sec: String,
  pub fair_val_level: String,
  pub is_cash_collateral: String,
  pub is_non_cash_collateral: String,
  pub is_loan_by_fund: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FundSymbolParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FundDisclosureSearchParams {
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FundDisclosureDatesParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub cik: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FundDisclosureParams {
  pub symbol: String,
  pub year: i32,
  pub quarter: i32,
  #[builder(default, setter(strip_option))]
  pub cik: Option<String>,
}

#[cfg(test)]
mod tests {
  use super::{
    FundAssetExposure, FundCountryAllocation, FundDisclosureDate, FundDisclosureHolder, FundHolding, FundInfo,
    FundSectorWeighting,
  };

  #[test]
  fn etf_holding_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/etf_holding.json").unwrap();
    let _: Vec<FundHolding> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn etf_info_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/etf_info.json").unwrap();
    let _: Vec<FundInfo> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn etf_country_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/etf_country.json").unwrap();
    let _: Vec<FundCountryAllocation> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn etf_asset_exposure_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/etf_asset_exposure.json").unwrap();
    let _: Vec<FundAssetExposure> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn etf_sector_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/etf_sector.json").unwrap();
    let _: Vec<FundSectorWeighting> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn fund_disclosure_holder_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/fund_disclosure_holder.json").unwrap();
    let _: Vec<FundDisclosureHolder> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn fund_disclosure_dates_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/fund_disclosure_dates.json").unwrap();
    let _: Vec<FundDisclosureDate> = serde_json::from_slice(&bytes).unwrap();
  }
}
