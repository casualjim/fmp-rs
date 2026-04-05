use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecFiling {
  pub symbol: String,
  pub cik: String,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub form_type: String,
  #[serde(default)]
  pub has_financials: Option<bool>,
  pub link: String,
  pub final_link: String,
}

pub type SecFilingFormType = SecFiling;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanySearchResult {
  pub symbol: String,
  pub name: String,
  pub cik: String,
  pub sic_code: String,
  pub industry_title: String,
  pub business_address: String,
  pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecCompanyProfile {
  pub symbol: String,
  pub cik: String,
  pub registrant_name: String,
  pub sic_code: String,
  pub sic_description: String,
  pub sic_group: String,
  pub isin: String,
  pub business_address: String,
  pub mailing_address: String,
  pub phone_number: String,
  pub postal_code: String,
  pub city: String,
  pub state: String,
  pub country: String,
  pub description: String,
  pub ceo: String,
  pub website: String,
  pub exchange: String,
  pub state_location: String,
  pub state_of_incorporation: String,
  pub fiscal_year_end: String,
  #[serde(default)]
  pub ipo_date: Option<FmpDate>,
  #[serde(default)]
  pub employees: Option<String>,
  pub sec_filings_url: String,
  pub tax_identification_number: String,
  #[serde(default)]
  pub fifty_two_week_range: Option<String>,
  #[serde(default)]
  pub is_active: Option<bool>,
  pub asset_type: String,
  pub open_figi_composite: String,
  pub price_currency: String,
  #[serde(default)]
  pub market_sector: Option<String>,
  #[serde(default)]
  pub security_type: Option<String>,
  #[serde(default)]
  pub is_etf: Option<bool>,
  #[serde(default)]
  pub is_adr: Option<bool>,
  #[serde(default)]
  pub is_fund: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryClassification {
  pub office: String,
  pub sic_code: String,
  pub industry_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct DateRangeParams {
  pub from: Option<FmpDate>,
  pub to: Option<FmpDate>,
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

pub type Form8kParams = DateRangeParams;
pub type FinancialsParams = DateRangeParams;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FormTypeParams {
  pub form_type: String,
  #[serde(flatten)]
  #[builder(default)]
  pub range: DateRangeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolParams {
  pub symbol: String,
  #[serde(flatten)]
  #[builder(default)]
  pub range: DateRangeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CikParams {
  pub cik: String,
  #[serde(flatten)]
  #[builder(default)]
  pub range: DateRangeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CompanyNameSearchParams {
  pub company: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CompanySymbolSearchParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CompanyCikSearchParams {
  pub cik: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfileParams {
  pub symbol: Option<String>,
  pub cik: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct IndustrySearchParams {
  pub industry_title: Option<String>,
  pub sic_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct IndustryClassificationSearchParams {
  pub symbol: Option<String>,
  pub cik: Option<String>,
  pub sic_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct AllIndustryClassificationParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[cfg(test)]
mod tests {
  use super::{CompanySearchResult, IndustryClassification, SecCompanyProfile, SecFiling};

  #[test]
  fn sec_filing_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/sec_filing.json").unwrap();
    let _: Vec<SecFiling> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn sec_company_search_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/sec_company_search.json").unwrap();
    let _: Vec<CompanySearchResult> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn sec_company_profile_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/sec_company_profile.json").unwrap();
    let _: Vec<SecCompanyProfile> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn industry_classification_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/industry_classification.json").unwrap();
    let _: Vec<IndustryClassification> = serde_json::from_slice(&bytes).unwrap();
  }
}
