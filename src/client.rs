use http::StatusCode;
use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use secrecy::ExposeSecret;
use serde::Serialize;
use serde_json::from_str;
use std::sync::Arc;

use crate::{
  config::FmpConfig,
  errors::{FmpError, FmpResult},
  make_reqwest_client,
  primitives::FmpErrorPayload,
};

/// Thin wrapper over the shared reqwest middleware client that appends the API key.
#[derive(Clone)]
pub struct FmpHttpClient {
  inner: ClientWithMiddleware,
  config: FmpConfig,
}

impl FmpHttpClient {
  pub fn new(config: FmpConfig) -> FmpResult<Self> {
    config.validate()?;
    let client = make_reqwest_client(&config)?;
    Ok(Self { inner: client, config })
  }

  pub fn into_arc(self) -> Arc<Self> {
    Arc::new(self)
  }

  /// Build a URL relative to the configured base and attach the `apikey` query parameter.
  fn url_with_api_key(&self, path: &str) -> FmpResult<Url> {
    let mut url = self
      .config
      .base_url
      .join(path.trim_start_matches('/'))
      .map_err(|err| FmpError::config(err.to_string()))?;
    url
      .query_pairs_mut()
      .append_pair("apikey", self.config.api_key.expose_secret());
    Ok(url)
  }

  /// Convert HTTP responses that include the FMP error envelope into `FmpError`.
  async fn map_api_error(status: StatusCode, body: &str) -> FmpError {
    let payload: Result<FmpErrorPayload, _> = from_str(body);
    let message = payload
      .as_ref()
      .map(|p| p.message.clone())
      .unwrap_or_else(|_| body.to_string());
    FmpError::api(status, message, payload.ok())
  }

  /// Generic GET returning JSON.
  pub async fn get_json<T, P>(&self, path: &str, params: &P) -> FmpResult<T>
  where
    T: serde::de::DeserializeOwned,
    P: Serialize,
  {
    let mut url = self.url_with_api_key(path)?;
    let query = serde_urlencoded::to_string(params)?;
    if !query.is_empty() {
      // Preserve the existing `apikey` while adding user-supplied parameters.
      let combined = match url.query() {
        Some(existing) if !existing.is_empty() => format!("{existing}&{query}"),
        _ => query,
      };
      url.set_query(Some(&combined));
    }

    let resp = self.inner.get(url).send().await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
      return Err(Self::map_api_error(status, &text).await.into());
    }
    let value = serde_json::from_str(&text)?;
    Ok(value)
  }
}
