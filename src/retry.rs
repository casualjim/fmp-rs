use http::StatusCode;
use reqwest_retry::{RetryDecision, RetryPolicy, policies::ExponentialBackoff};
use reqwest_retry::{Retryable, RetryableStrategy, default_on_request_failure, default_on_request_success};
use std::{
  sync::{Arc, Mutex},
  time::{Duration, SystemTime},
};

/// Shared inner state — held by both the [`RetryPolicy`] and [`RetryableStrategy`] roles.
struct Inner {
  max_retries: u32,
  backoff: ExponentialBackoff,
  /// Duration extracted from a `Retry-After` header on the most recent 429 response.
  /// The strategy sets it; the policy drains it exactly once.
  retry_after: Mutex<Option<Duration>>,
}

/// A combined [`RetryPolicy`] + [`RetryableStrategy`] that:
///
/// - On **429**: reads `Retry-After` (integer-seconds or HTTP-date) and waits that long.
///   Falls back to exponential backoff when no header is present.
/// - On **5xx / 408 / network errors**: delegates to exponential backoff (same as
///   [`ExponentialBackoff`] default behaviour).
///
/// Because both roles share the same `Arc<Inner>`, clone cheaply before passing to
/// [`RetryTransientMiddleware::new_with_policy_and_strategy`].
#[derive(Clone)]
pub struct RetryAfterPolicy {
  inner: Arc<Inner>,
}

impl RetryAfterPolicy {
  pub fn new(max_retries: u32) -> Self {
    let backoff = ExponentialBackoff::builder().build_with_max_retries(max_retries);
    Self {
      inner: Arc::new(Inner {
        max_retries,
        backoff,
        retry_after: Mutex::new(None),
      }),
    }
  }
}

impl RetryPolicy for RetryAfterPolicy {
  fn should_retry(&self, request_start_time: SystemTime, n_past_retries: u32) -> RetryDecision {
    if n_past_retries >= self.inner.max_retries {
      return RetryDecision::DoNotRetry;
    }
    // Drain any Retry-After duration captured by the strategy.
    let retry_after = self.inner.retry_after.lock().ok().and_then(|mut g| g.take());

    if let Some(wait) = retry_after {
      RetryDecision::Retry {
        execute_after: SystemTime::now() + wait,
      }
    } else {
      self.inner.backoff.should_retry(request_start_time, n_past_retries)
    }
  }
}

impl RetryableStrategy for RetryAfterPolicy {
  fn handle(&self, res: &reqwest_middleware::Result<reqwest::Response>) -> Option<Retryable> {
    match res {
      Ok(resp) if resp.status() == StatusCode::TOO_MANY_REQUESTS => {
        if let Some(duration) = resp
          .headers()
          .get(http::header::RETRY_AFTER)
          .and_then(|v| v.to_str().ok())
          .and_then(parse_retry_after)
          && let Ok(mut guard) = self.inner.retry_after.lock()
        {
          *guard = Some(duration);
        }
        Some(Retryable::Transient)
      }
      Ok(resp) => default_on_request_success(resp),
      Err(err) => default_on_request_failure(err),
    }
  }
}

/// Parse a `Retry-After` header value.
///
/// Handles:
/// - Integer seconds: `"60"`
/// - HTTP-date (RFC 7231): `"Wed, 21 Oct 2026 07:28:00 GMT"`
///
/// Returns `None` when the value cannot be parsed.
fn parse_retry_after(value: &str) -> Option<Duration> {
  let trimmed = value.trim();

  // Integer-seconds form.
  if let Ok(secs) = trimmed.parse::<u64>() {
    return Some(Duration::from_secs(secs));
  }

  // HTTP-date form — parse as RFC 2822 / RFC 1123 (same wire format).
  // Format: "Wed, 21 Oct 2026 07:28:00 GMT"
  parse_http_date(trimmed)
}

/// Minimal RFC 7231 HTTP-date parser using only stdlib.
///
/// Expected format: `"Wkd, DD Mon YYYY HH:MM:SS GMT"`
fn parse_http_date(s: &str) -> Option<Duration> {
  // Use jiff for robust parsing of the HTTP-date format.
  let fmt = "%a, %d %b %Y %H:%M:%S GMT";
  let dt = jiff::civil::DateTime::strptime(fmt, s).ok()?;
  let zdt: jiff::Zoned = dt.to_zoned(jiff::tz::TimeZone::UTC).ok()?;
  let now = jiff::Zoned::now();
  let span = zdt.duration_since(&now);
  let std_dur = span.try_into().ok()?;
  Some(std_dur)
}

#[cfg(test)]
mod tests {
  use super::parse_retry_after;
  use std::time::Duration;

  #[test]
  fn retry_after_parses_integer_seconds() {
    assert_eq!(parse_retry_after("60"), Some(Duration::from_secs(60)));
  }

  #[test]
  fn retry_after_parses_http_date() {
    let duration = parse_retry_after("Wed, 21 Oct 2099 07:28:00 GMT").unwrap();
    assert!(duration > Duration::from_secs(0));
  }

  #[test]
  fn retry_after_rejects_invalid_values() {
    assert_eq!(parse_retry_after("not-a-date"), None);
  }
}
