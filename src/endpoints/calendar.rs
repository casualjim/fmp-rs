use crate::macros::define_api_trait;
use crate::types::calendar::{
  CalendarRangeParams, Dividend, EarningsReport, Ipo, IpoDisclosure, IpoProspectus, StockSplit, SymbolLimitParams,
};

define_api_trait!(
  /// API endpoints for calendar.
  CalendarApi,
  dividends -> "/dividends" -> SymbolLimitParams  -> Vec<Dividend>,
  dividends_calendar -> "/dividends-calendar" -> CalendarRangeParams  -> Vec<Dividend>,
  earnings -> "/earnings" -> SymbolLimitParams  -> Vec<EarningsReport>,
  earnings_calendar -> "/earnings-calendar" -> CalendarRangeParams  -> Vec<EarningsReport>,
  ipos_calendar -> "/ipos-calendar" -> CalendarRangeParams  -> Vec<Ipo>,
  ipos_disclosure -> "/ipos-disclosure" -> CalendarRangeParams  -> Vec<IpoDisclosure>,
  ipos_prospectus -> "/ipos-prospectus" -> CalendarRangeParams  -> Vec<IpoProspectus>,
  splits -> "/splits" -> SymbolLimitParams  -> Vec<StockSplit>,
  splits_calendar -> "/splits-calendar" -> CalendarRangeParams  -> Vec<StockSplit>,
);
