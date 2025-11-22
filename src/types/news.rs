use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FmpArticle {
  pub title: String,
  pub date: FmpDate,
  pub content: String,
  pub tickers: String,
  pub image: String,
  pub link: String,
  pub author: String,
  pub site: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsArticle {
  #[serde(default)]
  pub symbol: Option<String>,
  pub published_date: FmpDateTime,
  pub publisher: String,
  pub title: String,
  pub image: String,
  pub site: String,
  pub text: String,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct NewsParams {
  pub from: Option<FmpDate>,
  pub to: Option<FmpDate>,
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct NewsSearchParams {
  pub symbols: String,
  #[serde(flatten)]
  #[builder(default)]
  pub params: NewsParams,
}
