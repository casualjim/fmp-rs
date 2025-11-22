use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::calendar::{
  CalendarRangeParams, Dividend, EarningsReport, Ipo, IpoDisclosure, IpoProspectus, StockSplit, SymbolLimitParams,
};

pub async fn dividends(http: &FmpHttpClient, params: SymbolLimitParams) -> FmpResult<Vec<Dividend>> {
  http.get_json("/dividends", &params).await
}

pub async fn dividends_calendar(http: &FmpHttpClient, params: CalendarRangeParams) -> FmpResult<Vec<Dividend>> {
  http.get_json("/dividends-calendar", &params).await
}

pub async fn earnings(http: &FmpHttpClient, params: SymbolLimitParams) -> FmpResult<Vec<EarningsReport>> {
  http.get_json("/earnings", &params).await
}

pub async fn earnings_calendar(http: &FmpHttpClient, params: CalendarRangeParams) -> FmpResult<Vec<EarningsReport>> {
  http.get_json("/earnings-calendar", &params).await
}

pub async fn ipos_calendar(http: &FmpHttpClient, params: CalendarRangeParams) -> FmpResult<Vec<Ipo>> {
  http.get_json("/ipos-calendar", &params).await
}

pub async fn ipos_disclosure(http: &FmpHttpClient, params: CalendarRangeParams) -> FmpResult<Vec<IpoDisclosure>> {
  http.get_json("/ipos-disclosure", &params).await
}

pub async fn ipos_prospectus(http: &FmpHttpClient, params: CalendarRangeParams) -> FmpResult<Vec<IpoProspectus>> {
  http.get_json("/ipos-prospectus", &params).await
}

pub async fn splits(http: &FmpHttpClient, params: SymbolLimitParams) -> FmpResult<Vec<StockSplit>> {
  http.get_json("/splits", &params).await
}

pub async fn splits_calendar(http: &FmpHttpClient, params: CalendarRangeParams) -> FmpResult<Vec<StockSplit>> {
  http.get_json("/splits-calendar", &params).await
}
