use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::economics::{
  DateRangeParams, EconomicCalendar, EconomicIndicator, EconomicIndicatorParams, MarketRiskPremium, TreasuryRate,
};

pub async fn treasury_rates(http: &FmpHttpClient, params: DateRangeParams) -> FmpResult<Vec<TreasuryRate>> {
  http.get_json("/treasury-rates", &params).await
}

pub async fn economic_indicator(
  http: &FmpHttpClient,
  params: EconomicIndicatorParams,
) -> FmpResult<Vec<EconomicIndicator>> {
  http.get_json("/economic-indicator", &params).await
}

pub async fn economic_calendar(http: &FmpHttpClient, params: DateRangeParams) -> FmpResult<Vec<EconomicCalendar>> {
  http.get_json("/economic-calendar", &params).await
}

pub async fn market_risk_premium(http: &FmpHttpClient) -> FmpResult<Vec<MarketRiskPremium>> {
  http.get_json("/market-risk-premium", &()).await
}
