use http::StatusCode;
use thiserror::Error;

use crate::primitives::FmpErrorPayload;

/// Crate-wide result alias using `eyre` for flexible reporting.
pub type FmpResult<T> = eyre::Result<T>;

/// Domain errors returned by the FMP client.
#[derive(Debug, Error)]
pub enum FmpError {
  #[error("config error: {0}")]
  Config(String),

  #[error("validation error: {0}")]
  Validation(String),

  #[error("http {status}: {message}")]
  Api {
    status: StatusCode,
    message: String,
    payload: Option<FmpErrorPayload>,
  },

  #[error(transparent)]
  Transport(#[from] reqwest::Error),
}

impl FmpError {
  pub fn api(status: StatusCode, message: impl Into<String>, payload: Option<FmpErrorPayload>) -> Self {
    Self::Api {
      status,
      message: message.into(),
      payload,
    }
  }

  pub fn config(message: impl Into<String>) -> Self {
    Self::Config(message.into())
  }

  pub fn validation(message: impl Into<String>) -> Self {
    Self::Validation(message.into())
  }
}
