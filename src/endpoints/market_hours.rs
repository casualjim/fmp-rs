use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::market_hours::{ExchangeMarketHours, ExchangeParams, HolidayByExchange, HolidaysByExchangeParams};

pub async fn exchange_market_hours(
  http: &FmpHttpClient,
  params: ExchangeParams,
) -> FmpResult<Vec<ExchangeMarketHours>> {
  http.get_json("/exchange-market-hours", &params).await
}

pub async fn holidays_by_exchange(
  http: &FmpHttpClient,
  params: HolidaysByExchangeParams,
) -> FmpResult<Vec<HolidayByExchange>> {
  http.get_json("/holidays-by-exchange", &params).await
}

pub async fn all_exchange_market_hours(http: &FmpHttpClient) -> FmpResult<Vec<ExchangeMarketHours>> {
  http.get_json("/all-exchange-market-hours", &()).await
}
