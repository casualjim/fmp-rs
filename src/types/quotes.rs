use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Real-time or delayed quote for a stock, ETF, index, crypto, or other security.
///
/// Contains price, intraday range, 52-week range, volume, market cap, and moving
/// averages. `timestamp` is a Unix epoch second indicating last update time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockQuote {
  /// Ticker symbol (e.g., "AAPL").
  pub symbol: String,
  /// Full company or security name (e.g., "Apple Inc.").
  pub name: String,
  /// Current market price per share.
  pub price: f64,
  /// Percentage change from the previous session's closing price.
  pub change_percentage: f64,
  /// Absolute dollar change from the previous session's closing price.
  pub change: f64,
  /// Shares traded during the current session.
  pub volume: f64,
  /// Lowest trade price during the current session.
  pub day_low: f64,
  /// Highest trade price during the current session.
  pub day_high: f64,
  /// Highest price over the trailing 52 weeks.
  pub year_high: f64,
  /// Lowest price over the trailing 52 weeks.
  pub year_low: f64,
  /// Total market capitalisation: shares outstanding × current price.
  #[serde(alias = "mktCap")]
  pub market_cap: f64,
  /// 50-day simple moving average of closing prices.
  #[serde(alias = "priceAvg50")]
  pub price_avg50: f64,
  /// 200-day simple moving average of closing prices.
  #[serde(alias = "priceAvg200")]
  pub price_avg200: f64,
  /// Primary exchange the security is listed on (e.g., "NASDAQ", "NYSE").
  pub exchange: String,
  /// Opening price for the current trading session.
  pub open: f64,
  /// Closing price from the previous trading session.
  pub previous_close: f64,
  /// Unix epoch timestamp (seconds) when this quote was last updated.
  pub timestamp: i64,
}

/// Lightweight quote containing only price, change, and volume.
///
/// Use this instead of [`StockQuote`] when you need fast, low-overhead data
/// for many symbols at once.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockQuoteShort {
  /// Ticker symbol.
  pub symbol: String,
  /// Current market price per share.
  pub price: f64,
  /// Absolute dollar change from the previous session's close.
  pub change: f64,
  /// Shares traded during the current session.
  pub volume: f64,
}

/// A single after-hours or pre-market trade for a security.
///
/// Represents an individual transaction that occurred outside regular market
/// hours, with the executed price, share size, and exact timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AftermarketTrade {
  /// Ticker symbol.
  pub symbol: String,
  /// Price at which the trade executed.
  pub price: f64,
  /// Number of shares in the trade.
  pub trade_size: f64,
  /// Unix epoch timestamp (seconds) of the trade.
  pub timestamp: i64,
}

/// After-hours or pre-market bid/ask quote for a security.
///
/// Shows the best bid and ask prices and sizes during extended-hours trading,
/// along with total extended-hours session volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AftermarketQuote {
  /// Ticker symbol.
  pub symbol: String,
  /// Number of shares at the best bid price.
  pub bid_size: f64,
  /// Highest price buyers are currently willing to pay.
  pub bid_price: f64,
  /// Number of shares at the best ask price.
  pub ask_size: f64,
  /// Lowest price sellers are currently willing to accept.
  pub ask_price: f64,
  /// Total shares traded in the extended-hours session so far.
  pub volume: f64,
  /// Unix epoch timestamp (seconds) of this quote snapshot.
  pub timestamp: i64,
}

/// Percentage price changes for a security over standard time horizons.
///
/// All values are percentage returns (e.g., `0.05` = +5%). Useful for
/// quickly comparing performance across periods without fetching full history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPriceChange {
  /// Ticker symbol.
  pub symbol: String,
  /// 1-day (daily) percentage return.
  #[serde(rename = "1D")]
  pub one_day: f64,
  /// 5-day (weekly) percentage return.
  #[serde(rename = "5D")]
  pub five_day: f64,
  /// 1-month percentage return.
  #[serde(rename = "1M")]
  pub one_month: f64,
  /// 3-month percentage return.
  #[serde(rename = "3M")]
  pub three_month: f64,
  /// 6-month percentage return.
  #[serde(rename = "6M")]
  pub six_month: f64,
  /// Year-to-date percentage return.
  pub ytd: f64,
  /// 1-year percentage return.
  #[serde(rename = "1Y")]
  pub one_year: f64,
  /// 3-year percentage return.
  #[serde(rename = "3Y")]
  pub three_year: f64,
  /// 5-year percentage return.
  #[serde(rename = "5Y")]
  pub five_year: f64,
  /// 10-year percentage return.
  #[serde(rename = "10Y")]
  pub ten_year: f64,
  /// All-time percentage return since the security's IPO or inception.
  pub max: f64,
}

// --- Request parameter types ---

/// Parameters for single-symbol quote endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct QuoteParams {
  /// Ticker symbol (e.g., "AAPL"). Some endpoints accept a comma-separated list.
  pub symbol: String,
}

/// Parameters for batch quote endpoints that accept multiple symbols.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct BatchQuoteParams {
  /// Comma-separated list of ticker symbols (e.g., "AAPL,MSFT,GOOGL").
  pub symbols: String,
}

/// Parameters for exchange-wide quote endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ExchangeQuoteParams {
  /// Exchange identifier (e.g., "NYSE", "NASDAQ", "AMEX", "LSE").
  pub exchange: String,
  /// When `true`, returns [`StockQuoteShort`] instead of full quotes.
  #[serde(default)]
  #[builder(default, setter(strip_option))]
  pub short: Option<bool>,
}

/// Optional `short` flag for asset-class batch quote endpoints (mutual funds,
/// ETFs, commodities, crypto, forex, indexes).
///
/// When `short` is `true` the API returns [`StockQuoteShort`] records;
/// when absent or `false` it returns full [`StockQuote`] records.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct ShortParams {
  /// Return lightweight quotes when `true`.
  pub short: Option<bool>,
}
