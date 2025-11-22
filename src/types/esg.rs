use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsgDisclosure {
  pub date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub symbol: String,
  pub cik: String,
  pub company_name: String,
  pub form_type: String,
  #[serde(default)]
  pub environmental_score: Option<f64>,
  #[serde(default)]
  pub social_score: Option<f64>,
  #[serde(default)]
  pub governance_score: Option<f64>,
  #[serde(rename = "ESGScore")]
  pub esg_score: f64,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsgRating {
  pub symbol: String,
  pub cik: String,
  pub company_name: String,
  pub industry: String,
  pub fiscal_year: i32,
  #[serde(rename = "ESGRiskRating")]
  pub esg_risk_rating: String,
  pub industry_rank: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EsgBenchmark {
  pub fiscal_year: i32,
  pub sector: String,
  #[serde(default)]
  pub environmental_score: Option<f64>,
  #[serde(default)]
  pub social_score: Option<f64>,
  #[serde(default)]
  pub governance_score: Option<f64>,
  #[serde(rename = "ESGScore")]
  pub esg_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct EsgSymbolParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct EsgBenchmarkParams {
  pub year: Option<String>,
}
