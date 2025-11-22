use std::time::Duration;

use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use typed_builder::TypedBuilder;

use crate::errors::{FmpError, FmpResult};

const DEFAULT_BASE_URL: &str = "https://financialmodelingprep.com/stable/";
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(10_000);
const DEFAULT_RETRY_ATTEMPTS: u32 = 3;

#[derive(Clone, Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FmpConfig {
  pub api_key: SecretString,

  #[builder(default = Url::parse(DEFAULT_BASE_URL).expect("valid base url"))]
  pub base_url: Url,

  #[builder(default = DEFAULT_TIMEOUT)]
  pub timeout: Duration,

  #[builder(default = DEFAULT_RETRY_ATTEMPTS)]
  pub retry_attempts: u32,
}

impl FmpConfig {
  pub fn new(api_key: impl Into<SecretString>) -> FmpResult<Self> {
    let cfg = Self::builder().api_key(api_key).build();
    cfg.validate()?;
    Ok(cfg)
  }

  pub fn validate(&self) -> FmpResult<()> {
    if self.api_key.expose_secret().is_empty() {
      return Err(FmpError::config("FMP API key is required").into());
    }
    if self.timeout == Duration::ZERO {
      return Err(FmpError::config("timeout must be greater than zero").into());
    }
    Ok(())
  }
}
