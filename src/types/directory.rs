use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanySymbol {
  pub symbol: String,
  pub company_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialStatementSymbol {
  #[serde(flatten)]
  pub base: CompanySymbol,
  pub trading_currency: String,
  pub reporting_currency: String,
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
  pub delay: String,
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
