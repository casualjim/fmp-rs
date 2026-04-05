use futures::{StreamExt, stream::BoxStream};
use http::{StatusCode, header::ACCEPT};
use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use secrecy::ExposeSecret;
use serde::Serialize;
use serde_json::from_slice;
use std::sync::Arc;
use tokio_util::io::StreamReader;

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
  fn map_api_error(status: StatusCode, body: &[u8]) -> FmpError {
    let payload: Result<FmpErrorPayload, _> = serde_json::from_slice(body);
    let message = payload
      .as_ref()
      .map(|p| p.message.clone())
      .unwrap_or_else(|_| String::from_utf8_lossy(body).into_owned());
    FmpError::api(status, message, payload.ok())
  }

  /// Build query URL, appending serialized params alongside the existing `apikey`.
  fn build_url<P: Serialize>(&self, path: &str, params: &P) -> FmpResult<Url> {
    let mut url = self.url_with_api_key(path)?;
    let query = serde_urlencoded::to_string(params)?;
    if !query.is_empty() {
      let combined = match url.query() {
        Some(existing) if !existing.is_empty() => format!("{existing}&{query}"),
        _ => query,
      };
      url.set_query(Some(&combined));
    }
    Ok(url)
  }

  /// Generic GET returning JSON.
  pub async fn get_json<T, P>(&self, path: &str, params: &P) -> FmpResult<T>
  where
    T: serde::de::DeserializeOwned,
    P: Serialize,
  {
    let url = self.build_url(path, params)?;
    let resp = self.inner.get(url).header(ACCEPT, "application/json").send().await?;
    let status = resp.status();
    let bytes = resp.bytes().await?;
    if !status.is_success() {
      return Err(Self::map_api_error(status, &bytes).into());
    }
    Ok(from_slice(&bytes)?)
  }

  /// Generic GET returning a streaming CSV response, deserialized record-by-record.
  pub async fn get_csv<T, P>(&self, path: &str, params: &P) -> FmpResult<BoxStream<'static, FmpResult<T>>>
  where
    T: serde::de::DeserializeOwned + Send + 'static,
    P: Serialize,
  {
    let url = self.build_url(path, params)?;
    let resp = self.inner.get(url).header(ACCEPT, "text/csv").send().await?;
    let status = resp.status();
    if !status.is_success() {
      let bytes = resp.bytes().await?;
      return Err(Self::map_api_error(status, &bytes).into());
    }

    let byte_stream = resp.bytes_stream().map(|r| r.map_err(std::io::Error::other));
    let reader = StreamReader::new(byte_stream);
    let deserializer = csv_async::AsyncDeserializer::from_reader(reader);
    let records = deserializer
      .into_deserialize::<T>()
      .map(|r| r.map_err(FmpError::from).map_err(eyre::Error::from))
      .boxed();
    Ok(records)
  }
}

#[cfg(test)]
mod tests {
  use super::FmpHttpClient;
  use crate::{config::FmpConfig, errors::FmpError, types::bulk::PartParams};
  use futures::StreamExt;
  use reqwest::Url;
  use serde::Deserialize;
  use std::{
    sync::{Arc, Mutex},
    time::Duration,
  };
  use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
  };

  #[derive(Debug, Deserialize, PartialEq)]
  struct QuoteRow {
    symbol: String,
    price: f64,
  }

  #[derive(Debug, Deserialize, PartialEq)]
  struct SymbolOnlyRow {
    symbol: String,
  }

  async fn spawn_http_server(response: String) -> (Url, Arc<Mutex<String>>, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let captured_request = Arc::new(Mutex::new(String::new()));
    let captured_request_clone = Arc::clone(&captured_request);

    let handle = tokio::spawn(async move {
      let (mut socket, _) = listener.accept().await.unwrap();
      let mut request = Vec::new();
      let mut buf = [0_u8; 4096];
      loop {
        let read = socket.read(&mut buf).await.unwrap();
        if read == 0 {
          break;
        }
        request.extend_from_slice(&buf[..read]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") {
          break;
        }
      }
      *captured_request_clone.lock().unwrap() = String::from_utf8_lossy(&request).into_owned();
      socket.write_all(response.as_bytes()).await.unwrap();
      socket.shutdown().await.unwrap();
    });

    let base_url = Url::parse(&format!("http://{addr}/")).unwrap();
    (base_url, captured_request, handle)
  }

  fn test_client(base_url: Url) -> FmpHttpClient {
    let config = FmpConfig::builder()
      .api_key("test-key")
      .base_url(base_url)
      .timeout(Duration::from_secs(2))
      .retry_attempts(0_u32)
      .build();
    FmpHttpClient::new(config).unwrap()
  }

  fn http_response(status: &str, content_type: &str, body: &str) -> String {
    format!(
      "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
      body.len()
    )
  }

  #[tokio::test]
  async fn get_csv_streams_records_and_sends_csv_accept_header() {
    let body = "symbol,price\nAAA,1.25\nBBB,2.50\n";
    let response = http_response("200 OK", "text/csv", body);
    let (base_url, captured_request, handle) = spawn_http_server(response).await;
    let client = test_client(base_url);

    let mut stream = client
      .get_csv::<QuoteRow, _>("/profile-bulk", &PartParams { part: 2 })
      .await
      .unwrap();

    assert_eq!(
      stream.next().await.unwrap().unwrap(),
      QuoteRow {
        symbol: "AAA".into(),
        price: 1.25
      }
    );
    assert_eq!(
      stream.next().await.unwrap().unwrap(),
      QuoteRow {
        symbol: "BBB".into(),
        price: 2.50
      }
    );
    assert!(stream.next().await.is_none());

    handle.await.unwrap();
    let request = captured_request.lock().unwrap().clone();
    let request_lower = request.to_ascii_lowercase();
    assert!(request.contains("GET /profile-bulk?apikey=test-key&part=2 HTTP/1.1"));
    assert!(request_lower.contains("accept: text/csv"));
  }

  #[tokio::test]
  async fn get_csv_with_unit_params_only_appends_apikey() {
    let body = "symbol\nAAA\n";
    let response = http_response("200 OK", "text/csv", body);
    let (base_url, captured_request, handle) = spawn_http_server(response).await;
    let client = test_client(base_url);

    let mut stream = client
      .get_csv::<SymbolOnlyRow, _>("/key-metrics-ttm-bulk", &())
      .await
      .unwrap();
    assert_eq!(
      stream.next().await.unwrap().unwrap(),
      SymbolOnlyRow { symbol: "AAA".into() }
    );
    assert!(stream.next().await.is_none());

    handle.await.unwrap();
    let request = captured_request.lock().unwrap().clone();
    assert!(request.contains("GET /key-metrics-ttm-bulk?apikey=test-key HTTP/1.1"));
  }

  #[tokio::test]
  async fn get_csv_returns_api_error_before_streaming_on_non_success_status() {
    let body = r#"{"Error Message":"rate limited"}"#;
    let response = http_response("429 Too Many Requests", "application/json", body);
    let (base_url, captured_request, handle) = spawn_http_server(response).await;
    let client = test_client(base_url);

    let err = match client
      .get_csv::<QuoteRow, _>("/profile-bulk", &PartParams { part: 0 })
      .await
    {
      Ok(_) => panic!("expected API error"),
      Err(err) => err,
    };

    handle.await.unwrap();
    let request = captured_request.lock().unwrap().clone();
    let request_lower = request.to_ascii_lowercase();
    assert!(request_lower.contains("accept: text/csv"));

    let fmp_err = err.downcast::<FmpError>().unwrap();
    match fmp_err {
      FmpError::Api { status, message, .. } => {
        assert_eq!(status, http::StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(message, "rate limited");
      }
      other => panic!("expected API error, got {other:?}"),
    }
  }

  #[tokio::test]
  async fn get_csv_surfaces_row_parse_errors_and_continues_streaming() {
    let body = "symbol,price\nAAA,1.25\nBBB,not-a-number\nCCC,3.75\n";
    let response = http_response("200 OK", "text/csv", body);
    let (base_url, _captured_request, handle) = spawn_http_server(response).await;
    let client = test_client(base_url);

    let mut stream = client
      .get_csv::<QuoteRow, _>("/profile-bulk", &PartParams { part: 1 })
      .await
      .unwrap();

    assert_eq!(
      stream.next().await.unwrap().unwrap(),
      QuoteRow {
        symbol: "AAA".into(),
        price: 1.25
      }
    );

    let err = stream.next().await.unwrap().unwrap_err();
    let fmp_err = err.downcast::<FmpError>().unwrap();
    assert!(matches!(fmp_err, FmpError::CsvParse(_)));

    assert_eq!(
      stream.next().await.unwrap().unwrap(),
      QuoteRow {
        symbol: "CCC".into(),
        price: 3.75
      }
    );
    assert!(stream.next().await.is_none());

    handle.await.unwrap();
  }
}
