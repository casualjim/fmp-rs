use crate::macros::define_api_trait;
use crate::types::esg::{EsgBenchmark, EsgBenchmarkParams, EsgDisclosure, EsgRating, EsgSymbolParams};

define_api_trait!(
  /// API endpoints for esg.
  EsgApi,
  esg_disclosure -> "/esg-disclosure" -> EsgSymbolParams  -> Vec<EsgDisclosure>,
  esg_ratings -> "/esg-ratings" -> EsgSymbolParams  -> Vec<EsgRating>,
  esg_benchmark -> "/esg-benchmark" -> EsgBenchmarkParams  -> Vec<EsgBenchmark>,
);
