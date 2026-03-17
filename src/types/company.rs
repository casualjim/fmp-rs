use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

/// Comprehensive company profile returned by the `/profile` and `/profile-cik` endpoints.
///
/// Combines real-time pricing data with static company metadata (sector, industry,
/// CEO, employees, description, exchange, and global identifiers). Optional fields
/// are only present when the company has relevant data available from its filings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfile {
  /// Ticker symbol (e.g., "AAPL").
  pub symbol: String,
  /// Current market price per share.
  #[serde(alias = "price")]
  pub price: f64,
  /// Total market capitalisation: shares outstanding × current price.
  #[serde(alias = "mktCap")]
  pub market_cap: f64,
  /// Beta — measure of price volatility relative to the broader market (1.0 = market risk).
  pub beta: f64,
  /// Most recent annual dividend paid per share, in the reporting currency.
  pub last_dividend: f64,
  /// 52-week price range formatted as "low - high" (e.g., "124.17 - 199.62").
  pub range: String,
  /// Absolute dollar change from the previous session's close.
  pub change: f64,
  /// Percentage change from the previous session's close.
  pub change_percentage: f64,
  /// Shares traded during the current session.
  pub volume: f64,
  /// Average daily trading volume over approximately the past 3 months.
  #[serde(alias = "volAvg")]
  pub average_volume: f64,
  /// Full legal company name (e.g., "Apple Inc.").
  pub company_name: String,
  /// ISO 4217 reporting currency code (e.g., "USD", "EUR", "GBP").
  pub currency: String,
  /// SEC Central Index Key — unique identifier for SEC-registered entities.
  pub cik: String,
  /// International Securities Identification Number (ISO 6166, 12-character alphanumeric).
  pub isin: String,
  /// CUSIP number — 9-character identifier used in the US and Canada.
  pub cusip: String,
  /// Full name of the listing exchange (e.g., "Nasdaq Global Select Market").
  pub exchange_full_name: String,
  /// Short exchange code (e.g., "NASDAQ", "NYSE", "LSE").
  pub exchange: String,
  /// Alternate short exchange code; may differ from `exchange` for cross-listed securities.
  #[serde(alias = "exchangeShortName")]
  #[serde(default)]
  pub exchange_short_name: Option<String>,
  /// GICS industry classification (e.g., "Consumer Electronics", "Semiconductors").
  pub industry: String,
  /// Official company website URL.
  pub website: String,
  /// Business description sourced from the latest annual report or SEC filing.
  pub description: String,
  /// Name of the current Chief Executive Officer.
  pub ceo: String,
  /// GICS sector classification (e.g., "Technology", "Healthcare", "Financials").
  pub sector: String,
  /// ISO 3166-1 alpha-2 country code where the company is incorporated (e.g., "US", "GB").
  pub country: String,
  /// Number of full-time employees as reported in the latest SEC filing.
  pub full_time_employees: String,
  /// Investor relations or main corporate phone number.
  pub phone: String,
  /// Street address of the principal executive offices.
  pub address: String,
  /// City of the principal executive offices.
  pub city: String,
  /// State or province of the principal executive offices.
  pub state: String,
  /// Postal / ZIP code of the principal executive offices.
  pub zip: String,
  /// URL to the company logo image hosted by FMP.
  pub image: String,
  /// Date of the company's initial public offering.
  pub ipo_date: FmpDate,
  /// `true` if FMP is using a generic fallback logo (company-specific logo unavailable).
  pub default_image: bool,
  /// `true` if the security is an Exchange-Traded Fund.
  pub is_etf: bool,
  /// `true` if the security is currently actively trading on its primary exchange.
  pub is_actively_trading: bool,
  /// `true` if the security is an American Depositary Receipt (foreign company listed in the US).
  pub is_adr: bool,
  /// `true` if the security is a mutual fund or closed-end fund.
  pub is_fund: bool,
  /// Most recent balance-sheet cash or total-assets figure (context-dependent).
  #[serde(default, alias = "lastBal")]
  pub last_balance: Option<f64>,
  /// Trailing-twelve-month dividend yield as a decimal (e.g., `0.0058` = 0.58%).
  #[serde(default, alias = "dividendYield")]
  pub dividend_yield: Option<f64>,
  /// Previous ex-dividend date; absent for non-dividend-paying securities.
  #[serde(default, alias = "prevExDividendDate")]
  pub prev_ex_dividend_date: Option<FmpDate>,
  /// Date of the most recently accepted financial report.
  #[serde(default, alias = "reportDate")]
  pub report_date: Option<FmpDate>,
  /// Currency in which the stock price is quoted (may differ from `currency` for cross-listings).
  #[serde(default, alias = "priceCurrency")]
  pub price_currency: Option<String>,
  /// Raw XBRL value from the bottom-left quadrant of the 10-K cover page.
  #[serde(default, alias = "leftBottomForm10k")]
  pub left_bottom_form_10k: Option<String>,
  /// Raw XBRL value from the bottom-right quadrant of the 10-K cover page.
  #[serde(default, alias = "rightBottomForm10k")]
  pub right_bottom_form_10k: Option<String>,
  /// Raw XBRL value from the top-left quadrant of the 10-K cover page.
  #[serde(default, alias = "leftTopForm10k")]
  pub left_top_form_10k: Option<String>,
  /// Raw XBRL value from the top-right quadrant of the 10-K cover page.
  #[serde(default, alias = "rightTopForm10k")]
  pub right_top_form_10k: Option<String>,
  /// Serialised income statement XBRL data as a JSON string (raw, unstandardised).
  #[serde(default, alias = "incomeStatementRaw")]
  pub income_statement_raw: Option<String>,
  /// Serialised balance sheet XBRL data as a JSON string (raw, unstandardised).
  #[serde(default, alias = "balanceSheetRaw")]
  pub balance_sheet_raw: Option<String>,
  /// Serialised cash flow statement XBRL data as a JSON string (raw, unstandardised).
  #[serde(default, alias = "cashFlowRaw")]
  pub cash_flow_raw: Option<String>,
  /// FMP-derived qualitative indicator of management reporting quality or complexity.
  #[serde(default, alias = "managementDifficulty")]
  pub management_difficulty: Option<String>,
}

/// An SEC-registered note or bond issued by a company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyNote {
  /// SEC Central Index Key of the issuing company.
  pub cik: String,
  /// Ticker symbol of the issuing company.
  pub symbol: String,
  /// Official title of the note or debt security.
  pub title: String,
  /// Exchange where the note is listed.
  pub exchange: String,
}

/// A peer company in the same industry and market-cap tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPeer {
  /// Ticker symbol of the peer company.
  pub symbol: String,
  /// Full company name of the peer.
  pub company_name: String,
  /// Current share price of the peer.
  pub price: f64,
  /// Market capitalisation of the peer.
  pub mkt_cap: f64,
}

/// A company that has been delisted from its exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelistedCompany {
  /// Ticker symbol at the time of delisting.
  pub symbol: String,
  /// Company name at the time of delisting.
  pub company_name: String,
  /// Exchange the company was listed on before delisting.
  pub exchange: String,
  /// Date of the company's initial public offering.
  pub ipo_date: FmpDate,
  /// Date the company was removed from the exchange.
  pub delisted_date: FmpDate,
}

/// Historical employee headcount sourced from SEC filings.
///
/// Each record corresponds to a single filing (10-K or proxy statement) that
/// disclosed the number of full-time employees.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeCount {
  /// Ticker symbol.
  pub symbol: String,
  /// SEC Central Index Key of the company.
  pub cik: String,
  /// Date and time the filing was accepted by the SEC EDGAR system.
  pub acceptance_time: FmpDateTime,
  /// End of the period covered by the filing (fiscal year-end or quarter-end).
  pub period_of_report: FmpDate,
  /// Company name as reported in the filing.
  pub company_name: String,
  /// SEC form type that contained the employee count (e.g., "10-K", "DEF 14A").
  pub form_type: String,
  /// Date the filing was submitted to the SEC.
  pub filing_date: FmpDate,
  /// Number of full-time employees as disclosed in the filing.
  pub employee_count: f64,
  /// URL to the source filing on SEC EDGAR.
  pub source: String,
}

/// Point-in-time market capitalisation for a company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCap {
  /// Ticker symbol.
  pub symbol: String,
  /// Date of this market-cap snapshot.
  pub date: FmpDate,
  /// Market capitalisation in the company's reporting currency.
  pub market_cap: f64,
}

/// Share float data — the portion of outstanding shares available for public trading.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareFloat {
  /// Ticker symbol.
  pub symbol: String,
  /// Date of this float snapshot.
  pub date: FmpDate,
  /// Float as a percentage of total shares outstanding (e.g., `0.95` = 95%).
  pub free_float: f64,
  /// Number of shares in the public float (outstanding minus insider/restricted shares).
  pub float_shares: f64,
  /// Total shares outstanding (issued and not yet repurchased or cancelled).
  pub outstanding_shares: f64,
}

/// A merger or acquisition event sourced from SEC Form SC-TO or 8-K filings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergerAcquisition {
  /// Ticker symbol of the acquiring company.
  pub symbol: String,
  /// Name of the acquiring company.
  pub company_name: String,
  /// SEC CIK of the acquiring company.
  pub cik: String,
  /// Name of the company being acquired.
  pub targeted_company_name: String,
  /// SEC CIK of the company being acquired.
  pub targeted_cik: String,
  /// Ticker symbol of the company being acquired (empty if not publicly traded).
  pub targeted_symbol: String,
  /// Date the transaction was announced or agreed upon.
  pub transaction_date: FmpDate,
  /// Date and time the SEC filing was accepted.
  pub accepted_date: FmpDateTime,
  /// URL to the source SEC filing.
  pub link: String,
}

/// A key executive (officer or director) of a company.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyExecutive {
  /// Job title (e.g., "Chief Executive Officer", "Chief Financial Officer").
  pub title: String,
  /// Full name of the executive.
  pub name: String,
  /// Most recent annual total compensation in `currency_pay` units; absent if not disclosed.
  pub pay: Option<f64>,
  /// ISO 4217 currency code for the `pay` figure (e.g., "USD").
  pub currency_pay: String,
  /// Self-reported gender; absent if not disclosed.
  pub gender: Option<String>,
  /// Birth year; absent if not disclosed.
  pub year_born: Option<i32>,
  /// `true` if the executive is currently active; `false` if former; absent if unknown.
  pub active: Option<bool>,
}

/// Executive compensation detail sourced from DEF 14A proxy statement filings.
///
/// Each row corresponds to one named executive officer (NEO) in one fiscal year.
/// Amounts are in the currency reported in the proxy statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutiveCompensation {
  /// SEC CIK of the reporting company.
  pub cik: String,
  /// Ticker symbol.
  pub symbol: String,
  /// Full legal company name.
  pub company_name: String,
  /// Date the proxy statement was filed with the SEC.
  pub filing_date: FmpDate,
  /// Date and time the filing was accepted by SEC EDGAR.
  pub accepted_date: FmpDateTime,
  /// Combined name and title of the executive as reported in the proxy (e.g., "Tim Cook, CEO").
  pub name_and_position: String,
  /// Fiscal year covered by this compensation record.
  pub year: i32,
  /// Base salary paid during the fiscal year.
  pub salary: f64,
  /// Cash bonus paid during the fiscal year.
  pub bonus: f64,
  /// Fair value of stock awards granted during the fiscal year.
  pub stock_award: f64,
  /// Fair value of stock option awards granted during the fiscal year.
  pub option_award: f64,
  /// Non-equity incentive plan compensation earned during the fiscal year.
  pub incentive_plan_compensation: f64,
  /// All other compensation (perquisites, employer 401k match, etc.).
  pub all_other_compensation: f64,
  /// Total of all compensation components above.
  pub total: f64,
  /// URL to the source DEF 14A proxy statement on SEC EDGAR.
  pub link: String,
}

/// Industry-level average executive compensation benchmark for a given year.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutiveCompensationBenchmark {
  /// Industry category title (e.g., "Technology", "Healthcare").
  pub industry_title: String,
  /// Fiscal year of the benchmark data.
  pub year: i32,
  /// Average total compensation across all NEOs in this industry for the year.
  pub average_compensation: f64,
}

// --- Request parameter types ---

/// Parameters for endpoints that take a single ticker symbol.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolParams {
  /// Ticker symbol (e.g., "AAPL").
  pub symbol: String,
}

/// Parameters for batch endpoints that take multiple comma-separated symbols.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolsParams {
  /// Comma-separated ticker symbols (e.g., "AAPL,MSFT,GOOGL").
  pub symbols: String,
}

/// Parameters for endpoints that look up by SEC CIK number.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CikParams {
  /// SEC Central Index Key (e.g., "0000320193" for Apple).
  pub cik: String,
}

/// Pagination parameters for list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
  /// Zero-indexed page number (default: 0).
  pub page: Option<u32>,
  /// Maximum number of results to return per page.
  pub limit: Option<u32>,
}

/// Parameters for symbol-based endpoints with an optional result limit.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SymbolLimitParams {
  /// Ticker symbol (e.g., "AAPL").
  pub symbol: String,
  /// Maximum number of records to return (most-recent first).
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
}

/// Parameters for historical market-cap endpoints with optional date range and limit.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct MarketCapHistoryParams {
  /// Ticker symbol (e.g., "AAPL").
  pub symbol: String,
  /// Maximum number of records to return (most-recent first).
  #[builder(default, setter(strip_option))]
  pub limit: Option<u32>,
  /// Earliest date to include (inclusive).
  #[builder(default, setter(strip_option))]
  pub from: Option<FmpDate>,
  /// Latest date to include (inclusive).
  #[builder(default, setter(strip_option))]
  pub to: Option<FmpDate>,
}

/// Parameters for the key-executives endpoint with optional active-status filter.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct ExecutivesParams {
  /// Ticker symbol (e.g., "AAPL").
  pub symbol: String,
  /// Filter by active status: `"true"` for current executives, `"false"` for former.
  #[builder(default, setter(strip_option))]
  pub active: Option<String>,
}

/// Pagination parameters for the shares-float-all endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct ShareFloatAllParams {
  /// Zero-indexed page number (default: 0).
  pub page: Option<u32>,
  /// Maximum number of results per page.
  pub limit: Option<u32>,
}

/// Parameters for M&A search by company name.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct MnaSearchParams {
  /// Partial or full company name to search for (acquiring or target company).
  pub name: String,
}

#[cfg(test)]
mod tests {
  use super::{
    CompanyExecutive, CompanyNote, CompanyProfile, DelistedCompany, EmployeeCount, ExecutiveCompensation,
    ExecutiveCompensationBenchmark, MarketCap, MergerAcquisition, ShareFloat, StockPeer,
  };

  #[test]
  fn company_profile_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/company_profile.json").unwrap();
    let _: Vec<CompanyProfile> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn company_notes_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/company_notes.json").unwrap();
    let _: Vec<CompanyNote> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn stock_peers_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/stock_peers.json").unwrap();
    let _: Vec<StockPeer> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn delisted_companies_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/delisted_companies.json").unwrap();
    let _: Vec<DelistedCompany> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn employee_count_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/employee_count.json").unwrap();
    let _: Vec<EmployeeCount> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn market_cap_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/market_cap.json").unwrap();
    let _: Vec<MarketCap> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn share_float_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/share_float.json").unwrap();
    let _: Vec<ShareFloat> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn merger_acquisition_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/merger_acquisition.json").unwrap();
    let _: Vec<MergerAcquisition> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn company_executive_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/company_executive.json").unwrap();
    let _: Vec<CompanyExecutive> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn executive_compensation_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/executive_compensation.json").unwrap();
    let _: Vec<ExecutiveCompensation> = serde_json::from_slice(&bytes).unwrap();
  }

  #[test]
  fn executive_compensation_benchmark_fixture_deserializes() {
    let bytes = std::fs::read("tests/fixtures/executive_compensation_benchmark.json").unwrap();
    let _: Vec<ExecutiveCompensationBenchmark> = serde_json::from_slice(&bytes).unwrap();
  }
}
