use serde::Deserializer;
use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

fn de_f64_string_or_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  struct F64Visitor;

  impl<'de> serde::de::Visitor<'de> for F64Visitor {
    type Value = f64;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "a float or a string containing a float")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v as f64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v as f64)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      v.parse::<f64>().map_err(E::custom)
    }
  }

  deserializer.deserialize_any(F64Visitor)
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfile {
  pub symbol: String,
  #[serde(deserialize_with = "de_f64_string_or_number")]
  pub price: f64,
  #[serde(deserialize_with = "de_f64_string_or_number")]
  pub market_cap: f64,
  #[serde(deserialize_with = "de_f64_string_or_number")]
  pub beta: f64,
  #[serde(deserialize_with = "de_f64_string_or_number")]
  pub last_dividend: f64,
  pub range: String,
  #[serde(deserialize_with = "de_f64_string_or_number")]
  pub change: f64,
  #[serde(deserialize_with = "de_f64_string_or_number")]
  pub change_percentage: f64,
  #[serde(deserialize_with = "de_f64_string_or_number")]
  pub volume: f64,
  #[serde(deserialize_with = "de_f64_string_or_number")]
  pub average_volume: f64,
  pub company_name: String,
  pub currency: String,
  pub cik: Option<String>,
  pub isin: String,
  pub cusip: Option<String>,
  pub exchange_full_name: String,
  pub exchange: String,
  pub industry: String,
  pub website: String,
  pub description: String,
  pub ceo: String,
  pub sector: String,
  pub country: String,
  pub full_time_employees: String,
  pub phone: String,
  pub address: String,
  pub city: String,
  pub state: Option<String>,
  pub zip: String,
  pub image: String,
  pub ipo_date: String,
  pub default_image: bool,
  pub is_etf: bool,
  pub is_actively_trading: bool,
  pub is_adr: bool,
  pub is_fund: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockRating {
  pub symbol: String,
  pub date: String,
  pub rating: String,
  pub rating_recommendation: String,
  pub rating_details_dcf_recommendation: String,
  pub rating_details_roe_recommendation: String,
  pub rating_details_roa_recommendation: String,
  pub rating_details_de_recommendation: String,
  pub rating_details_pe_recommendation: String,
  pub rating_details_pb_recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcfValuation {
  pub symbol: String,
  pub date: String,
  pub discounted_cash_flow: String,
  pub dcf_percent_diff: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialScore {
  pub symbol: String,
  pub reported_currency: String,
  pub altman_z_score: String,
  pub piotroski_score: String,
  pub working_capital: String,
  pub total_assets: String,
  pub retained_earnings: String,
  pub ebit: String,
  pub market_cap: String,
  pub total_liabilities: String,
  pub revenue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTargetSummary {
  pub symbol: String,
  pub last_month: String,
  pub last_month_avg_pt: String,
  pub last_month_avg_pt_percent_dif: String,
  pub last_quarter: String,
  pub last_quarter_avg_pt: String,
  pub last_quarter_avg_pt_percent_dif: String,
  pub last_year: String,
  pub last_year_avg_pt: String,
  pub last_year_avg_pt_percent_dif: String,
  pub all_time: String,
  pub all_time_avg_pt: String,
  pub all_time_avg_pt_percent_dif: String,
  pub publishers: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EtfHolder {
  pub symbol: String,
  pub shares_number: String,
  pub asset: String,
  pub weight_percentage: String,
  pub cusip: String,
  pub isin: String,
  pub name: String,
  pub market_value: String,
  pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradesDowngradesConsensus {
  pub symbol: String,
  pub strong_buy: String,
  pub buy: String,
  pub hold: String,
  pub sell: String,
  pub strong_sell: String,
  pub consensus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyMetricsTtm {
  pub symbol: String,
  pub market_cap_ttm: String,
  pub enterprise_value_ttm: String,
  pub ev_to_sales_ttm: String,
  pub ev_to_operating_cash_flow_ttm: String,
  pub ev_to_free_cash_flow_ttm: String,
  #[serde(rename = "evToEBITDATTM")]
  pub ev_to_ebitdat_tm: String,
  #[serde(rename = "netDebtToEBITDATTM")]
  pub net_debt_to_ebitdat_tm: String,
  pub current_ratio_ttm: String,
  pub income_quality_ttm: String,
  pub graham_number_ttm: String,
  pub graham_net_net_ttm: String,
  pub working_capital_ttm: String,
  pub invested_capital_ttm: String,
  pub return_on_assets_ttm: String,
  pub return_on_equity_ttm: String,
  pub return_on_invested_capital_ttm: String,
  pub earnings_yield_ttm: String,
  pub free_cash_flow_yield_ttm: String,
  pub capex_to_operating_cash_flow_ttm: String,
  pub capex_to_revenue_ttm: String,
  pub other_metrics: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatiosTtm {
  pub symbol: String,
  pub gross_profit_margin_ttm: String,
  pub operating_profit_margin_ttm: String,
  pub net_profit_margin_ttm: String,
  pub current_ratio_ttm: String,
  pub quick_ratio_ttm: String,
  pub price_to_earnings_ratio_ttm: String,
  pub price_to_book_ratio_ttm: String,
  pub price_to_sales_ratio_ttm: String,
  pub debt_to_equity_ratio_ttm: String,
  pub dividend_yield_ttm: String,
  pub dividend_yield_percentage_ttm: String,
  pub other_ratios: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPeer {
  pub symbol: String,
  pub peers: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsSurprise {
  pub symbol: String,
  pub date: String,
  pub eps_actual: String,
  pub eps_estimated: String,
  pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialStatement {
  pub date: String,
  pub symbol: String,
  pub reported_currency: String,
  pub cik: String,
  pub filing_date: String,
  pub accepted_date: String,
  pub fiscal_year: String,
  pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeStatement {
  #[serde(flatten)]
  pub base: FinancialStatement,
  pub revenue: String,
  pub cost_of_revenue: String,
  pub gross_profit: String,
  pub operating_expenses: String,
  pub operating_income: String,
  pub income_before_tax: String,
  pub income_tax_expense: String,
  pub net_income: String,
  pub eps: String,
  pub eps_diluted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeStatementGrowth {
  pub symbol: String,
  pub date: String,
  pub fiscal_year: String,
  pub period: String,
  pub reported_currency: String,
  pub growth_revenue: String,
  pub growth_cost_of_revenue: String,
  pub growth_gross_profit: String,
  pub growth_operating_expenses: String,
  pub growth_operating_income: String,
  pub growth_net_income: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheetStatement {
  #[serde(flatten)]
  pub base: FinancialStatement,
  pub cash_and_cash_equivalents: String,
  pub short_term_investments: String,
  pub total_current_assets: String,
  pub property_plant_equipment_net: String,
  pub goodwill: String,
  pub total_assets: String,
  pub total_current_liabilities: String,
  pub total_liabilities: String,
  pub total_stockholders_equity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheetGrowth {
  pub symbol: String,
  pub date: String,
  pub fiscal_year: String,
  pub period: String,
  pub reported_currency: String,
  pub growth_cash_and_cash_equivalents: String,
  pub growth_total_current_assets: String,
  pub growth_total_assets: String,
  pub growth_total_liabilities: String,
  pub growth_total_stockholders_equity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashFlowStatement {
  #[serde(flatten)]
  pub base: FinancialStatement,
  pub net_income: String,
  pub operating_cash_flow: String,
  pub capital_expenditure: String,
  pub free_cash_flow: String,
  pub net_cash_provided_by_operating_activities: String,
  pub net_cash_provided_by_investing_activities: String,
  pub net_cash_provided_by_financing_activities: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashFlowGrowth {
  pub symbol: String,
  pub date: String,
  pub fiscal_year: String,
  pub period: String,
  pub reported_currency: String,
  pub growth_net_income: String,
  pub growth_operating_cash_flow: String,
  pub growth_capital_expenditure: String,
  pub growth_free_cash_flow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EodData {
  pub symbol: String,
  pub date: String,
  pub open: f64,
  pub low: f64,
  pub high: f64,
  pub close: f64,
  pub adj_close: f64,
  pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartParams {
  pub part: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YearPeriodParams {
  pub year: String,
  pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsSurpriseParams {
  pub year: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EodParams {
  pub date: String,
}
