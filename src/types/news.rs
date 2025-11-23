use serde::{Deserialize, Serialize};
use std::str::FromStr;
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

fn deserialize_news_datetime<'de, D>(deserializer: D) -> Result<FmpDateTime, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let raw = String::deserialize(deserializer)?;
  jiff::Timestamp::from_str(&raw)
    .or_else(|_| {
      let normalized = format!("{}Z", raw.replace(' ', "T"));
      jiff::Timestamp::from_str(&normalized)
    })
    .map_err(serde::de::Error::custom)
}

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
  #[serde(deserialize_with = "deserialize_news_datetime")]
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
