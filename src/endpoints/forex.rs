use crate::macros::define_api_trait;
use crate::types::forex::{
  ForexBatchParams, ForexHistoricalChart, ForexHistoryParams, ForexIntradayChart, ForexLightChart, ForexPair,
  ForexQuote, ForexShortQuote, ForexSymbolParams,
};

define_api_trait!(
  /// API endpoints for forex data.
  ForexApi,
  forex_list -> "/forex-list" -> () -> Vec<ForexPair>,
  quote -> "/quote" -> ForexSymbolParams  -> Vec<ForexQuote>,
  quote_short -> "/quote-short" -> ForexSymbolParams  -> Vec<ForexShortQuote>,
  batch_forex_quotes -> "/batch-forex-quotes" -> ForexBatchParams  -> Vec<ForexShortQuote>,
  historical_price_eod_light -> "/historical-price-eod/light" -> ForexHistoryParams  -> Vec<ForexLightChart>,
  historical_price_eod_full -> "/historical-price-eod/full" -> ForexHistoryParams  -> Vec<ForexHistoricalChart>,
  historical_chart_1min -> "/historical-chart/1min" -> ForexHistoryParams  -> Vec<ForexIntradayChart>,
  historical_chart_5min -> "/historical-chart/5min" -> ForexHistoryParams  -> Vec<ForexIntradayChart>,
  historical_chart_1hour -> "/historical-chart/1hour" -> ForexHistoryParams  -> Vec<ForexIntradayChart>,
);
