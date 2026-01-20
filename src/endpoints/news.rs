use crate::macros::define_api_trait;
use crate::types::news::{FmpArticle, NewsArticle, NewsParams, NewsSearchParams};

define_api_trait!(
  /// API endpoints for news.
  NewsApi,
  fmp_articles -> "/fmp-articles" -> NewsParams  -> Vec<FmpArticle>,
  general_news -> "/news/general-latest" -> NewsParams  -> Vec<NewsArticle>,
  press_releases -> "/news/press-releases-latest" -> NewsParams  -> Vec<NewsArticle>,
  stock_news -> "/news/stock-latest" -> NewsParams  -> Vec<NewsArticle>,
  crypto_news -> "/news/crypto-latest" -> NewsParams  -> Vec<NewsArticle>,
  forex_news -> "/news/forex-latest" -> NewsParams  -> Vec<NewsArticle>,
  search_press_releases -> "/news/press-releases" -> NewsSearchParams  -> Vec<NewsArticle>,
  search_stock_news -> "/news/stock" -> NewsSearchParams  -> Vec<NewsArticle>,
  search_crypto_news -> "/news/crypto" -> NewsSearchParams  -> Vec<NewsArticle>,
  search_forex_news -> "/news/forex" -> NewsSearchParams  -> Vec<NewsArticle>,
);
