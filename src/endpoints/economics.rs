use crate::macros::define_api_trait;
use crate::types::economics::{
  DateRangeParams, EconomicCalendar, EconomicIndicator, EconomicIndicatorParams, MarketRiskPremium, TreasuryRate,
};

define_api_trait!(
  /// API endpoints for economics.
  EconomicsApi,
  treasury_rates -> "/treasury-rates" -> DateRangeParams  -> Vec<TreasuryRate>,
  economic_indicator -> "/economic-indicator" -> EconomicIndicatorParams  -> Vec<EconomicIndicator>,
  economic_calendar -> "/economic-calendar" -> DateRangeParams  -> Vec<EconomicCalendar>,
  market_risk_premium -> "/market-risk-premium" -> () -> Vec<MarketRiskPremium>,
);
