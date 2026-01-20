use crate::macros::define_api_trait;
use crate::types::chart::{ChartData, ChartHistoryParams, IntradayChartData, LightChartData, UnadjustedChartData};
use crate::types::chart::{ChartIntradayParams, Interval};

define_api_trait!(
  /// API endpoints for historical price charts.
  ChartApi,
  historical_price_eod_light -> "/historical-price-eod/light" -> ChartHistoryParams  -> Vec<LightChartData>,
  historical_price_eod_full -> "/historical-price-eod/full" -> ChartHistoryParams  -> Vec<ChartData>,
  historical_price_eod_non_split_adjusted -> "/historical-price-eod/non-split-adjusted" -> ChartHistoryParams  -> Vec<UnadjustedChartData>,
  historical_price_eod_dividend_adjusted -> "/historical-price-eod/dividend-adjusted" -> ChartHistoryParams  -> Vec<UnadjustedChartData>,
);

/// Extension trait for chart endpoints with interval-based path selection.
pub trait ChartIntervalApi {
  fn historical_chart_interval(
    &self,
    interval: Interval,
    params: ChartIntradayParams,
  ) -> impl std::future::Future<Output = crate::errors::FmpResult<Vec<IntradayChartData>>> + Send;
}

impl ChartIntervalApi for crate::client::FmpHttpClient {
  fn historical_chart_interval(
    &self,
    interval: Interval,
    params: ChartIntradayParams,
  ) -> impl std::future::Future<Output = crate::errors::FmpResult<Vec<IntradayChartData>>> + Send {
    async move {
      let path = match interval {
        Interval::I1Min => "/historical-chart/1min",
        Interval::I5Min => "/historical-chart/5min",
        Interval::I15Min => "/historical-chart/15min",
        Interval::I30Min => "/historical-chart/30min",
        Interval::I1Hour => "/historical-chart/1hour",
        Interval::I4Hour => "/historical-chart/4hour",
      };
      self.get_json(path, &params).await
    }
  }
}
