mod client;
mod config;
pub mod endpoints;
mod errors;
mod primitives;
pub mod types;

pub use crate::client::FmpHttpClient;
pub use crate::config::FmpConfig;
pub use crate::errors::{FmpError, FmpResult};
pub use crate::primitives::{FmpErrorPayload, FmpResponse};

use http::{HeaderMap, header::CONTENT_TYPE};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use reqwest_tracing::TracingMiddleware;

/// Construct the shared reqwest middleware client.
/// KEEP: this is the canonical client builder (do not fork settings elsewhere).
pub(crate) fn make_reqwest_client(config: &FmpConfig) -> eyre::Result<reqwest_middleware::ClientWithMiddleware> {
  let mut default_headers = HeaderMap::new();
  default_headers.insert(CONTENT_TYPE, "application/json".parse()?);

  let client = reqwest::Client::builder()
    .user_agent(format!("fmp-rs/{}", env!("CARGO_PKG_VERSION")))
    .brotli(true)
    .gzip(true)
    .zstd(true)
    .deflate(true)
    .connect_timeout(config.timeout)
    .timeout(config.timeout)
    .default_headers(default_headers)
    .build()?;

  let client = reqwest_middleware::ClientBuilder::new(client)
    .with(RetryTransientMiddleware::new_with_policy(
      ExponentialBackoff::builder().build_with_max_retries(config.retry_attempts),
    ))
    .with(TracingMiddleware::default())
    .build();

  Ok(client)
}
