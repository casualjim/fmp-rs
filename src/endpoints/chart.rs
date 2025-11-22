use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::chart::Interval;
use crate::types::chart::{
  ChartData, ChartHistoryParams, ChartIntradayParams, IntradayChartData, LightChartData, UnadjustedChartData,
};

pub async fn historical_price_eod_light(
  http: &FmpHttpClient,
  params: ChartHistoryParams,
) -> FmpResult<Vec<LightChartData>> {
  http.get_json("/historical-price-eod/light", &params).await
}

pub async fn historical_price_eod_full(http: &FmpHttpClient, params: ChartHistoryParams) -> FmpResult<Vec<ChartData>> {
  http.get_json("/historical-price-eod/full", &params).await
}

pub async fn historical_price_eod_non_split_adjusted(
  http: &FmpHttpClient,
  params: ChartHistoryParams,
) -> FmpResult<Vec<UnadjustedChartData>> {
  http.get_json("/historical-price-eod/non-split-adjusted", &params).await
}

pub async fn historical_price_eod_dividend_adjusted(
  http: &FmpHttpClient,
  params: ChartHistoryParams,
) -> FmpResult<Vec<UnadjustedChartData>> {
  http.get_json("/historical-price-eod/dividend-adjusted", &params).await
}

pub async fn historical_chart_interval(
  http: &FmpHttpClient,
  interval: Interval,
  params: ChartIntradayParams,
) -> FmpResult<Vec<IntradayChartData>> {
  let path = match interval {
    Interval::I1Min => "/historical-chart/1min",
    Interval::I5Min => "/historical-chart/5min",
    Interval::I15Min => "/historical-chart/15min",
    Interval::I30Min => "/historical-chart/30min",
    Interval::I1Hour => "/historical-chart/1hour",
    Interval::I4Hour => "/historical-chart/4hour",
  };
  http.get_json(path, &params).await
}
