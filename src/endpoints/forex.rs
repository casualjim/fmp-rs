use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::forex::ForexIntradayChart;
use crate::types::forex::{
  ForexBatchParams, ForexHistoryParams, ForexLightChart, ForexPair, ForexQuote, ForexShortQuote, ForexSymbolParams,
};

pub async fn forex_list(http: &FmpHttpClient) -> FmpResult<Vec<ForexPair>> {
  http.get_json("/forex-list", &()).await
}

pub async fn quote(http: &FmpHttpClient, params: ForexSymbolParams) -> FmpResult<Vec<ForexQuote>> {
  http.get_json("/quote", &params).await
}

pub async fn quote_short(http: &FmpHttpClient, params: ForexSymbolParams) -> FmpResult<Vec<ForexShortQuote>> {
  http.get_json("/quote-short", &params).await
}

pub async fn batch_forex_quotes(http: &FmpHttpClient, params: ForexBatchParams) -> FmpResult<Vec<ForexShortQuote>> {
  http.get_json("/batch-forex-quotes", &params).await
}

pub async fn historical_price_eod_light(
  http: &FmpHttpClient,
  params: ForexHistoryParams,
) -> FmpResult<Vec<ForexLightChart>> {
  http.get_json("/historical-price-eod/light", &params).await
}

pub async fn historical_price_eod_full(
  http: &FmpHttpClient,
  params: ForexHistoryParams,
) -> FmpResult<Vec<crate::types::forex::ForexHistoricalChart>> {
  http.get_json("/historical-price-eod/full", &params).await
}

pub async fn historical_chart_1min(
  http: &FmpHttpClient,
  params: ForexHistoryParams,
) -> FmpResult<Vec<ForexIntradayChart>> {
  http.get_json("/historical-chart/1min", &params).await
}

pub async fn historical_chart_5min(
  http: &FmpHttpClient,
  params: ForexHistoryParams,
) -> FmpResult<Vec<ForexIntradayChart>> {
  http.get_json("/historical-chart/5min", &params).await
}

pub async fn historical_chart_1hour(
  http: &FmpHttpClient,
  params: ForexHistoryParams,
) -> FmpResult<Vec<ForexIntradayChart>> {
  http.get_json("/historical-chart/1hour", &params).await
}
