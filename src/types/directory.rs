use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanySymbol {
  pub symbol: String,
  pub company_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialStatementSymbol {
  #[serde(flatten)]
  pub base: CompanySymbol,
  pub trading_currency: Option<String>,
  pub reporting_currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CikEntry {
  pub cik: String,
  pub company_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolChange {
  pub date: FmpDate,
  pub company_name: String,
  pub old_symbol: String,
  pub new_symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EtfEntry {
  pub symbol: String,
  pub name: String,
}

pub type ActivelyTradingEntry = EtfEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsTranscriptEntry {
  pub symbol: String,
  pub company_name: String,
  pub no_of_transcripts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeEntry {
  pub exchange: String,
  pub name: String,
  pub country_name: String,
  pub country_code: String,
  pub symbol_suffix: String,
  pub delay: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorEntry {
  pub sector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryEntry {
  pub industry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryEntry {
  pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct CikListParams {
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolChangeParams {
  pub invalid: Option<bool>,
  pub limit: Option<u32>,
}

#[cfg(test)]
mod tests {
  use super::{
    CikEntry, CompanySymbol, CountryEntry, ExchangeEntry, FinancialStatementSymbol, IndustryEntry, SectorEntry,
  };

  #[test]
  fn stock_list_allows_null_company_name() {
    let data: Vec<CompanySymbol> = serde_json::from_str(
      r#"[
        {"symbol":"603459.SS","companyName":null}
      ]"#,
    )
    .unwrap();

    assert_eq!(data[0].symbol, "603459.SS");
    assert_eq!(data[0].company_name, None);
  }

  #[test]
  fn financial_statement_symbol_list_allows_null_currencies() {
    let data: Vec<FinancialStatementSymbol> = serde_json::from_str(
      r#"[
        {
          "symbol":"0010F0.KQ",
          "companyName":"Bowon Chemical Co Ltd",
          "tradingCurrency":null,
          "reportingCurrency":null
        }
      ]"#,
    )
    .unwrap();

    assert_eq!(data[0].base.symbol, "0010F0.KQ");
    assert_eq!(data[0].base.company_name.as_deref(), Some("Bowon Chemical Co Ltd"));
    assert_eq!(data[0].trading_currency, None);
    assert_eq!(data[0].reporting_currency, None);
  }

  #[test]
  fn available_exchanges_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/available_exchanges.json").unwrap();
    let _: Vec<ExchangeEntry> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn available_sectors_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/available_sectors.json").unwrap();
    let _: Vec<SectorEntry> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn available_industries_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/available_industries.json").unwrap();
    let _: Vec<IndustryEntry> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn available_countries_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/available_countries.json").unwrap();
    let _: Vec<CountryEntry> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn cik_list_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/cik_list.json").unwrap();
    let _: Vec<CikEntry> = serde_json::from_slice(&bytes).unwrap();
  }
}
