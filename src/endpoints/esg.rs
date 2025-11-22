use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::esg::{EsgBenchmark, EsgBenchmarkParams, EsgDisclosure, EsgRating, EsgSymbolParams};

pub async fn esg_disclosure(http: &FmpHttpClient, params: EsgSymbolParams) -> FmpResult<Vec<EsgDisclosure>> {
  http.get_json("/esg-disclosure", &params).await
}

pub async fn esg_ratings(http: &FmpHttpClient, params: EsgSymbolParams) -> FmpResult<Vec<EsgRating>> {
  http.get_json("/esg-ratings", &params).await
}

pub async fn esg_benchmark(http: &FmpHttpClient, params: EsgBenchmarkParams) -> FmpResult<Vec<EsgBenchmark>> {
  http.get_json("/esg-benchmark", &params).await
}
