use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfile {
  pub symbol: String,
  #[serde(alias = "price")]
  pub price: f64,
  #[serde(alias = "mktCap")]
  pub market_cap: f64,
  pub beta: f64,
  pub last_dividend: f64,
  pub range: String,
  pub change: f64,
  pub change_percentage: f64,
  pub volume: f64,
  #[serde(alias = "volAvg")]
  pub average_volume: f64,
  pub company_name: String,
  pub currency: String,
  pub cik: String,
  pub isin: String,
  pub cusip: String,
  pub exchange_full_name: String,
  pub exchange: String,
  #[serde(alias = "exchangeShortName")]
  #[serde(default)]
  pub exchange_short_name: Option<String>,
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
  pub state: String,
  pub zip: String,
  pub image: String,
  pub ipo_date: FmpDate,
  pub default_image: bool,
  pub is_etf: bool,
  pub is_actively_trading: bool,
  pub is_adr: bool,
  pub is_fund: bool,
  #[serde(default, alias = "lastBal")]
  pub last_balance: Option<f64>,
  #[serde(default, alias = "dividendYield")]
  pub dividend_yield: Option<f64>,
  #[serde(default, alias = "prevExDividendDate")]
  pub prev_ex_dividend_date: Option<FmpDate>,
  #[serde(default, alias = "reportDate")]
  pub report_date: Option<FmpDate>,
  #[serde(default, alias = "priceCurrency")]
  pub price_currency: Option<String>,
  #[serde(default, alias = "leftBottomForm10k")]
  pub left_bottom_form_10k: Option<String>,
  #[serde(default, alias = "rightBottomForm10k")]
  pub right_bottom_form_10k: Option<String>,
  #[serde(default, alias = "leftTopForm10k")]
  pub left_top_form_10k: Option<String>,
  #[serde(default, alias = "rightTopForm10k")]
  pub right_top_form_10k: Option<String>,
  #[serde(default, alias = "incomeStatementRaw")]
  pub income_statement_raw: Option<String>,
  #[serde(default, alias = "balanceSheetRaw")]
  pub balance_sheet_raw: Option<String>,
  #[serde(default, alias = "cashFlowRaw")]
  pub cash_flow_raw: Option<String>,
  #[serde(default, alias = "managementDifficulty")]
  pub management_difficulty: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyNote {
  pub cik: String,
  pub symbol: String,
  pub title: String,
  pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPeer {
  pub symbol: String,
  pub company_name: String,
  pub price: f64,
  pub mkt_cap: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelistedCompany {
  pub symbol: String,
  pub company_name: String,
  pub exchange: String,
  pub ipo_date: FmpDate,
  pub delisted_date: FmpDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeCount {
  pub symbol: String,
  pub cik: String,
  pub acceptance_time: FmpDateTime,
  pub period_of_report: FmpDate,
  pub company_name: String,
  pub form_type: String,
  pub filing_date: FmpDate,
  pub employee_count: f64,
  pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCap {
  pub symbol: String,
  pub date: FmpDate,
  pub market_cap: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareFloat {
  pub symbol: String,
  pub date: FmpDate,
  pub free_float: f64,
  pub float_shares: f64,
  pub outstanding_shares: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergerAcquisition {
  pub symbol: String,
  pub company_name: String,
  pub cik: String,
  pub targeted_company_name: String,
  pub targeted_cik: String,
  pub targeted_symbol: String,
  pub transaction_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyExecutive {
  pub title: String,
  pub name: String,
  pub pay: Option<f64>,
  pub currency_pay: String,
  pub gender: Option<String>,
  pub year_born: Option<i32>,
  pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutiveCompensation {
  pub cik: String,
  pub symbol: String,
  pub company_name: String,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub name_and_position: String,
  pub year: i32,
  pub salary: f64,
  pub bonus: f64,
  pub stock_award: f64,
  pub option_award: f64,
  pub incentive_plan_compensation: f64,
  pub all_other_compensation: f64,
  pub total: f64,
  pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutiveCompensationBenchmark {
  pub industry_title: String,
  pub year: i32,
  pub average_compensation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolParams {
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolsParams {
  pub symbols: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CikParams {
  pub cik: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolLimitParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct MarketCapHistoryParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ExecutivesParams {
  pub symbol: String,
  #[builder(default, setter(strip_option))]
  pub active: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct ShareFloatAllParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct MnaSearchParams {
  pub name: String,
}
