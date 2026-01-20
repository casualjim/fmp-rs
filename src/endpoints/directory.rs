use crate::macros::define_api_trait;
use crate::types::directory::{
  ActivelyTradingEntry, CikEntry, CikListParams, CompanySymbol, CountryEntry, EarningsTranscriptEntry, EtfEntry,
  ExchangeEntry, FinancialStatementSymbol, IndustryEntry, SectorEntry, SymbolChange, SymbolChangeParams,
};

define_api_trait!(
  /// API endpoints for directory listings.
  DirectoryApi,
  stock_list -> "/stock-list" -> () -> Vec<CompanySymbol>,
  financial_statement_symbol_list -> "/financial-statement-symbol-list" -> () -> Vec<FinancialStatementSymbol>,
  cik_list -> "/cik-list" -> CikListParams  -> Vec<CikEntry>,
  symbol_change -> "/symbol-change" -> SymbolChangeParams  -> Vec<SymbolChange>,
  etf_list -> "/etf-list" -> () -> Vec<EtfEntry>,
  actively_trading_list -> "/actively-trading-list" -> () -> Vec<ActivelyTradingEntry>,
  earnings_transcript_list -> "/earnings-transcript-list" -> () -> Vec<EarningsTranscriptEntry>,
  available_exchanges -> "/available-exchanges" -> () -> Vec<ExchangeEntry>,
  available_sectors -> "/available-sectors" -> () -> Vec<SectorEntry>,
  available_industries -> "/available-industries" -> () -> Vec<IndustryEntry>,
  available_countries -> "/available-countries" -> () -> Vec<CountryEntry>,
);
