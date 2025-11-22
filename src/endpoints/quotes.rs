use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::quotes::{
  AftermarketQuote, AftermarketTrade, BatchQuoteParams, ExchangeQuoteParams, QuoteParams, ShortParams,
  StockPriceChange, StockQuote, StockQuoteShort,
};

pub async fn quote(http: &FmpHttpClient, params: QuoteParams) -> FmpResult<Vec<StockQuote>> {
  http.get_json("/quote", &params).await
}

pub async fn quote_short(http: &FmpHttpClient, params: QuoteParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/quote-short", &params).await
}

pub async fn aftermarket_trade(http: &FmpHttpClient, params: QuoteParams) -> FmpResult<Vec<AftermarketTrade>> {
  http.get_json("/aftermarket-trade", &params).await
}

pub async fn aftermarket_quote(http: &FmpHttpClient, params: QuoteParams) -> FmpResult<Vec<AftermarketQuote>> {
  http.get_json("/aftermarket-quote", &params).await
}

pub async fn stock_price_change(http: &FmpHttpClient, params: QuoteParams) -> FmpResult<Vec<StockPriceChange>> {
  http.get_json("/stock-price-change", &params).await
}

pub async fn batch_quote(http: &FmpHttpClient, params: BatchQuoteParams) -> FmpResult<Vec<StockQuote>> {
  http.get_json("/batch-quote", &params).await
}

pub async fn batch_quote_short(http: &FmpHttpClient, params: BatchQuoteParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/batch-quote-short", &params).await
}

pub async fn batch_aftermarket_trade(
  http: &FmpHttpClient,
  params: BatchQuoteParams,
) -> FmpResult<Vec<AftermarketTrade>> {
  http.get_json("/batch-aftermarket-trade", &params).await
}

pub async fn batch_aftermarket_quote(
  http: &FmpHttpClient,
  params: BatchQuoteParams,
) -> FmpResult<Vec<AftermarketQuote>> {
  http.get_json("/batch-aftermarket-quote", &params).await
}

pub async fn exchange_quotes(http: &FmpHttpClient, params: ExchangeQuoteParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/batch-exchange-quote", &params).await
}

pub async fn mutual_fund_quotes(http: &FmpHttpClient, params: ShortParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/batch-mutualfund-quotes", &params).await
}

pub async fn etf_quotes(http: &FmpHttpClient, params: ShortParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/batch-etf-quotes", &params).await
}

pub async fn commodity_quotes(http: &FmpHttpClient, params: ShortParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/batch-commodity-quotes", &params).await
}

pub async fn crypto_quotes(http: &FmpHttpClient, params: ShortParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/batch-crypto-quotes", &params).await
}

pub async fn forex_quotes(http: &FmpHttpClient, params: ShortParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/batch-forex-quotes", &params).await
}

pub async fn index_quotes(http: &FmpHttpClient, params: ShortParams) -> FmpResult<Vec<StockQuoteShort>> {
  http.get_json("/batch-index-quotes", &params).await
}
