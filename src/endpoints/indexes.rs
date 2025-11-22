use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::indexes::{
  HistoricalIndexChange, IndexBatchParams, IndexConstituent, IndexFullChart, IndexHistoryParams, IndexIntradayData,
  IndexItem, IndexLightChart, IndexQuote, IndexShortQuote, IndexSymbolParams,
};

pub async fn index_list(http: &FmpHttpClient) -> FmpResult<Vec<IndexItem>> {
  http.get_json("/index-list", &()).await
}

pub async fn quote(http: &FmpHttpClient, params: IndexSymbolParams) -> FmpResult<Vec<IndexQuote>> {
  http.get_json("/quote", &params).await
}

pub async fn quote_short(http: &FmpHttpClient, params: IndexSymbolParams) -> FmpResult<Vec<IndexShortQuote>> {
  http.get_json("/quote-short", &params).await
}

pub async fn batch_index_quotes(http: &FmpHttpClient, params: IndexBatchParams) -> FmpResult<Vec<IndexShortQuote>> {
  http.get_json("/batch-index-quotes", &params).await
}

pub async fn historical_price_eod_light(
  http: &FmpHttpClient,
  params: IndexHistoryParams,
) -> FmpResult<Vec<IndexLightChart>> {
  http.get_json("/historical-price-eod/light", &params).await
}

pub async fn historical_price_eod_full(
  http: &FmpHttpClient,
  params: IndexHistoryParams,
) -> FmpResult<Vec<IndexFullChart>> {
  http.get_json("/historical-price-eod/full", &params).await
}

pub async fn historical_chart_1min(
  http: &FmpHttpClient,
  params: IndexHistoryParams,
) -> FmpResult<Vec<IndexIntradayData>> {
  http.get_json("/historical-chart/1min", &params).await
}

pub async fn historical_chart_5min(
  http: &FmpHttpClient,
  params: IndexHistoryParams,
) -> FmpResult<Vec<IndexIntradayData>> {
  http.get_json("/historical-chart/5min", &params).await
}

pub async fn historical_chart_1hour(
  http: &FmpHttpClient,
  params: IndexHistoryParams,
) -> FmpResult<Vec<IndexIntradayData>> {
  http.get_json("/historical-chart/1hour", &params).await
}

pub async fn sp500_constituent(http: &FmpHttpClient) -> FmpResult<Vec<IndexConstituent>> {
  http.get_json("/sp500-constituent", &()).await
}

pub async fn nasdaq_constituent(http: &FmpHttpClient) -> FmpResult<Vec<IndexConstituent>> {
  http.get_json("/nasdaq-constituent", &()).await
}

pub async fn dowjones_constituent(http: &FmpHttpClient) -> FmpResult<Vec<IndexConstituent>> {
  http.get_json("/dowjones-constituent", &()).await
}

pub async fn historical_sp500_constituent(http: &FmpHttpClient) -> FmpResult<Vec<HistoricalIndexChange>> {
  http.get_json("/historical-sp500-constituent", &()).await
}

pub async fn historical_nasdaq_constituent(http: &FmpHttpClient) -> FmpResult<Vec<HistoricalIndexChange>> {
  http.get_json("/historical-nasdaq-constituent", &()).await
}

pub async fn historical_dowjones_constituent(http: &FmpHttpClient) -> FmpResult<Vec<HistoricalIndexChange>> {
  http.get_json("/historical-dowjones-constituent", &()).await
}
