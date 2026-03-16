use crate::macros::define_api_trait;
use crate::types::quotes::{
  AftermarketQuote, AftermarketTrade, BatchQuoteParams, ExchangeQuoteParams, QuoteParams, ShortParams,
  StockPriceChange, StockQuote, StockQuoteShort,
};

define_api_trait!(
  /// Real-time and delayed price quotes for stocks, ETFs, crypto, forex, commodities, and indexes.
  ///
  /// # Single-symbol endpoints
  /// - `quote` — full quote (price, volume, market cap, 52-week range, moving averages)
  /// - `quote_short` — lightweight quote (price, change, volume only)
  /// - `aftermarket_trade` — most recent extended-hours trade (price, size, timestamp)
  /// - `aftermarket_quote` — extended-hours bid/ask spread and volume
  /// - `stock_price_change` — price returns over 1D / 5D / 1M / 3M / 6M / YTD / 1Y / 3Y / 5Y / 10Y / max
  ///
  /// # Batch endpoints (comma-separated symbols)
  /// - `batch_quote` — full quotes for up to ~500 symbols in one request
  /// - `batch_quote_short` — lightweight quotes for multiple symbols
  /// - `batch_aftermarket_trade` — extended-hours trades for multiple symbols
  /// - `batch_aftermarket_quote` — extended-hours bid/ask for multiple symbols
  ///
  /// # Exchange and asset-class endpoints
  /// - `exchange_quotes` — all quotes for a given exchange (NYSE, NASDAQ, etc.)
  /// - `mutual_fund_quotes` — quotes for all mutual funds
  /// - `etf_quotes` — quotes for all ETFs
  /// - `commodity_quotes` — quotes for all commodity contracts
  /// - `crypto_quotes` — quotes for all cryptocurrencies
  /// - `forex_quotes` — quotes for all forex pairs
  /// - `index_quotes` — quotes for all market indexes
  QuotesApi,
  quote -> "/quote" -> QuoteParams  -> Vec<StockQuote>,
  quote_short -> "/quote-short" -> QuoteParams  -> Vec<StockQuoteShort>,
  aftermarket_trade -> "/aftermarket-trade" -> QuoteParams  -> Vec<AftermarketTrade>,
  aftermarket_quote -> "/aftermarket-quote" -> QuoteParams  -> Vec<AftermarketQuote>,
  stock_price_change -> "/stock-price-change" -> QuoteParams  -> Vec<StockPriceChange>,
  batch_quote -> "/batch-quote" -> BatchQuoteParams  -> Vec<StockQuote>,
  batch_quote_short -> "/batch-quote-short" -> BatchQuoteParams  -> Vec<StockQuoteShort>,
  batch_aftermarket_trade -> "/batch-aftermarket-trade" -> BatchQuoteParams  -> Vec<AftermarketTrade>,
  batch_aftermarket_quote -> "/batch-aftermarket-quote" -> BatchQuoteParams  -> Vec<AftermarketQuote>,
  exchange_quotes -> "/batch-exchange-quote" -> ExchangeQuoteParams  -> Vec<StockQuoteShort>,
  mutual_fund_quotes -> "/batch-mutualfund-quotes" -> ShortParams  -> Vec<StockQuoteShort>,
  etf_quotes -> "/batch-etf-quotes" -> ShortParams  -> Vec<StockQuoteShort>,
  commodity_quotes -> "/batch-commodity-quotes" -> ShortParams  -> Vec<StockQuoteShort>,
  crypto_quotes -> "/batch-crypto-quotes" -> ShortParams  -> Vec<StockQuoteShort>,
  forex_quotes -> "/batch-forex-quotes" -> ShortParams  -> Vec<StockQuoteShort>,
  index_quotes -> "/batch-index-quotes" -> ShortParams  -> Vec<StockQuoteShort>,
);
