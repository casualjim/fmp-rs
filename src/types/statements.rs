use std::collections::HashMap;

use serde::Deserializer;
use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};

use crate::primitives::{FmpDate, FmpDateTime};

fn de_i32_string_or_number<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
  D: Deserializer<'de>,
{
  struct I32Visitor;

  impl<'de> serde::de::Visitor<'de> for I32Visitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("an integer or a string containing an integer")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v as i32)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v as i32)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      v.parse::<i32>().map_err(E::custom)
    }
  }

  deserializer.deserialize_any(I32Visitor)
}

/// Fiscal reporting period for a financial statement.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Period {
  /// First fiscal quarter (3 months ending ~March or company-specific).
  Q1,
  /// Second fiscal quarter (3 months ending ~June or company-specific).
  Q2,
  /// Third fiscal quarter (3 months ending ~September or company-specific).
  Q3,
  /// Fourth fiscal quarter (3 months ending ~December or company-specific).
  Q4,
  /// Full fiscal year (12 months; corresponds to 10-K annual report).
  FY,
}

/// Common metadata shared by all financial statement types.
///
/// Contains the period identification and SEC filing provenance for a
/// financial statement. Flattened into each concrete statement struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseStatement {
  /// End date of the reporting period covered by this statement.
  pub date: FmpDate,
  /// Ticker symbol of the reporting company.
  pub symbol: String,
  /// ISO 4217 currency code in which amounts are reported (e.g., "USD").
  pub reported_currency: String,
  /// SEC Central Index Key of the reporting company; absent from growth statements.
  #[serde(default)]
  pub cik: Option<String>,
  /// Date the document was filed with the SEC; absent from growth statements.
  #[serde(default)]
  pub filing_date: Option<FmpDate>,
  /// Date and time the SEC EDGAR system accepted the filing; absent from growth statements.
  #[serde(default)]
  pub accepted_date: Option<FmpDateTime>,
  /// Fiscal year number (e.g., 2024). Deserialised from either an integer or string.
  #[serde(deserialize_with = "de_i32_string_or_number")]
  pub fiscal_year: i32,
  /// Fiscal period (Q1, Q2, Q3, Q4, or FY for the full fiscal year).
  pub period: Period,
}

/// Standardised income statement (profit & loss) for one reporting period.
///
/// Sourced from SEC 10-K (annual) and 10-Q (quarterly) XBRL filings, normalised
/// by FMP for comparability across companies and reporting standards (US GAAP,
/// IFRS). All monetary amounts are in [`BaseStatement::reported_currency`].
#[allow(clippy::too_many_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeStatement {
  #[serde(flatten)]
  pub base: BaseStatement,
  pub revenue: f64,
  pub cost_of_revenue: f64,
  pub gross_profit: f64,
  pub research_and_development_expenses: f64,
  pub general_and_administrative_expenses: f64,
  pub selling_and_marketing_expenses: f64,
  pub selling_general_and_administrative_expenses: f64,
  pub other_expenses: f64,
  pub operating_expenses: f64,
  pub cost_and_expenses: f64,
  pub net_interest_income: f64,
  pub interest_income: f64,
  pub interest_expense: f64,
  pub depreciation_and_amortization: f64,
  pub ebitda: f64,
  pub ebit: f64,
  pub non_operating_income_excluding_interest: f64,
  pub operating_income: f64,
  pub total_other_income_expenses_net: f64,
  pub income_before_tax: f64,
  pub income_tax_expense: f64,
  pub net_income_from_continuing_operations: f64,
  pub net_income_from_discontinued_operations: f64,
  pub other_adjustments_to_net_income: f64,
  pub net_income: f64,
  pub net_income_deductions: f64,
  pub bottom_line_net_income: f64,
  pub eps: f64,
  pub eps_diluted: f64,
  pub weighted_average_shs_out: f64,
  pub weighted_average_shs_out_dil: f64,
}

/// Standardised balance sheet (statement of financial position) for one reporting period.
///
/// All assets, liabilities, and equity amounts are in [`BaseStatement::reported_currency`].
/// Sourced from SEC XBRL filings and normalised by FMP for cross-company comparability.
#[allow(clippy::too_many_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheetStatement {
  #[serde(flatten)]
  pub base: BaseStatement,
  pub cash_and_cash_equivalents: f64,
  pub short_term_investments: f64,
  pub cash_and_short_term_investments: f64,
  pub net_receivables: f64,
  pub accounts_receivables: f64,
  pub other_receivables: f64,
  pub inventory: f64,
  pub prepaids: f64,
  pub other_current_assets: f64,
  pub total_current_assets: f64,
  pub property_plant_equipment_net: f64,
  pub goodwill: f64,
  pub intangible_assets: f64,
  pub goodwill_and_intangible_assets: f64,
  pub long_term_investments: f64,
  pub tax_assets: f64,
  pub other_non_current_assets: f64,
  pub total_non_current_assets: f64,
  pub other_assets: f64,
  pub total_assets: f64,
  pub total_payables: f64,
  pub account_payables: f64,
  pub other_payables: f64,
  pub accrued_expenses: f64,
  pub short_term_debt: f64,
  pub capital_lease_obligations_current: f64,
  pub tax_payables: f64,
  pub deferred_revenue: f64,
  pub other_current_liabilities: f64,
  pub total_current_liabilities: f64,
  pub long_term_debt: f64,
  pub deferred_revenue_non_current: f64,
  pub deferred_tax_liabilities_non_current: f64,
  pub other_non_current_liabilities: f64,
  pub total_non_current_liabilities: f64,
  pub other_liabilities: f64,
  pub capital_lease_obligations: f64,
  pub total_liabilities: f64,
  pub treasury_stock: f64,
  pub preferred_stock: f64,
  pub common_stock: f64,
  pub retained_earnings: f64,
  pub additional_paid_in_capital: f64,
  pub accumulated_other_comprehensive_income_loss: f64,
  pub other_total_stockholders_equity: f64,
  pub total_stockholders_equity: f64,
  pub total_equity: f64,
  pub minority_interest: f64,
  pub total_liabilities_and_total_equity: f64,
  pub total_investments: f64,
  pub total_debt: f64,
  pub net_debt: f64,
}

/// Standardised cash flow statement for one reporting period.
///
/// Cash flows are split into operating, investing, and financing activities.
/// All amounts are in [`BaseStatement::reported_currency`].
#[allow(clippy::too_many_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashFlowStatement {
  #[serde(flatten)]
  pub base: BaseStatement,
  pub net_income: f64,
  pub depreciation_and_amortization: f64,
  pub deferred_income_tax: f64,
  pub stock_based_compensation: f64,
  pub change_in_working_capital: f64,
  pub accounts_receivables: f64,
  pub inventory: f64,
  pub accounts_payables: f64,
  pub other_working_capital: f64,
  pub other_non_cash_items: f64,
  pub net_cash_provided_by_operating_activities: f64,
  pub investments_in_property_plant_and_equipment: f64,
  pub acquisitions_net: f64,
  pub purchases_of_investments: f64,
  pub sales_maturities_of_investments: f64,
  pub other_investing_activities: f64,
  pub net_cash_provided_by_investing_activities: f64,
  pub net_debt_issuance: f64,
  pub long_term_net_debt_issuance: f64,
  pub short_term_net_debt_issuance: f64,
  pub net_stock_issuance: f64,
  pub net_common_stock_issuance: f64,
  pub common_stock_issuance: f64,
  pub common_stock_repurchased: f64,
  pub net_preferred_stock_issuance: f64,
  pub net_dividends_paid: f64,
  pub common_dividends_paid: f64,
  pub preferred_dividends_paid: f64,
  pub other_financing_activities: f64,
  pub net_cash_provided_by_financing_activities: f64,
  pub effect_of_forex_changes_on_cash: f64,
  pub net_change_in_cash: f64,
  pub cash_at_end_of_period: f64,
  pub cash_at_beginning_of_period: f64,
  pub operating_cash_flow: f64,
  pub capital_expenditure: f64,
  pub free_cash_flow: f64,
  pub income_taxes_paid: f64,
  pub interest_paid: f64,
}

// Growth variants
#[allow(clippy::too_many_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeStatementGrowth {
  #[serde(flatten)]
  pub base: BaseStatement,
  pub growth_revenue: f64,
  pub growth_cost_of_revenue: f64,
  pub growth_gross_profit: f64,
  pub growth_gross_profit_ratio: f64,
  pub growth_research_and_development_expenses: f64,
  pub growth_general_and_administrative_expenses: f64,
  pub growth_selling_and_marketing_expenses: f64,
  pub growth_other_expenses: f64,
  pub growth_operating_expenses: f64,
  pub growth_cost_and_expenses: f64,
  pub growth_interest_income: f64,
  pub growth_interest_expense: f64,
  pub growth_depreciation_and_amortization: f64,
  #[serde(default)]
  pub growth_ebitda: Option<f64>,
  pub growth_operating_income: f64,
  pub growth_income_before_tax: f64,
  pub growth_income_tax_expense: f64,
  pub growth_net_income: f64,
  #[serde(default)]
  pub growth_eps: Option<f64>,
  #[serde(default)]
  pub growth_eps_diluted: Option<f64>,
  pub growth_weighted_average_shs_out: f64,
  pub growth_weighted_average_shs_out_dil: f64,
  #[serde(default)]
  pub growth_ebit: Option<f64>,
  pub growth_non_operating_income_excluding_interest: f64,
  pub growth_net_interest_income: f64,
  pub growth_total_other_income_expenses_net: f64,
  pub growth_net_income_from_continuing_operations: f64,
  pub growth_other_adjustments_to_net_income: f64,
  pub growth_net_income_deductions: f64,
}

#[allow(clippy::too_many_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheetStatementGrowth {
  #[serde(flatten)]
  pub base: BaseStatement,
  pub growth_cash_and_cash_equivalents: f64,
  pub growth_short_term_investments: f64,
  pub growth_cash_and_short_term_investments: f64,
  pub growth_net_receivables: f64,
  pub growth_inventory: f64,
  pub growth_other_current_assets: f64,
  pub growth_total_current_assets: f64,
  pub growth_property_plant_equipment_net: f64,
  pub growth_goodwill: f64,
  pub growth_intangible_assets: f64,
  pub growth_goodwill_and_intangible_assets: f64,
  pub growth_long_term_investments: f64,
  pub growth_tax_assets: f64,
  pub growth_other_non_current_assets: f64,
  pub growth_total_non_current_assets: f64,
  pub growth_other_assets: f64,
  pub growth_total_assets: f64,
  pub growth_account_payables: f64,
  pub growth_short_term_debt: f64,
  pub growth_tax_payables: f64,
  pub growth_deferred_revenue: f64,
  pub growth_other_current_liabilities: f64,
  pub growth_total_current_liabilities: f64,
  pub growth_long_term_debt: f64,
  pub growth_deferred_revenue_non_current: f64,
  pub growth_deferred_tax_liabilities_non_current: f64,
  pub growth_other_non_current_liabilities: f64,
  pub growth_total_non_current_liabilities: f64,
  pub growth_other_liabilities: f64,
  pub growth_total_liabilities: f64,
  pub growth_preferred_stock: f64,
  pub growth_common_stock: f64,
  pub growth_retained_earnings: f64,
  pub growth_accumulated_other_comprehensive_income_loss: f64,
  pub growth_othertotal_stockholders_equity: f64,
  pub growth_total_stockholders_equity: f64,
  pub growth_minority_interest: f64,
  pub growth_total_equity: f64,
  pub growth_total_liabilities_and_stockholders_equity: f64,
  pub growth_total_investments: f64,
  pub growth_total_debt: f64,
  pub growth_net_debt: f64,
  pub growth_accounts_receivables: f64,
  pub growth_other_receivables: f64,
  pub growth_prepaids: f64,
  pub growth_total_payables: f64,
  pub growth_other_payables: f64,
  pub growth_accrued_expenses: f64,
  pub growth_capital_lease_obligations_current: f64,
  pub growth_additional_paid_in_capital: f64,
  pub growth_treasury_stock: f64,
}

#[allow(clippy::too_many_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashFlowStatementGrowth {
  #[serde(flatten)]
  pub base: BaseStatement,
  pub growth_net_income: f64,
  pub growth_depreciation_and_amortization: f64,
  pub growth_deferred_income_tax: f64,
  pub growth_stock_based_compensation: f64,
  pub growth_change_in_working_capital: f64,
  pub growth_accounts_receivables: f64,
  pub growth_inventory: f64,
  pub growth_accounts_payables: f64,
  pub growth_other_working_capital: f64,
  pub growth_other_non_cash_items: f64,
  pub growth_net_cash_provided_by_operating_activites: f64,
  pub growth_investments_in_property_plant_and_equipment: f64,
  pub growth_acquisitions_net: f64,
  pub growth_purchases_of_investments: f64,
  pub growth_sales_maturities_of_investments: f64,
  pub growth_other_investing_activites: f64,
  pub growth_net_cash_used_for_investing_activites: f64,
  pub growth_debt_repayment: f64,
  pub growth_common_stock_issued: f64,
  pub growth_common_stock_repurchased: f64,
  pub growth_dividends_paid: f64,
  pub growth_other_financing_activites: f64,
  pub growth_net_cash_used_provided_by_financing_activities: f64,
  pub growth_effect_of_forex_changes_on_cash: f64,
  pub growth_net_change_in_cash: f64,
  pub growth_cash_at_end_of_period: f64,
  pub growth_cash_at_beginning_of_period: f64,
  pub growth_operating_cash_flow: f64,
  pub growth_capital_expenditure: f64,
  pub growth_free_cash_flow: f64,
  pub growth_net_debt_issuance: f64,
  pub growth_long_term_net_debt_issuance: f64,
  pub growth_short_term_net_debt_issuance: f64,
  pub growth_net_stock_issuance: f64,
  pub growth_preferred_dividends_paid: f64,
  pub growth_income_taxes_paid: f64,
  pub growth_interest_paid: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialStatementGrowth {
  #[serde(flatten)]
  pub base: BaseStatement,
  pub revenue_growth: f64,
  pub gross_profit_growth: f64,
  pub ebitgrowth: f64,
  pub operating_income_growth: f64,
  pub net_income_growth: f64,
  pub epsgrowth: f64,
  pub epsdiluted_growth: f64,
  pub weighted_average_shares_growth: f64,
  pub weighted_average_shares_diluted_growth: f64,
  pub dividends_per_share_growth: f64,
  pub operating_cash_flow_growth: f64,
  pub receivables_growth: f64,
  pub inventory_growth: f64,
  pub asset_growth: f64,
  pub book_valueper_share_growth: f64,
  pub debt_growth: f64,
  pub rdexpense_growth: f64,
  pub sgaexpenses_growth: f64,
  pub free_cash_flow_growth: f64,
  pub ten_y_revenue_growth_per_share: f64,
  pub five_y_revenue_growth_per_share: f64,
  pub three_y_revenue_growth_per_share: f64,
  #[serde(default)]
  pub ten_y_operating_cf_growth_per_share: Option<f64>,
  #[serde(default)]
  pub five_y_operating_cf_growth_per_share: Option<f64>,
  #[serde(default)]
  pub three_y_operating_cf_growth_per_share: Option<f64>,
  pub ten_y_net_income_growth_per_share: f64,
  pub five_y_net_income_growth_per_share: f64,
  pub three_y_net_income_growth_per_share: f64,
  pub ten_y_shareholders_equity_growth_per_share: f64,
  pub five_y_shareholders_equity_growth_per_share: f64,
  pub three_y_shareholders_equity_growth_per_share: f64,
  #[serde(default)]
  pub ten_y_dividend_per_share_growth_per_share: Option<f64>,
  #[serde(default)]
  pub five_y_dividend_per_share_growth_per_share: Option<f64>,
  #[serde(default)]
  pub three_y_dividend_per_share_growth_per_share: Option<f64>,
  pub ebitda_growth: Option<f64>,
  pub growth_capital_expenditure: Option<f64>,
  pub ten_y_bottom_line_net_income_growth_per_share: Option<f64>,
  pub five_y_bottom_line_net_income_growth_per_share: Option<f64>,
  pub three_y_bottom_line_net_income_growth_per_share: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialReportDate {
  pub symbol: String,
  #[serde(deserialize_with = "de_i32_string_or_number")]
  pub fiscal_year: i32,
  pub period: Period,
  pub link_xlsx: String,
  pub link_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestFinancialStatement {
  pub symbol: String,
  #[serde(deserialize_with = "de_i32_string_or_number")]
  pub calendar_year: i32,
  pub period: Period,
  pub date: FmpDate,
  pub date_added: FmpDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialReportItem {
  #[serde(flatten)]
  pub items: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialReport10K {
  #[serde(default)]
  pub symbol: Option<String>,
  #[serde(default)]
  pub period: Option<String>,
  #[serde(default)]
  pub year: Option<String>,
  #[serde(flatten)]
  pub rest: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueProductSegmentation {
  pub symbol: String,
  #[serde(deserialize_with = "de_i32_string_or_number")]
  pub fiscal_year: i32,
  pub period: String,
  pub reported_currency: Option<String>,
  pub date: FmpDate,
  pub data: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueGeographicSegmentation {
  pub symbol: String,
  #[serde(deserialize_with = "de_i32_string_or_number")]
  pub fiscal_year: i32,
  pub period: String,
  pub reported_currency: Option<String>,
  pub date: FmpDate,
  pub data: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsReportedStatement {
  pub symbol: String,
  #[serde(deserialize_with = "de_i32_string_or_number")]
  pub fiscal_year: i32,
  pub period: String,
  pub reported_currency: Option<String>,
  pub date: FmpDate,
  pub data: HashMap<String, serde_json::Value>,
}

pub type AsReportedIncomeStatement = AsReportedStatement;
pub type AsReportedBalanceSheet = AsReportedStatement;
pub type AsReportedCashFlowStatement = AsReportedStatement;
pub type AsReportedFinancialStatement = AsReportedStatement;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyMetrics {
  pub symbol: String,
  pub date: FmpDate,
  pub fiscal_year: String,
  pub period: String,
  pub reported_currency: String,
  pub market_cap: f64,
  pub enterprise_value: f64,
  pub ev_to_sales: f64,
  pub ev_to_operating_cash_flow: f64,
  pub ev_to_free_cash_flow: f64,
  #[serde(rename = "evToEBITDA")]
  pub ev_to_ebitda: f64,
  #[serde(rename = "netDebtToEBITDA")]
  pub net_debt_to_ebitda: f64,
  pub current_ratio: f64,
  pub income_quality: f64,
  pub graham_number: f64,
  pub graham_net_net: f64,
  pub tax_burden: f64,
  pub interest_burden: f64,
  pub working_capital: f64,
  pub invested_capital: f64,
  pub return_on_assets: f64,
  pub operating_return_on_assets: f64,
  pub return_on_tangible_assets: f64,
  pub return_on_equity: f64,
  pub return_on_invested_capital: f64,
  pub return_on_capital_employed: f64,
  pub earnings_yield: f64,
  pub free_cash_flow_yield: f64,
  pub capex_to_operating_cash_flow: f64,
  pub capex_to_depreciation: f64,
  pub capex_to_revenue: f64,
  pub sales_general_and_administrative_to_revenue: f64,
  pub research_and_developement_to_revenue: f64,
  pub stock_based_compensation_to_revenue: f64,
  pub intangibles_to_total_assets: f64,
  pub average_receivables: f64,
  pub average_payables: f64,
  pub average_inventory: f64,
  pub days_of_sales_outstanding: f64,
  pub days_of_payables_outstanding: f64,
  pub days_of_inventory_outstanding: f64,
  pub operating_cycle: f64,
  pub cash_conversion_cycle: f64,
  pub free_cash_flow_to_equity: f64,
  pub free_cash_flow_to_firm: f64,
  pub tangible_asset_value: f64,
  pub net_current_asset_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratios {
  pub symbol: String,
  pub date: FmpDate,
  pub fiscal_year: String,
  pub period: String,
  pub reported_currency: String,
  pub gross_profit_margin: f64,
  pub ebit_margin: f64,
  pub ebitda_margin: f64,
  pub operating_profit_margin: f64,
  pub pretax_profit_margin: f64,
  pub continuous_operations_profit_margin: f64,
  pub net_profit_margin: f64,
  pub bottom_line_profit_margin: f64,
  pub receivables_turnover: f64,
  pub payables_turnover: f64,
  pub inventory_turnover: f64,
  pub fixed_asset_turnover: f64,
  pub asset_turnover: f64,
  pub current_ratio: f64,
  pub quick_ratio: f64,
  pub solvency_ratio: f64,
  pub cash_ratio: f64,
  pub price_to_earnings_ratio: f64,
  pub price_to_earnings_growth_ratio: f64,
  pub forward_price_to_earnings_growth_ratio: f64,
  pub price_to_book_ratio: f64,
  pub price_to_sales_ratio: f64,
  pub price_to_free_cash_flow_ratio: f64,
  pub price_to_operating_cash_flow_ratio: f64,
  pub debt_to_assets_ratio: f64,
  pub debt_to_equity_ratio: f64,
  pub debt_to_capital_ratio: f64,
  pub long_term_debt_to_capital_ratio: f64,
  pub financial_leverage_ratio: f64,
  pub working_capital_turnover_ratio: f64,
  pub operating_cash_flow_ratio: f64,
  pub operating_cash_flow_sales_ratio: f64,
  pub free_cash_flow_operating_cash_flow_ratio: f64,
  pub debt_service_coverage_ratio: f64,
  pub interest_coverage_ratio: f64,
  pub short_term_operating_cash_flow_coverage_ratio: f64,
  pub operating_cash_flow_coverage_ratio: f64,
  pub capital_expenditure_coverage_ratio: f64,
  pub dividend_paid_and_capex_coverage_ratio: f64,
  pub dividend_payout_ratio: f64,
  pub dividend_yield: f64,
  pub dividend_yield_percentage: f64,
  pub revenue_per_share: f64,
  pub net_income_per_share: f64,
  pub interest_debt_per_share: f64,
  pub cash_per_share: f64,
  pub book_value_per_share: f64,
  pub tangible_book_value_per_share: f64,
  pub shareholders_equity_per_share: f64,
  pub operating_cash_flow_per_share: f64,
  pub capex_per_share: f64,
  pub free_cash_flow_per_share: f64,
  #[serde(default)]
  pub net_income_per_ebt: Option<f64>,
  pub ebt_per_ebit: f64,
  pub price_to_fair_value: f64,
  pub debt_to_market_cap: f64,
  pub effective_tax_rate: f64,
  pub enterprise_value_multiple: f64,
}

// The FMP API uses all-caps "TTM" and "EBITDA" in JSON field names, which
// camelCase rename_all cannot produce. Every TTM field needs an explicit rename.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyMetricsTtm {
  pub symbol: String,
  pub market_cap: f64,
  #[serde(rename = "enterpriseValueTTM")]
  pub enterprise_value_ttm: f64,
  #[serde(rename = "evToSalesTTM")]
  pub ev_to_sales_ttm: f64,
  #[serde(rename = "evToOperatingCashFlowTTM")]
  pub ev_to_operating_cash_flow_ttm: f64,
  #[serde(rename = "evToFreeCashFlowTTM")]
  pub ev_to_free_cash_flow_ttm: f64,
  #[serde(rename = "evToEBITDATTM")]
  pub ev_to_ebitda_ttm: f64,
  #[serde(rename = "netDebtToEBITDATTM")]
  pub net_debt_to_ebitda_ttm: f64,
  #[serde(rename = "currentRatioTTM")]
  pub current_ratio_ttm: f64,
  #[serde(rename = "incomeQualityTTM")]
  pub income_quality_ttm: f64,
  #[serde(rename = "grahamNumberTTM")]
  pub graham_number_ttm: f64,
  #[serde(rename = "grahamNetNetTTM")]
  pub graham_net_net_ttm: f64,
  #[serde(rename = "taxBurdenTTM")]
  pub tax_burden_ttm: f64,
  #[serde(rename = "interestBurdenTTM")]
  pub interest_burden_ttm: f64,
  #[serde(rename = "workingCapitalTTM")]
  pub working_capital_ttm: f64,
  #[serde(rename = "investedCapitalTTM")]
  pub invested_capital_ttm: f64,
  #[serde(rename = "returnOnAssetsTTM")]
  pub return_on_assets_ttm: f64,
  #[serde(rename = "operatingReturnOnAssetsTTM")]
  pub operating_return_on_assets_ttm: f64,
  #[serde(rename = "returnOnTangibleAssetsTTM")]
  pub return_on_tangible_assets_ttm: f64,
  #[serde(rename = "returnOnEquityTTM")]
  pub return_on_equity_ttm: f64,
  #[serde(rename = "returnOnInvestedCapitalTTM")]
  pub return_on_invested_capital_ttm: f64,
  #[serde(rename = "returnOnCapitalEmployedTTM")]
  pub return_on_capital_employed_ttm: f64,
  #[serde(rename = "earningsYieldTTM")]
  pub earnings_yield_ttm: f64,
  #[serde(rename = "freeCashFlowYieldTTM")]
  pub free_cash_flow_yield_ttm: f64,
  #[serde(rename = "capexToOperatingCashFlowTTM")]
  pub capex_to_operating_cash_flow_ttm: f64,
  #[serde(rename = "capexToDepreciationTTM")]
  pub capex_to_depreciation_ttm: f64,
  #[serde(rename = "capexToRevenueTTM")]
  pub capex_to_revenue_ttm: f64,
  #[serde(rename = "salesGeneralAndAdministrativeToRevenueTTM")]
  pub sales_general_and_administrative_to_revenue_ttm: f64,
  #[serde(rename = "researchAndDevelopementToRevenueTTM")]
  pub research_and_developement_to_revenue_ttm: f64,
  #[serde(rename = "stockBasedCompensationToRevenueTTM")]
  pub stock_based_compensation_to_revenue_ttm: f64,
  #[serde(rename = "intangiblesToTotalAssetsTTM")]
  pub intangibles_to_total_assets_ttm: f64,
  #[serde(rename = "averageReceivablesTTM")]
  pub average_receivables_ttm: f64,
  #[serde(rename = "averagePayablesTTM")]
  pub average_payables_ttm: f64,
  #[serde(rename = "averageInventoryTTM")]
  pub average_inventory_ttm: f64,
  #[serde(rename = "daysOfSalesOutstandingTTM")]
  pub days_of_sales_outstanding_ttm: f64,
  #[serde(rename = "daysOfPayablesOutstandingTTM")]
  pub days_of_payables_outstanding_ttm: f64,
  #[serde(rename = "daysOfInventoryOutstandingTTM")]
  pub days_of_inventory_outstanding_ttm: f64,
  #[serde(rename = "operatingCycleTTM")]
  pub operating_cycle_ttm: f64,
  #[serde(rename = "cashConversionCycleTTM")]
  pub cash_conversion_cycle_ttm: f64,
  #[serde(rename = "freeCashFlowToEquityTTM")]
  pub free_cash_flow_to_equity_ttm: f64,
  #[serde(rename = "freeCashFlowToFirmTTM")]
  pub free_cash_flow_to_firm_ttm: f64,
  #[serde(rename = "tangibleAssetValueTTM")]
  pub tangible_asset_value_ttm: f64,
  #[serde(rename = "netCurrentAssetValueTTM")]
  pub net_current_asset_value_ttm: f64,
}

// FMP uses all-caps "TTM" in JSON keys; camelCase rename_all produces "Ttm".
// Every field needs an explicit rename.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialRatiosTtm {
  pub symbol: String,
  #[serde(default)]
  pub date: Option<FmpDate>,
  #[serde(default, rename = "fiscalYear")]
  pub fiscal_year: Option<String>,
  #[serde(default)]
  pub period: Option<String>,
  #[serde(default, rename = "reportedCurrency")]
  pub reported_currency: Option<String>,
  #[serde(rename = "grossProfitMarginTTM")]
  pub gross_profit_margin_ttm: f64,
  #[serde(rename = "ebitMarginTTM")]
  pub ebit_margin_ttm: f64,
  #[serde(rename = "ebitdaMarginTTM")]
  pub ebitda_margin_ttm: f64,
  #[serde(rename = "operatingProfitMarginTTM")]
  pub operating_profit_margin_ttm: f64,
  #[serde(rename = "pretaxProfitMarginTTM")]
  pub pretax_profit_margin_ttm: f64,
  #[serde(rename = "continuousOperationsProfitMarginTTM")]
  pub continuous_operations_profit_margin_ttm: f64,
  #[serde(rename = "netProfitMarginTTM")]
  pub net_profit_margin_ttm: f64,
  #[serde(rename = "bottomLineProfitMarginTTM")]
  pub bottom_line_profit_margin_ttm: f64,
  #[serde(rename = "receivablesTurnoverTTM")]
  pub receivables_turnover_ttm: f64,
  #[serde(rename = "payablesTurnoverTTM")]
  pub payables_turnover_ttm: f64,
  #[serde(rename = "inventoryTurnoverTTM")]
  pub inventory_turnover_ttm: f64,
  #[serde(rename = "fixedAssetTurnoverTTM")]
  pub fixed_asset_turnover_ttm: f64,
  #[serde(rename = "assetTurnoverTTM")]
  pub asset_turnover_ttm: f64,
  #[serde(rename = "currentRatioTTM")]
  pub current_ratio_ttm: f64,
  #[serde(rename = "quickRatioTTM")]
  pub quick_ratio_ttm: f64,
  #[serde(rename = "solvencyRatioTTM")]
  pub solvency_ratio_ttm: f64,
  #[serde(rename = "cashRatioTTM")]
  pub cash_ratio_ttm: f64,
  #[serde(rename = "priceToEarningsRatioTTM")]
  pub price_to_earnings_ratio_ttm: f64,
  #[serde(rename = "priceToEarningsGrowthRatioTTM")]
  pub price_to_earnings_growth_ratio_ttm: f64,
  #[serde(rename = "forwardPriceToEarningsGrowthRatioTTM")]
  pub forward_price_to_earnings_growth_ratio_ttm: f64,
  #[serde(rename = "priceToBookRatioTTM")]
  pub price_to_book_ratio_ttm: f64,
  #[serde(rename = "priceToSalesRatioTTM")]
  pub price_to_sales_ratio_ttm: f64,
  #[serde(rename = "priceToFreeCashFlowRatioTTM")]
  pub price_to_free_cash_flow_ratio_ttm: f64,
  #[serde(rename = "priceToOperatingCashFlowRatioTTM")]
  pub price_to_operating_cash_flow_ratio_ttm: f64,
  #[serde(rename = "debtToAssetsRatioTTM")]
  pub debt_to_assets_ratio_ttm: f64,
  #[serde(rename = "debtToEquityRatioTTM")]
  pub debt_to_equity_ratio_ttm: f64,
  #[serde(rename = "debtToCapitalRatioTTM")]
  pub debt_to_capital_ratio_ttm: f64,
  #[serde(rename = "longTermDebtToCapitalRatioTTM")]
  pub long_term_debt_to_capital_ratio_ttm: f64,
  #[serde(rename = "financialLeverageRatioTTM")]
  pub financial_leverage_ratio_ttm: f64,
  #[serde(rename = "workingCapitalTurnoverRatioTTM")]
  pub working_capital_turnover_ratio_ttm: f64,
  #[serde(rename = "operatingCashFlowRatioTTM")]
  pub operating_cash_flow_ratio_ttm: f64,
  #[serde(rename = "operatingCashFlowSalesRatioTTM")]
  pub operating_cash_flow_sales_ratio_ttm: f64,
  #[serde(rename = "freeCashFlowOperatingCashFlowRatioTTM")]
  pub free_cash_flow_operating_cash_flow_ratio_ttm: f64,
  #[serde(rename = "debtServiceCoverageRatioTTM")]
  pub debt_service_coverage_ratio_ttm: f64,
  #[serde(rename = "interestCoverageRatioTTM")]
  pub interest_coverage_ratio_ttm: f64,
  #[serde(rename = "shortTermOperatingCashFlowCoverageRatioTTM")]
  pub short_term_operating_cash_flow_coverage_ratio_ttm: f64,
  #[serde(rename = "operatingCashFlowCoverageRatioTTM")]
  pub operating_cash_flow_coverage_ratio_ttm: f64,
  #[serde(rename = "capitalExpenditureCoverageRatioTTM")]
  pub capital_expenditure_coverage_ratio_ttm: f64,
  #[serde(rename = "dividendPaidAndCapexCoverageRatioTTM")]
  pub dividend_paid_and_capex_coverage_ratio_ttm: f64,
  #[serde(rename = "dividendPayoutRatioTTM")]
  pub dividend_payout_ratio_ttm: f64,
  #[serde(rename = "dividendYieldTTM")]
  pub dividend_yield_ttm: f64,
  #[serde(rename = "enterpriseValueTTM")]
  pub enterprise_value_ttm: f64,
  #[serde(rename = "revenuePerShareTTM")]
  pub revenue_per_share_ttm: f64,
  #[serde(rename = "netIncomePerShareTTM")]
  pub net_income_per_share_ttm: f64,
  #[serde(rename = "interestDebtPerShareTTM")]
  pub interest_debt_per_share_ttm: f64,
  #[serde(rename = "cashPerShareTTM")]
  pub cash_per_share_ttm: f64,
  #[serde(rename = "bookValuePerShareTTM")]
  pub book_value_per_share_ttm: f64,
  #[serde(rename = "tangibleBookValuePerShareTTM")]
  pub tangible_book_value_per_share_ttm: f64,
  #[serde(rename = "shareholdersEquityPerShareTTM")]
  pub shareholders_equity_per_share_ttm: f64,
  #[serde(rename = "operatingCashFlowPerShareTTM")]
  pub operating_cash_flow_per_share_ttm: f64,
  #[serde(rename = "capexPerShareTTM")]
  pub capex_per_share_ttm: f64,
  #[serde(rename = "freeCashFlowPerShareTTM")]
  pub free_cash_flow_per_share_ttm: f64,
  #[serde(rename = "netIncomePerEBTTTM")]
  pub net_income_per_ebt_tttm: f64,
  #[serde(rename = "ebtPerEbitTTM")]
  pub ebt_per_ebit_ttm: f64,
  #[serde(rename = "priceToFairValueTTM")]
  pub price_to_fair_value_ttm: f64,
  #[serde(rename = "debtToMarketCapTTM")]
  pub debt_to_market_cap_ttm: f64,
  #[serde(rename = "effectiveTaxRateTTM")]
  pub effective_tax_rate_ttm: f64,
  #[serde(rename = "enterpriseValueMultipleTTM")]
  pub enterprise_value_multiple_ttm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialScores {
  pub symbol: String,
  pub reported_currency: String,
  pub altman_z_score: f64,
  pub piotroski_score: f64,
  pub working_capital: f64,
  pub total_assets: f64,
  pub retained_earnings: f64,
  pub ebit: f64,
  pub market_cap: f64,
  pub total_liabilities: f64,
  pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerEarnings {
  pub symbol: String,
  pub reported_currency: String,
  pub fiscal_year: String,
  pub period: String,
  pub date: FmpDate,
  #[serde(default)]
  pub average_ppe: Option<f64>,
  pub maintenance_capex: f64,
  pub owners_earnings: f64,
  pub growth_capex: f64,
  pub owners_earnings_per_share: f64,
}

use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct StatementPaginationParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct StatementCommonParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
  #[builder(default, setter(strip_option))]
  pub period: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct StatementLimitParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FinancialReportParams {
  pub symbol: String,
  pub year: i32,
  pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SegmentationParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub period: Option<String>,
  #[builder(default, setter(strip_option))]
  pub structure: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolParam {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct FinancialScoresParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[cfg(test)]
mod tests {
  use super::{
    AsReportedBalanceSheet, AsReportedCashFlowStatement, AsReportedIncomeStatement, BalanceSheetStatement,
    BalanceSheetStatementGrowth, CashFlowStatement, CashFlowStatementGrowth, FinancialRatiosTtm, FinancialReport10K,
    FinancialReportDate, FinancialScores, FinancialStatementGrowth, IncomeStatement, IncomeStatementGrowth, KeyMetrics,
    KeyMetricsTtm, LatestFinancialStatement, OwnerEarnings, Ratios, RevenueGeographicSegmentation,
    RevenueProductSegmentation,
  };

  #[test]
  fn income_statement_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/income_statement.json").unwrap();
    let _: Vec<IncomeStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn balance_sheet_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/balance_sheet.json").unwrap();
    let _: Vec<BalanceSheetStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn cash_flow_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/cash_flow.json").unwrap();
    let _: Vec<CashFlowStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn latest_financial_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/latest_financial.json").unwrap();
    let _: Vec<LatestFinancialStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn income_statement_ttm_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/income_statement_ttm.json").unwrap();
    let _: Vec<IncomeStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn balance_sheet_ttm_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/balance_sheet_ttm.json").unwrap();
    let _: Vec<BalanceSheetStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn cash_flow_ttm_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/cash_flow_ttm.json").unwrap();
    let _: Vec<CashFlowStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn income_growth_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/income_growth.json").unwrap();
    let _: Vec<IncomeStatementGrowth> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn balance_sheet_growth_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/balance_sheet_growth.json").unwrap();
    let _: Vec<BalanceSheetStatementGrowth> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn cash_flow_growth_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/cash_flow_growth.json").unwrap();
    let _: Vec<CashFlowStatementGrowth> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn financial_growth_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/financial_growth.json").unwrap();
    let _: Vec<FinancialStatementGrowth> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn financial_report_dates_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/financial_report_dates.json").unwrap();
    let _: Vec<FinancialReportDate> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn financial_report_10k_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/financial_report_10k.json").unwrap();
    let _: FinancialReport10K = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn revenue_product_seg_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/revenue_product_seg.json").unwrap();
    let _: Vec<RevenueProductSegmentation> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn revenue_geo_seg_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/revenue_geo_seg.json").unwrap();
    let _: Vec<RevenueGeographicSegmentation> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn income_as_reported_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/income_as_reported.json").unwrap();
    let _: Vec<AsReportedIncomeStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn balance_sheet_as_reported_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/balance_sheet_as_reported.json").unwrap();
    let _: Vec<AsReportedBalanceSheet> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn cash_flow_as_reported_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/cash_flow_as_reported.json").unwrap();
    let _: Vec<AsReportedCashFlowStatement> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn key_metrics_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/key_metrics.json").unwrap();
    let _: Vec<KeyMetrics> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn ratios_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/ratios.json").unwrap();
    let _: Vec<Ratios> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn key_metrics_ttm_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/key_metrics_ttm.json").unwrap();
    let items: Vec<KeyMetricsTtm> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(items[0].symbol, "AAPL");
    assert!(items[0].market_cap > 0.0);
  }

  #[test]
  fn ratios_ttm_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/ratios_ttm.json").unwrap();
    let _: Vec<FinancialRatiosTtm> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn financial_scores_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/financial_scores.json").unwrap();
    let _: Vec<FinancialScores> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn owner_earnings_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/owner_earnings.json").unwrap();
    let _: Vec<OwnerEarnings> = serde_json::from_slice(&bytes).unwrap();
  }
}
