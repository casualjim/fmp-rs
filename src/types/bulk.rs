use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::primitives::{de_opt_fmpdate, FmpDate, FmpDateTime};

/// Deserialize ticker symbols as text, including the `NAN` ticker.
///
/// Some bulk statement fixtures contain a real symbol value of `NAN`.
/// In CSV deserialization this token can be surfaced as floating `NaN` instead
/// of a plain string token, which would fail for `symbol: String`.
/// This keeps symbol handling text-first and maps that specific float `NaN`
/// token back to the literal "NAN" symbol.
fn de_symbol_token<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: serde::Deserializer<'de>,
{
  struct SymbolTokenVisitor;

  impl<'de> serde::de::Visitor<'de> for SymbolTokenVisitor {
    type Value = String;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.write_str("a ticker symbol token")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v.to_string())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(v.to_string())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      if v.is_nan() {
        return Ok("NAN".to_owned());
      }
      Ok(v.to_string())
    }
  }

  deserializer.deserialize_any(SymbolTokenVisitor)
}

fn de_opt_u64_empty_or_null<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  struct OptU64Visitor;

  impl<'de> serde::de::Visitor<'de> for OptU64Visitor {
    type Value = Option<u64>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.write_str("an optional u64, empty string, or null")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(None)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      Ok(Some(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      if v < 0 {
        return Err(E::custom("negative value is not valid for u64"));
      }
      Ok(Some(v as u64))
    }

    fn visit_str<E>(self, raw: &str) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      let trimmed = raw.trim();
      if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("null") {
        return Ok(None);
      }
      trimmed.parse::<u64>().map(Some).map_err(E::custom)
    }

    fn visit_string<E>(self, raw: String) -> Result<Self::Value, E>
    where
      E: DeError,
    {
      self.visit_str(&raw)
    }
  }

  deserializer.deserialize_any(OptU64Visitor)
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfile {
  pub symbol: String,
  pub price: f64,
  pub market_cap: f64,
  pub beta: f64,
  pub last_dividend: Option<f64>,
  pub range: String,
  pub change: f64,
  pub change_percentage: f64,
  pub volume: f64,
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
  pub full_time_employees: Option<u64>,
  pub phone: String,
  pub address: String,
  pub city: String,
  pub state: Option<String>,
  pub zip: String,
  pub image: String,
  #[serde(deserialize_with = "de_opt_fmpdate")]
  pub ipo_date: Option<FmpDate>,
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
  pub date: FmpDate,
  pub rating: String,
  pub discounted_cash_flow_score: u8,
  pub return_on_equity_score: u8,
  pub return_on_assets_score: u8,
  pub debt_to_equity_score: u8,
  pub price_to_earnings_score: u8,
  pub price_to_book_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcfValuation {
  pub symbol: String,
  pub date: FmpDate,
  pub dcf: Option<f64>,
  #[serde(rename = "Stock Price")]
  pub stock_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialScore {
  pub symbol: String,
  pub reported_currency: String,
  pub altman_z_score: Option<f64>,
  pub piotroski_score: u8,
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
pub struct PriceTargetSummary {
  pub symbol: String,
  pub last_month_count: u32,
  pub last_month_avg_price_target: f64,
  pub last_quarter_count: u32,
  pub last_quarter_avg_price_target: f64,
  pub last_year_count: u32,
  pub last_year_avg_price_target: f64,
  pub all_time_count: u32,
  pub all_time_avg_price_target: f64,
  pub publishers: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EtfHolder {
  pub symbol: String,
  pub name: Option<String>,
  pub shares_number: f64,
  pub asset: Option<String>,
  pub weight_percentage: f64,
  pub cusip: Option<String>,
  pub isin: Option<String>,
  pub market_value: f64,
  pub last_updated: FmpDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradesDowngradesConsensus {
  pub symbol: String,
  pub strong_buy: u32,
  pub buy: u32,
  pub hold: u32,
  pub sell: u32,
  pub strong_sell: u32,
  pub consensus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyMetricsTtm {
  pub symbol: String,
  #[serde(rename = "marketCap")]
  pub market_cap_ttm: Option<f64>,
  #[serde(rename = "enterpriseValueTTM")]
  pub enterprise_value_ttm: Option<f64>,
  #[serde(rename = "evToSalesTTM")]
  pub ev_to_sales_ttm: Option<f64>,
  #[serde(rename = "evToOperatingCashFlowTTM")]
  pub ev_to_operating_cash_flow_ttm: Option<f64>,
  #[serde(rename = "evToFreeCashFlowTTM")]
  pub ev_to_free_cash_flow_ttm: Option<f64>,
  #[serde(rename = "evToEBITDATTM")]
  pub ev_to_ebitdat_tm: Option<f64>,
  #[serde(rename = "netDebtToEBITDATTM")]
  pub net_debt_to_ebitdat_tm: f64,
  #[serde(rename = "currentRatioTTM")]
  pub current_ratio_ttm: f64,
  #[serde(rename = "incomeQualityTTM")]
  pub income_quality_ttm: f64,
  #[serde(rename = "grahamNumberTTM")]
  pub graham_number_ttm: Option<f64>,
  #[serde(rename = "grahamNetNetTTM")]
  pub graham_net_net_ttm: f64,
  #[serde(rename = "workingCapitalTTM")]
  pub working_capital_ttm: f64,
  #[serde(rename = "investedCapitalTTM")]
  pub invested_capital_ttm: f64,
  #[serde(rename = "returnOnAssetsTTM")]
  pub return_on_assets_ttm: f64,
  #[serde(rename = "returnOnEquityTTM")]
  pub return_on_equity_ttm: f64,
  #[serde(rename = "returnOnInvestedCapitalTTM")]
  pub return_on_invested_capital_ttm: f64,
  #[serde(rename = "earningsYieldTTM")]
  pub earnings_yield_ttm: Option<f64>,
  #[serde(rename = "freeCashFlowYieldTTM")]
  pub free_cash_flow_yield_ttm: Option<f64>,
  #[serde(rename = "capexToOperatingCashFlowTTM")]
  pub capex_to_operating_cash_flow_ttm: f64,
  #[serde(rename = "capexToRevenueTTM")]
  pub capex_to_revenue_ttm: f64,
  #[serde(default)]
  pub other_metrics: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatiosTtm {
  pub symbol: String,
  #[serde(rename = "grossProfitMarginTTM")]
  pub gross_profit_margin_ttm: f64,
  #[serde(rename = "operatingProfitMarginTTM")]
  pub operating_profit_margin_ttm: f64,
  #[serde(rename = "netProfitMarginTTM")]
  pub net_profit_margin_ttm: f64,
  #[serde(rename = "currentRatioTTM")]
  pub current_ratio_ttm: f64,
  #[serde(rename = "quickRatioTTM")]
  pub quick_ratio_ttm: f64,
  #[serde(rename = "priceToEarningsRatioTTM")]
  pub price_to_earnings_ratio_ttm: Option<f64>,
  #[serde(rename = "priceToBookRatioTTM")]
  pub price_to_book_ratio_ttm: Option<f64>,
  #[serde(rename = "priceToSalesRatioTTM")]
  pub price_to_sales_ratio_ttm: Option<f64>,
  #[serde(rename = "debtToEquityRatioTTM")]
  pub debt_to_equity_ratio_ttm: f64,
  #[serde(rename = "dividendYieldTTM")]
  pub dividend_yield_ttm: Option<f64>,
  #[serde(rename = "dividendYieldPercentageTTM", default)]
  pub dividend_yield_percentage_ttm: Option<f64>,
  #[serde(default)]
  pub other_ratios: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockPeer {
  pub symbol: String,
  pub peers: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsSurprise {
  pub symbol: String,
  pub date: FmpDate,
  pub eps_actual: Option<f64>,
  pub eps_estimated: Option<f64>,
  pub last_updated: FmpDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialStatement {
  pub date: FmpDate,
  #[serde(deserialize_with = "de_symbol_token")]
  pub symbol: String,
  pub reported_currency: String,
  #[serde(deserialize_with = "de_opt_u64_empty_or_null")]
  pub cik: Option<u64>,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub fiscal_year: i32,
  pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeStatement {
  #[serde(flatten)]
  pub base: FinancialStatement,
  pub revenue: f64,
  pub cost_of_revenue: f64,
  pub gross_profit: f64,
  pub operating_expenses: f64,
  pub operating_income: f64,
  pub income_before_tax: f64,
  pub income_tax_expense: f64,
  pub net_income: f64,
  pub eps: Option<f64>,
  pub eps_diluted: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeStatementGrowth {
  pub symbol: String,
  pub date: FmpDate,
  pub fiscal_year: i32,
  pub period: String,
  pub reported_currency: String,
  pub growth_revenue: f64,
  pub growth_cost_of_revenue: f64,
  pub growth_gross_profit: f64,
  pub growth_operating_expenses: f64,
  pub growth_operating_income: f64,
  pub growth_net_income: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheetStatement {
  #[serde(flatten)]
  pub base: FinancialStatement,
  pub cash_and_cash_equivalents: f64,
  pub short_term_investments: f64,
  pub total_current_assets: f64,
  pub property_plant_equipment_net: f64,
  pub goodwill: f64,
  pub total_assets: f64,
  pub total_current_liabilities: f64,
  pub total_liabilities: f64,
  pub total_stockholders_equity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceSheetGrowth {
  pub symbol: String,
  pub date: FmpDate,
  pub fiscal_year: i32,
  pub period: String,
  pub reported_currency: String,
  pub growth_cash_and_cash_equivalents: f64,
  pub growth_total_current_assets: f64,
  pub growth_total_assets: f64,
  pub growth_total_liabilities: f64,
  pub growth_total_stockholders_equity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashFlowStatement {
  #[serde(flatten)]
  pub base: FinancialStatement,
  pub net_income: f64,
  pub operating_cash_flow: f64,
  pub capital_expenditure: f64,
  pub free_cash_flow: f64,
  pub net_cash_provided_by_operating_activities: f64,
  pub net_cash_provided_by_investing_activities: f64,
  pub net_cash_provided_by_financing_activities: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashFlowGrowth {
  pub symbol: String,
  pub date: FmpDate,
  pub fiscal_year: i32,
  pub period: String,
  pub reported_currency: String,
  pub growth_net_income: f64,
  pub growth_operating_cash_flow: f64,
  pub growth_capital_expenditure: f64,
  pub growth_free_cash_flow: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EodData {
  pub symbol: String,
  pub date: FmpDate,
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
  pub part: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YearPeriodParams {
  pub year: i32,
  pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsSurpriseParams {
  pub year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EodParams {
  pub date: FmpDate,
}

#[cfg(all(test, feature = "bulk-fixture-tests"))]
mod tests {
  use super::{
    BalanceSheetGrowth, BalanceSheetStatement, CashFlowGrowth, CashFlowStatement, CompanyProfile, DcfValuation,
    EarningsSurprise, EodData, EtfHolder, FinancialScore, IncomeStatement, IncomeStatementGrowth, KeyMetricsTtm,
    PriceTargetSummary, RatiosTtm, StockPeer, StockRating, UpgradesDowngradesConsensus,
  };
  use futures::StreamExt;
  use serde::de::DeserializeOwned;

  async fn read_csv_fixture<T>(path: &str) -> Vec<T>
  where
    T: DeserializeOwned,
  {
    let reader = crate::test_fixtures::open_fixture_xz_reader(path).await.unwrap();
    let mut records = csv_async::AsyncDeserializer::from_reader(reader).into_deserialize::<T>();
    let mut items = Vec::new();
    while let Some(record) = records.next().await {
      items.push(record.unwrap());
    }
    items
  }

  async fn read_header(path: &str) -> String {
    let bytes = crate::test_fixtures::read_fixture_bytes(path).unwrap();
    let content = String::from_utf8(bytes).unwrap();
    content.lines().next().unwrap().to_owned()
  }

  #[tokio::test]
  async fn bulk_profile_header_matches_live_fixture() {
    let header = read_header("tests/fixtures/bulk_profile.csv").await;
    assert_eq!(
      header,
      "\"symbol\",\"price\",\"marketCap\",\"beta\",\"lastDividend\",\"range\",\"change\",\"changePercentage\",\"volume\",\"averageVolume\",\"companyName\",\"currency\",\"cik\",\"isin\",\"cusip\",\"exchangeFullName\",\"exchange\",\"industry\",\"website\",\"description\",\"ceo\",\"sector\",\"country\",\"fullTimeEmployees\",\"phone\",\"address\",\"city\",\"state\",\"zip\",\"image\",\"ipoDate\",\"defaultImage\",\"isEtf\",\"isActivelyTrading\",\"isAdr\",\"isFund\""
    );
  }

  #[tokio::test]
  async fn bulk_profile_fixture_deserializes() {
    let items: Vec<CompanyProfile> = read_csv_fixture("tests/fixtures/bulk_profile.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| item.price.is_finite()));
  }

  #[tokio::test]
  async fn bulk_etf_holder_fixture_deserializes() {
    let items: Vec<EtfHolder> = read_csv_fixture("tests/fixtures/bulk_etf_holder.csv").await;
    assert!(!items.is_empty());
    assert!(
      items
        .iter()
        .any(|item| item.name.as_deref().map(|name| !name.is_empty()).unwrap_or(false))
    );
  }

  #[tokio::test]
  async fn bulk_eod_fixture_deserializes() {
    let items: Vec<EodData> = read_csv_fixture("tests/fixtures/bulk_eod.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| item.close.is_finite()));
  }

  #[tokio::test]
  async fn bulk_earnings_surprises_fixture_deserializes() {
    let items: Vec<EarningsSurprise> = read_csv_fixture("tests/fixtures/bulk_earnings_surprises.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().any(|item| item.eps_actual.is_some()));
  }

  #[tokio::test]
  async fn bulk_income_statement_fixture_deserializes() {
    let items: Vec<IncomeStatement> = read_csv_fixture("tests/fixtures/bulk_income_statement.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.base.symbol.is_empty()));
    assert!(items.iter().all(|item| item.revenue.is_finite()));
  }

  #[tokio::test]
  async fn bulk_balance_sheet_statement_fixture_deserializes() {
    let items: Vec<BalanceSheetStatement> = read_csv_fixture("tests/fixtures/bulk_balance_sheet_statement.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.base.symbol.is_empty()));
    assert!(items.iter().all(|item| item.total_assets.is_finite()));
  }

  #[tokio::test]
  async fn bulk_cash_flow_statement_fixture_deserializes() {
    let items: Vec<CashFlowStatement> = read_csv_fixture("tests/fixtures/bulk_cash_flow_statement.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.base.symbol.is_empty()));
    assert!(items.iter().all(|item| item.free_cash_flow.is_finite()));
  }

  #[tokio::test]
  async fn bulk_income_statement_growth_fixture_deserializes() {
    let items: Vec<IncomeStatementGrowth> = read_csv_fixture("tests/fixtures/bulk_income_statement_growth.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| item.growth_revenue.is_finite()));
  }

  #[tokio::test]
  async fn bulk_balance_sheet_statement_growth_fixture_deserializes() {
    let items: Vec<BalanceSheetGrowth> =
      read_csv_fixture("tests/fixtures/bulk_balance_sheet_statement_growth.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| item.growth_total_assets.is_finite()));
  }

  #[tokio::test]
  async fn bulk_cash_flow_statement_growth_fixture_deserializes() {
    let items: Vec<CashFlowGrowth> = read_csv_fixture("tests/fixtures/bulk_cash_flow_statement_growth.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| item.growth_free_cash_flow.is_finite()));
  }

  #[tokio::test]
  async fn bulk_key_metrics_ttm_fixture_deserializes() {
    let items: Vec<KeyMetricsTtm> = read_csv_fixture("tests/fixtures/bulk_key_metrics_ttm.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().any(|item| item.enterprise_value_ttm.is_some()));
  }

  #[tokio::test]
  async fn bulk_ratios_ttm_fixture_deserializes() {
    let items: Vec<RatiosTtm> = read_csv_fixture("tests/fixtures/bulk_ratios_ttm.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| item.gross_profit_margin_ttm.is_finite()));
  }

  #[tokio::test]
  async fn bulk_scores_fixture_deserializes() {
    let items: Vec<FinancialScore> = read_csv_fixture("tests/fixtures/bulk_scores.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().any(|item| item.altman_z_score.is_some()));
  }

  #[tokio::test]
  async fn bulk_rating_fixture_deserializes() {
    let items: Vec<StockRating> = read_csv_fixture("tests/fixtures/bulk_rating.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| item.discounted_cash_flow_score > 0));
  }

  #[tokio::test]
  async fn bulk_upgrades_downgrades_consensus_fixture_deserializes() {
    let items: Vec<UpgradesDowngradesConsensus> =
      read_csv_fixture("tests/fixtures/bulk_upgrades_downgrades_consensus.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| !item.consensus.is_empty()));
  }

  #[tokio::test]
  async fn bulk_price_target_summary_fixture_deserializes() {
    let items: Vec<PriceTargetSummary> = read_csv_fixture("tests/fixtures/bulk_price_target_summary.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().any(|item| item.last_month_count > 0));
  }

  #[tokio::test]
  async fn bulk_peers_fixture_deserializes() {
    let items: Vec<StockPeer> = read_csv_fixture("tests/fixtures/bulk_peers.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(
      items
        .iter()
        .any(|item| item.peers.as_deref().map(|peers| !peers.is_empty()).unwrap_or(false))
    );
  }

  #[tokio::test]
  async fn bulk_dcf_fixture_deserializes() {
    let items: Vec<DcfValuation> = read_csv_fixture("tests/fixtures/bulk_dcf.csv").await;
    assert!(!items.is_empty());
    assert!(items.iter().all(|item| !item.symbol.is_empty()));
    assert!(items.iter().all(|item| item.stock_price.is_finite()));
  }

}

#[cfg(test)]
mod query_params_tests {
  use super::PartParams;

  #[test]
  fn part_params_serializes_as_integer_string() {
    let query = serde_urlencoded::to_string(PartParams { part: 2 }).unwrap();
    assert_eq!(query, "part=2");
  }
}
