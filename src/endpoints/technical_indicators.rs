use crate::macros::define_api_trait;
use crate::types::technical_indicators::{
  AdxIndicator, DemaIndicator, EmaIndicator, RsiIndicator, SmaIndicator, StandardDeviationIndicator,
  TechnicalIndicatorParams, TemaIndicator, WilliamsIndicator, WmaIndicator,
};

define_api_trait!(
  /// API endpoints for technical_indicators.
  TechnicalIndicatorsApi,
  sma -> "/technical-indicators/sma" -> TechnicalIndicatorParams  -> Vec<SmaIndicator>,
  ema -> "/technical-indicators/ema" -> TechnicalIndicatorParams  -> Vec<EmaIndicator>,
  wma -> "/technical-indicators/wma" -> TechnicalIndicatorParams  -> Vec<WmaIndicator>,
  dema -> "/technical-indicators/dema" -> TechnicalIndicatorParams  -> Vec<DemaIndicator>,
  tema -> "/technical-indicators/tema" -> TechnicalIndicatorParams  -> Vec<TemaIndicator>,
  rsi -> "/technical-indicators/rsi" -> TechnicalIndicatorParams  -> Vec<RsiIndicator>,
  standard_deviation -> "/technical-indicators/standarddeviation" -> TechnicalIndicatorParams  -> Vec<StandardDeviationIndicator>,
  williams -> "/technical-indicators/williams" -> TechnicalIndicatorParams  -> Vec<WilliamsIndicator>,
  adx -> "/technical-indicators/adx" -> TechnicalIndicatorParams  -> Vec<AdxIndicator>,
);
