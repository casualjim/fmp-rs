use crate::macros::define_api_trait;
use crate::types::crypto::{
  CryptoBatchParams, CryptoHistoryParams, CryptoSymbolParams, Cryptocurrency, CryptocurrencyHistoricalChart,
  CryptocurrencyIntradayPrice, CryptocurrencyLightChart, CryptocurrencyQuote, CryptocurrencyShortQuote,
};

define_api_trait!(
  /// API endpoints for crypto.
  CryptoApi,
  cryptocurrency_list -> "/cryptocurrency-list" -> CryptoSymbolParams  -> Vec<Cryptocurrency>,
  quote -> "/quote" -> CryptoSymbolParams  -> Vec<CryptocurrencyQuote>,
  quote_short -> "/quote-short" -> CryptoSymbolParams  -> Vec<CryptocurrencyShortQuote>,
  batch_crypto_quotes -> "/batch-crypto-quotes" -> CryptoBatchParams  -> Vec<CryptocurrencyShortQuote>,
  historical_price_eod_light -> "/historical-price-eod/light" -> CryptoHistoryParams  -> Vec<CryptocurrencyLightChart>,
  historical_price_eod_full -> "/historical-price-eod/full" -> CryptoHistoryParams  -> Vec<CryptocurrencyHistoricalChart>,
  historical_chart_1min -> "/historical-chart/1min" -> CryptoHistoryParams  -> Vec<CryptocurrencyIntradayPrice>,
  historical_chart_5min -> "/historical-chart/5min" -> CryptoHistoryParams  -> Vec<CryptocurrencyIntradayPrice>,
  historical_chart_1hour -> "/historical-chart/1hour" -> CryptoHistoryParams  -> Vec<CryptocurrencyIntradayPrice>,
);
