use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A civil date that handles the FMP API's occasional `"YYYY-MM-DDZ"` variant
/// in addition to the standard `"YYYY-MM-DD"` format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct FmpDate(pub jiff::civil::Date);

impl<'de> Deserialize<'de> for FmpDate {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    let raw = String::deserialize(deserializer)?;
    // Try standard YYYY-MM-DD first.
    if let Ok(d) = jiff::civil::Date::from_str(&raw) {
      return Ok(Self(d));
    }
    // Strip a trailing "Z" (some FMP endpoints append it to plain dates).
    let trimmed = raw.trim_end_matches('Z');
    if let Ok(d) = jiff::civil::Date::from_str(trimmed) {
      return Ok(Self(d));
    }
    // Try MM-DD-YYYY format used by some crowdfunding endpoints.
    if raw.len() == 10 && raw.as_bytes()[2] == b'-' && raw.as_bytes()[5] == b'-' {
      let rearranged = format!("{}-{}-{}", &raw[6..10], &raw[0..2], &raw[3..5]);
      if let Ok(d) = jiff::civil::Date::from_str(&rearranged) {
        return Ok(Self(d));
      }
    }
    // Last resort: parse as a full timestamp and extract the date.
    jiff::Timestamp::from_str(&raw)
      .or_else(|_| jiff::Timestamp::from_str(&format!("{}Z", raw.replace(' ', "T"))))
      .map(|ts| ts.to_zoned(jiff::tz::TimeZone::UTC).date())
      .map(Self)
      .map_err(serde::de::Error::custom)
  }
}

impl From<jiff::civil::Date> for FmpDate {
  fn from(d: jiff::civil::Date) -> Self {
    Self(d)
  }
}

/// A timestamp that handles RFC 3339, space-separated `"YYYY-MM-DD HH:MM:SS"`,
/// plain date `"YYYY-MM-DD"` (treated as midnight UTC), and integer Unix timestamps
/// (seconds) that some FMP quote endpoints return.
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct FmpDateTime(pub jiff::Timestamp);

impl<'de> Deserialize<'de> for FmpDateTime {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
      type Value = FmpDateTime;
      fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a timestamp string or integer Unix seconds")
      }
      fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<FmpDateTime, E> {
        jiff::Timestamp::from_second(v).map(FmpDateTime).map_err(E::custom)
      }
      fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<FmpDateTime, E> {
        self.visit_i64(v as i64)
      }
      fn visit_str<E: serde::de::Error>(self, raw: &str) -> Result<FmpDateTime, E> {
        jiff::Timestamp::from_str(raw)
          .or_else(|_| jiff::Timestamp::from_str(&format!("{}Z", raw.replace(' ', "T"))))
          .or_else(|_| {
            // Plain date ("YYYY-MM-DD" or "YYYY-MM-DDZ") → midnight UTC.
            let trimmed = raw.trim_end_matches('Z');
            jiff::civil::Date::from_str(trimmed)
              .and_then(|d| d.to_zoned(jiff::tz::TimeZone::UTC))
              .map(|z| z.timestamp())
          })
          .map(FmpDateTime)
          .map_err(E::custom)
      }
      fn visit_string<E: serde::de::Error>(self, v: String) -> Result<FmpDateTime, E> {
        self.visit_str(&v)
      }
    }
    deserializer.deserialize_any(Visitor)
  }
}

impl From<jiff::Timestamp> for FmpDateTime {
  fn from(ts: jiff::Timestamp) -> Self {
    Self(ts)
  }
}

/// Deserialise an `Option<FmpDate>` treating an empty string `""` as `None`.
///
/// Some FMP endpoints (e.g., equity offerings) emit `""` instead of omitting a
/// date field entirely. Using `#[serde(deserialize_with = "de_opt_fmpdate")]`
/// on an `Option<FmpDate>` field handles both cases.
pub fn de_opt_fmpdate<'de, D>(d: D) -> Result<Option<FmpDate>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let raw = String::deserialize(d)?;
  if raw.is_empty() {
    return Ok(None);
  }
  let inner = serde::de::value::StringDeserializer::<serde::de::value::Error>::new(raw);
  FmpDate::deserialize(inner).map(Some).map_err(serde::de::Error::custom)
}

/// Server-provided error envelope used by FMP.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FmpErrorPayload {
  #[serde(rename = "Error Message")]
  pub message: String,
  #[serde(default)]
  pub code: Option<String>,
  #[serde(default)]
  pub status: Option<u16>,
}

/// Generic response wrapper; many FMP endpoints return raw arrays instead, so this is
/// opt-in for endpoints that expose the metadata structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FmpResponse<T> {
  pub data: T,
  #[serde(default)]
  pub error: Option<FmpErrorPayload>,
  pub timestamp: i64,
}
