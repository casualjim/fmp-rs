use crate::macros::define_api_trait;
use crate::types::market_hours::{ExchangeMarketHours, ExchangeParams, HolidayByExchange, HolidaysByExchangeParams};

define_api_trait!(
  /// API endpoints for market_hours.
  MarketHoursApi,
  exchange_market_hours -> "/exchange-market-hours" -> ExchangeParams  -> Vec<ExchangeMarketHours>,
  holidays_by_exchange -> "/holidays-by-exchange" -> HolidaysByExchangeParams  -> Vec<HolidayByExchange>,
  all_exchange_market_hours -> "/all-exchange-market-hours" -> () -> Vec<ExchangeMarketHours>,
);
