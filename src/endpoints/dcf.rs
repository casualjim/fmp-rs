use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::dcf::{CustomDcfInput, CustomDcfOutput, DcfValuation};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SymbolParams {
  pub symbol: String,
}

pub async fn discounted_cash_flow(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<DcfValuation> {
  http.get_json("/discounted-cash-flow", &params).await
}

pub async fn levered_discounted_cash_flow(http: &FmpHttpClient, params: SymbolParams) -> FmpResult<Vec<DcfValuation>> {
  http.get_json("/levered-discounted-cash-flow", &params).await
}

pub async fn custom_levered_discounted_cash_flow(
  http: &FmpHttpClient,
  input: CustomDcfInput,
) -> FmpResult<CustomDcfOutput> {
  http.get_json("/custom-levered-discounted-cash-flow", &input).await
}

pub async fn custom_discounted_cash_flow(http: &FmpHttpClient, input: CustomDcfInput) -> FmpResult<CustomDcfOutput> {
  http.get_json("/custom-discounted-cash-flow", &input).await
}
