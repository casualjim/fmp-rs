use std::{env, time::Duration};

use fmp::{
  FmpConfig, FmpError, FmpHttpClient, NewsApi, QuotesApi,
  types::{
    news::NewsParams,
    quotes::{BatchQuoteParams, QuoteParams},
  },
};

/// Build a client from `FMP_API_KEY`; panics if the key is missing.
fn live_client() -> FmpHttpClient {
  let api_key = env::var("FMP_API_KEY").expect("set FMP_API_KEY for live tests");
  assert!(!api_key.trim().is_empty(), "FMP_API_KEY must not be empty");
  let config = FmpConfig::builder()
    .api_key(api_key)
    .timeout(Duration::from_millis(15_000))
    .retry_attempts(1u32)
    .build();
  FmpHttpClient::new(config).expect("build live client")
}

#[tokio::test]
async fn quote_returns_price_for_symbol() -> eyre::Result<()> {
  let client = live_client();

  let params = QuoteParams::builder().symbol("AAPL").build();
  let quotes = client.quote(params).await?;

  let aapl = quotes.iter().find(|q| q.symbol == "AAPL");
  let quote = aapl.expect("AAPL quote missing");
  assert!(quote.price.is_finite() && quote.price > 0.0);
  Ok(())
}

#[tokio::test]
async fn news_respects_limit() -> eyre::Result<()> {
  let client = live_client();

  let params = NewsParams::builder().limit(3u32).build();
  let items = client.stock_news(params).await?;

  assert!(!items.is_empty());
  assert!(items.len() <= 3);
  Ok(())
}

#[tokio::test]
async fn invalid_key_surfaces_api_error() {
  let config = FmpConfig::builder()
    .api_key("invalid-api-key")
    .timeout(Duration::from_millis(5_000))
    .retry_attempts(1u32)
    .build();
  let client = FmpHttpClient::new(config).expect("client");

  let err = client
    .quote(QuoteParams::builder().symbol("AAPL").build())
    .await
    .expect_err("expected API error");

  let fmp_err = err.downcast::<FmpError>().expect("downcast to FmpError");
  match fmp_err {
    FmpError::Api { status, .. } => assert!(status.is_client_error(), "unexpected status {status}"),
    other => panic!("expected API error, got {other:?}"),
  }
}

#[tokio::test]
async fn batch_quote_returns_multiple_symbols() -> eyre::Result<()> {
  let client = live_client();

  let params = BatchQuoteParams::builder().symbols("AAPL,MSFT").build();
  let quotes = client.batch_quote(params).await?;

  assert!(quotes.iter().any(|q| q.symbol == "AAPL"));
  assert!(quotes.iter().any(|q| q.symbol == "MSFT"));
  Ok(())
}
