use std::io::{Read, Result};

use async_compression::tokio::bufread::XzDecoder;
use tokio::io::BufReader as TokioBufReader;

fn compressed_path(path: &str) -> String {
  format!("{path}.xz")
}

pub fn read_fixture_bytes(path: &str) -> Result<Vec<u8>> {
  let file = std::fs::File::open(compressed_path(path))?;
  let mut decoder = xz2::read::XzDecoder::new(std::io::BufReader::new(file));
  let mut bytes = Vec::new();
  decoder.read_to_end(&mut bytes)?;
  Ok(bytes)
}

pub async fn open_fixture_xz_reader(path: &str) -> Result<XzDecoder<TokioBufReader<tokio::fs::File>>> {
  let file = tokio::fs::File::open(compressed_path(path)).await?;
  Ok(XzDecoder::new(TokioBufReader::new(file)))
}
