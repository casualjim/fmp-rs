use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::technical_indicators::{
  AdxIndicator, DemaIndicator, EmaIndicator, RsiIndicator, SmaIndicator, StandardDeviationIndicator,
  TechnicalIndicatorParams, TemaIndicator, WilliamsIndicator, WmaIndicator,
};

pub async fn sma(http: &FmpHttpClient, params: TechnicalIndicatorParams) -> FmpResult<Vec<SmaIndicator>> {
  http.get_json("/technical-indicators/sma", &params).await
}

pub async fn ema(http: &FmpHttpClient, params: TechnicalIndicatorParams) -> FmpResult<Vec<EmaIndicator>> {
  http.get_json("/technical-indicators/ema", &params).await
}

pub async fn wma(http: &FmpHttpClient, params: TechnicalIndicatorParams) -> FmpResult<Vec<WmaIndicator>> {
  http.get_json("/technical-indicators/wma", &params).await
}

pub async fn dema(http: &FmpHttpClient, params: TechnicalIndicatorParams) -> FmpResult<Vec<DemaIndicator>> {
  http.get_json("/technical-indicators/dema", &params).await
}

pub async fn tema(http: &FmpHttpClient, params: TechnicalIndicatorParams) -> FmpResult<Vec<TemaIndicator>> {
  http.get_json("/technical-indicators/tema", &params).await
}

pub async fn rsi(http: &FmpHttpClient, params: TechnicalIndicatorParams) -> FmpResult<Vec<RsiIndicator>> {
  http.get_json("/technical-indicators/rsi", &params).await
}

pub async fn standard_deviation(
  http: &FmpHttpClient,
  params: TechnicalIndicatorParams,
) -> FmpResult<Vec<StandardDeviationIndicator>> {
  http.get_json("/technical-indicators/standarddeviation", &params).await
}

pub async fn williams(http: &FmpHttpClient, params: TechnicalIndicatorParams) -> FmpResult<Vec<WilliamsIndicator>> {
  http.get_json("/technical-indicators/williams", &params).await
}

pub async fn adx(http: &FmpHttpClient, params: TechnicalIndicatorParams) -> FmpResult<Vec<AdxIndicator>> {
  http.get_json("/technical-indicators/adx", &params).await
}
