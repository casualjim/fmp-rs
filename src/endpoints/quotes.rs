use crate::macros::define_api_trait;
use crate::types::quotes::{
  AftermarketQuote, AftermarketTrade, BatchQuoteParams, ExchangeQuoteParams, QuoteParams, ShortParams,
  StockPriceChange, StockQuote, StockQuoteShort,
};

define_api_trait!(
  /// API endpoints for quotes.
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
