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

fn de_string_or_default<'de, D>(d: D) -> Result<String, D::Error>
where
  D: serde::Deserializer<'de>,
{
  Ok(Option::<String>::deserialize(d)?.unwrap_or_default())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsArticle {
  #[serde(default)]
  pub symbol: Option<String>,
  pub published_date: FmpDateTime,
  #[serde(default, deserialize_with = "de_string_or_default")]
  pub publisher: String,
  #[serde(default, deserialize_with = "de_string_or_default")]
  pub title: String,
  #[serde(default, deserialize_with = "de_string_or_default")]
  pub image: String,
  #[serde(default, deserialize_with = "de_string_or_default")]
  pub site: String,
  #[serde(default, deserialize_with = "de_string_or_default")]
  pub text: String,
  #[serde(default, deserialize_with = "de_string_or_default")]
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

#[cfg(test)]
mod tests {
  use super::{FmpArticle, NewsArticle};

  #[test]
  fn fmp_article_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/fmp_article.json").unwrap();
    let _: Vec<FmpArticle> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stock_news_fixture_deserializes() {
    let bytes = crate::test_fixtures::read_fixture_bytes("tests/fixtures/stock_news.json").unwrap();
    let _: Vec<NewsArticle> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stock_news_handles_null_string_fields() {
    let bytes = br#"[
      {
        "symbol": "AAPL",
        "publishedDate": "2026-03-17 02:19:00",
        "publisher": null,
        "title": "Sample headline",
        "image": null,
        "site": "example.com",
        "text": null,
        "url": null
      }
    ]"#;

    let items: Vec<NewsArticle> = serde_json::from_slice(bytes).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].publisher, "");
    assert_eq!(items[0].image, "");
    assert_eq!(items[0].text, "");
    assert_eq!(items[0].url, "");
  }
}
