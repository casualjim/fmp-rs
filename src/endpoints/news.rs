use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::news::{FmpArticle, NewsArticle, NewsParams, NewsSearchParams};

pub async fn fmp_articles(http: &FmpHttpClient, params: NewsParams) -> FmpResult<Vec<FmpArticle>> {
  http.get_json("/fmp-articles", &params).await
}

pub async fn general_news(http: &FmpHttpClient, params: NewsParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/general-latest", &params).await
}

pub async fn press_releases(http: &FmpHttpClient, params: NewsParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/press-releases-latest", &params).await
}

pub async fn stock_news(http: &FmpHttpClient, params: NewsParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/stock-latest", &params).await
}

pub async fn crypto_news(http: &FmpHttpClient, params: NewsParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/crypto-latest", &params).await
}

pub async fn forex_news(http: &FmpHttpClient, params: NewsParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/forex-latest", &params).await
}

pub async fn search_press_releases(http: &FmpHttpClient, params: NewsSearchParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/press-releases", &params).await
}

pub async fn search_stock_news(http: &FmpHttpClient, params: NewsSearchParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/stock", &params).await
}

pub async fn search_crypto_news(http: &FmpHttpClient, params: NewsSearchParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/crypto", &params).await
}

pub async fn search_forex_news(http: &FmpHttpClient, params: NewsSearchParams) -> FmpResult<Vec<NewsArticle>> {
  http.get_json("/news/forex", &params).await
}
