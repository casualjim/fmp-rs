use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::directory::{
  ActivelyTradingEntry, CikEntry, CikListParams, CompanySymbol, CountryEntry, EarningsTranscriptEntry, EtfEntry,
  ExchangeEntry, FinancialStatementSymbol, IndustryEntry, SectorEntry, SymbolChange, SymbolChangeParams,
};

pub async fn stock_list(http: &FmpHttpClient) -> FmpResult<Vec<CompanySymbol>> {
  http.get_json("/stock-list", &()).await
}

pub async fn financial_statement_symbol_list(http: &FmpHttpClient) -> FmpResult<Vec<FinancialStatementSymbol>> {
  http.get_json("/financial-statement-symbol-list", &()).await
}

pub async fn cik_list(http: &FmpHttpClient, params: CikListParams) -> FmpResult<Vec<CikEntry>> {
  http.get_json("/cik-list", &params).await
}

pub async fn symbol_change(http: &FmpHttpClient, params: SymbolChangeParams) -> FmpResult<Vec<SymbolChange>> {
  http.get_json("/symbol-change", &params).await
}

pub async fn etf_list(http: &FmpHttpClient) -> FmpResult<Vec<EtfEntry>> {
  http.get_json("/etf-list", &()).await
}

pub async fn actively_trading_list(http: &FmpHttpClient) -> FmpResult<Vec<ActivelyTradingEntry>> {
  http.get_json("/actively-trading-list", &()).await
}

pub async fn earnings_transcript_list(http: &FmpHttpClient) -> FmpResult<Vec<EarningsTranscriptEntry>> {
  http.get_json("/earnings-transcript-list", &()).await
}

pub async fn available_exchanges(http: &FmpHttpClient) -> FmpResult<Vec<ExchangeEntry>> {
  http.get_json("/available-exchanges", &()).await
}

pub async fn available_sectors(http: &FmpHttpClient) -> FmpResult<Vec<SectorEntry>> {
  http.get_json("/available-sectors", &()).await
}

pub async fn available_industries(http: &FmpHttpClient) -> FmpResult<Vec<IndustryEntry>> {
  http.get_json("/available-industries", &()).await
}

pub async fn available_countries(http: &FmpHttpClient) -> FmpResult<Vec<CountryEntry>> {
  http.get_json("/available-countries", &()).await
}
