use crate::macros::define_api_trait;
use crate::types::government_trading::{FinancialDisclosure, NameParams, PaginationParams, SymbolParams};

define_api_trait!(
  /// API endpoints for government_trading.
  GovernmentTradingApi,
  senate_latest -> "/senate-latest" -> PaginationParams  -> Vec<FinancialDisclosure>,
  house_latest -> "/house-latest" -> PaginationParams  -> Vec<FinancialDisclosure>,
  senate_trades -> "/senate-trades" -> SymbolParams  -> Vec<FinancialDisclosure>,
  senate_trades_by_name -> "/senate-trades-by-name" -> NameParams  -> Vec<FinancialDisclosure>,
  house_trades -> "/house-trades" -> SymbolParams  -> Vec<FinancialDisclosure>,
  house_trades_by_name -> "/house-trades-by-name" -> NameParams  -> Vec<FinancialDisclosure>,
);
