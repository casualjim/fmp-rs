use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::crypto::{
  CryptoBatchParams, CryptoHistoryParams, CryptoSymbolParams, Cryptocurrency, CryptocurrencyHistoricalChart,
  CryptocurrencyIntradayPrice, CryptocurrencyLightChart, CryptocurrencyQuote, CryptocurrencyShortQuote,
};

pub async fn cryptocurrency_list(http: &FmpHttpClient) -> FmpResult<Vec<Cryptocurrency>> {
  http.get_json("/cryptocurrency-list", &()).await
}

pub async fn quote(http: &FmpHttpClient, params: CryptoSymbolParams) -> FmpResult<Vec<CryptocurrencyQuote>> {
  http.get_json("/quote", &params).await
}

pub async fn quote_short(http: &FmpHttpClient, params: CryptoSymbolParams) -> FmpResult<Vec<CryptocurrencyShortQuote>> {
  http.get_json("/quote-short", &params).await
}

pub async fn batch_crypto_quotes(
  http: &FmpHttpClient,
  params: CryptoBatchParams,
) -> FmpResult<Vec<CryptocurrencyShortQuote>> {
  http.get_json("/batch-crypto-quotes", &params).await
}

pub async fn historical_price_eod_light(
  http: &FmpHttpClient,
  params: CryptoHistoryParams,
) -> FmpResult<Vec<CryptocurrencyLightChart>> {
  http.get_json("/historical-price-eod/light", &params).await
}

pub async fn historical_price_eod_full(
  http: &FmpHttpClient,
  params: CryptoHistoryParams,
) -> FmpResult<Vec<CryptocurrencyHistoricalChart>> {
  http.get_json("/historical-price-eod/full", &params).await
}

pub async fn historical_chart_1min(
  http: &FmpHttpClient,
  params: CryptoHistoryParams,
) -> FmpResult<Vec<CryptocurrencyIntradayPrice>> {
  http.get_json("/historical-chart/1min", &params).await
}

pub async fn historical_chart_5min(
  http: &FmpHttpClient,
  params: CryptoHistoryParams,
) -> FmpResult<Vec<CryptocurrencyIntradayPrice>> {
  http.get_json("/historical-chart/5min", &params).await
}

pub async fn historical_chart_1hour(
  http: &FmpHttpClient,
  params: CryptoHistoryParams,
) -> FmpResult<Vec<CryptocurrencyIntradayPrice>> {
  http.get_json("/historical-chart/1hour", &params).await
}
