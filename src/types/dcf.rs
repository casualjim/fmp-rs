use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::FmpDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcfValuation {
  pub symbol: String,
  pub date: FmpDate,
  #[serde(rename = "Stock Price")]
  pub stock_price: f64,
  pub dcf: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CustomDcfInput {
  pub symbol: String,
  pub revenue_growth_pct: Option<f64>,
  pub ebitda_pct: Option<f64>,
  pub depreciation_and_amortization_pct: Option<f64>,
  pub cash_and_short_term_investments_pct: Option<f64>,
  pub receivables_pct: Option<f64>,
  pub inventories_pct: Option<f64>,
  pub payable_pct: Option<f64>,
  pub ebit_pct: Option<f64>,
  pub capital_expenditure_pct: Option<f64>,
  pub operating_cash_flow_pct: Option<f64>,
  pub selling_general_and_administrative_expenses_pct: Option<f64>,
  pub tax_rate: Option<f64>,
  pub long_term_growth_rate: Option<f64>,
  pub cost_of_debt: Option<f64>,
  pub cost_of_equity: Option<f64>,
  pub market_risk_premium: Option<f64>,
  pub beta: Option<f64>,
  pub risk_free_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDcfOutput {
  pub symbol: String,
  pub revenue: Option<f64>,
  pub revenue_percentage: Option<f64>,
  pub ebitda: Option<f64>,
  pub ebitda_percentage: Option<f64>,
  pub ebit: Option<f64>,
  pub ebit_percentage: Option<f64>,
  pub depreciation: Option<f64>,
  pub depreciation_percentage: Option<f64>,
  pub total_cash: Option<f64>,
  pub total_cash_percentage: Option<f64>,
  pub receivables: Option<f64>,
  pub receivables_percentage: Option<f64>,
  pub inventories: Option<f64>,
  pub inventories_percentage: Option<f64>,
  pub payable: Option<f64>,
  pub payable_percentage: Option<f64>,
  pub capital_expenditure: Option<f64>,
  pub capital_expenditure_percentage: Option<f64>,
  pub price: Option<f64>,
  pub beta: Option<f64>,
  pub diluted_shares_outstanding: Option<f64>,
  #[serde(rename = "costofDebt")]
  pub costof_debt: Option<f64>,
  pub tax_rate: Option<f64>,
  pub after_tax_cost_of_debt: Option<f64>,
  pub risk_free_rate: Option<f64>,
  pub market_risk_premium: Option<f64>,
  pub cost_of_equity: Option<f64>,
  pub total_debt: Option<f64>,
  pub total_equity: Option<f64>,
  pub total_capital: Option<f64>,
  pub debt_weighting: Option<f64>,
  pub equity_weighting: Option<f64>,
  pub wacc: Option<f64>,
  pub tax_rate_cash: Option<f64>,
  pub ebiat: Option<f64>,
  pub ufcf: Option<f64>,
  pub sum_pv_ufcf: Option<f64>,
  pub long_term_growth_rate: Option<f64>,
  pub terminal_value: Option<f64>,
  pub present_terminal_value: Option<f64>,
  pub enterprise_value: Option<f64>,
  pub net_debt: Option<f64>,
  pub equity_value: Option<f64>,
  pub equity_value_per_share: Option<f64>,
  pub free_cash_flow_t1: Option<f64>,
}
