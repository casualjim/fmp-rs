use crate::macros::define_api_trait;
use crate::types::indexes::{
  IndexBatchParams, IndexFullChart, IndexHistoryParams, IndexIntradayData,
  IndexItem, IndexLightChart, IndexQuote, IndexShortQuote, IndexSymbolParams,
};

define_api_trait!(
  /// API endpoints for indexes.
  IndexesApi,
  index_list -> "/index-list" -> IndexSymbolParams  -> Vec<IndexItem>,
  quote -> "/quote" -> IndexSymbolParams  -> Vec<IndexQuote>,
  quote_short -> "/quote-short" -> IndexSymbolParams  -> Vec<IndexShortQuote>,
  batch_index_quotes -> "/batch-index-quotes" -> IndexBatchParams  -> Vec<IndexShortQuote>,
  historical_price_eod_light -> "/historical-price-eod/light" -> IndexHistoryParams  -> Vec<IndexLightChart>,
  historical_price_eod_full -> "/historical-price-eod/full" -> IndexHistoryParams  -> Vec<IndexFullChart>,
  historical_chart_1min -> "/historical-chart/1min" -> IndexHistoryParams  -> Vec<IndexIntradayData>,
  historical_chart_5min -> "/historical-chart/5min" -> IndexHistoryParams  -> Vec<IndexIntradayData>,
  historical_chart_1hour -> "/historical-chart/1hour" -> IndexHistoryParams  -> Vec<IndexIntradayData>,
);
