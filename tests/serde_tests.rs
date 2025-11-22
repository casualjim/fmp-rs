use fmp_rs::types::{
  calendar::EarningsReport,
  chart::ChartData,
  company::CompanyProfile,
  news::NewsArticle,
  quotes::StockQuote,
};
use serde_json::from_slice;

fn load(path: &str) -> Vec<u8> {
  std::fs::read(path).expect("fixture read")
}

#[test]
fn profile_fixture_deserializes() {
  let bytes = load("tests/fixtures/profile.json");
  let _: Vec<CompanyProfile> = from_slice(&bytes).expect("profile fixture");
}

#[test]
fn quote_fixture_deserializes() {
  let bytes = load("tests/fixtures/quote.json");
  let _: Vec<StockQuote> = from_slice(&bytes).expect("quote fixture");
}

#[test]
fn calendar_earnings_fixture_deserializes() {
  let bytes = load("tests/fixtures/calendar_earnings.json");
  let _: Vec<EarningsReport> = from_slice(&bytes).expect("earnings fixture");
}

#[test]
fn news_fixture_deserializes() {
  let bytes = load("tests/fixtures/news.json");
  let _: Vec<NewsArticle> = from_slice(&bytes).expect("news fixture");
}

#[test]
fn chart_fixture_deserializes() {
  let bytes = load("tests/fixtures/chart.json");
  let _: Vec<ChartData> = from_slice(&bytes).expect("chart fixture");
}
