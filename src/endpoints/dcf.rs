use crate::macros::define_api_trait;
use crate::types::dcf::{CustomDcfInput, CustomDcfOutput, DcfValuation};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SymbolParams {
  pub symbol: String,
}

define_api_trait!(
  /// API endpoints for discounted cash flow analysis.
  DcfApi,
  discounted_cash_flow -> "/discounted-cash-flow" -> SymbolParams  -> DcfValuation,
  levered_discounted_cash_flow -> "/levered-discounted-cash-flow" -> SymbolParams  -> Vec<DcfValuation>,
  custom_levered_discounted_cash_flow -> "/custom-levered-discounted-cash-flow" -> CustomDcfInput  -> CustomDcfOutput,
  custom_discounted_cash_flow -> "/custom-discounted-cash-flow" -> CustomDcfInput  -> CustomDcfOutput,
);
