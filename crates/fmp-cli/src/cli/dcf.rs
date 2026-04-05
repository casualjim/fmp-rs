use clap::{Args, Subcommand};
use eyre::Result;
use fmp::DcfApi;

use super::Context;

#[derive(Subcommand, Debug, Clone)]
pub enum DcfArgs {
  /// Standard DCF intrinsic value using FMP's default assumptions
  Valuation(ValuationArgs),
  /// Levered DCF valuation accounting for company's actual debt structure
  LeveredValuation(LeveredValuationArgs),
  /// Custom DCF with user-supplied growth rate, margins, and WACC inputs
  Custom(CustomArgs),
  /// Levered custom DCF with user-supplied assumptions and debt structure
  CustomLevered(CustomLeveredArgs),
}

impl DcfArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::Valuation(args) => args.handle(ctx).await,
      Self::LeveredValuation(args) => args.handle(ctx).await,
      Self::Custom(args) => args.handle(ctx).await,
      Self::CustomLevered(args) => args.handle(ctx).await,
    }
  }
}

#[derive(Args, Debug, Clone)]
pub struct ValuationArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,
}

impl ValuationArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::endpoints::dcf::SymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.discounted_cash_flow(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct LeveredValuationArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,
}

impl LeveredValuationArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::endpoints::dcf::SymbolParams {
      symbol: self.symbol.clone(),
    };
    let data = ctx.client.levered_discounted_cash_flow(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct CustomArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Revenue growth rate as percentage (e.g., 0.05 for 5%)")]
  pub revenue_growth_pct: Option<f64>,

  #[arg(long, help = "EBITDA as percentage of revenue")]
  pub ebitda_pct: Option<f64>,

  #[arg(long, help = "D&A as percentage of revenue")]
  pub depreciation_and_amortization_pct: Option<f64>,

  #[arg(long, help = "Cash & short-term investments as percentage of revenue")]
  pub cash_and_short_term_investments_pct: Option<f64>,

  #[arg(long, help = "Accounts receivable as percentage of revenue")]
  pub receivables_pct: Option<f64>,

  #[arg(long, help = "Inventories as percentage of revenue")]
  pub inventories_pct: Option<f64>,

  #[arg(long, help = "Accounts payable as percentage of revenue")]
  pub payable_pct: Option<f64>,

  #[arg(long, help = "EBIT as percentage of revenue")]
  pub ebit_pct: Option<f64>,

  #[arg(long, help = "Capital expenditure as percentage of revenue")]
  pub capital_expenditure_pct: Option<f64>,

  #[arg(long, help = "Operating cash flow as percentage of revenue")]
  pub operating_cash_flow_pct: Option<f64>,

  #[arg(long, help = "SG&A as percentage of revenue")]
  pub selling_general_and_administrative_expenses_pct: Option<f64>,

  #[arg(long, help = "Effective tax rate (e.g., 0.21 for 21%)")]
  pub tax_rate: Option<f64>,

  #[arg(long, help = "Terminal/perpetuity growth rate (e.g., 0.03 for 3%)")]
  pub long_term_growth_rate: Option<f64>,

  #[arg(long, help = "Pre-tax cost of debt (e.g., 0.04 for 4%)")]
  pub cost_of_debt: Option<f64>,

  #[arg(long, help = "Cost of equity from CAPM (e.g., 0.10 for 10%)")]
  pub cost_of_equity: Option<f64>,

  #[arg(long, help = "Equity risk premium (e.g., 0.055 for 5.5%)")]
  pub market_risk_premium: Option<f64>,

  #[arg(long, help = "Company beta (systematic risk relative to market)")]
  pub beta: Option<f64>,

  #[arg(long, help = "Risk-free rate, typically 10Y treasury yield")]
  pub risk_free_rate: Option<f64>,
}

impl CustomArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::dcf::CustomDcfInput::builder()
      .symbol(&self.symbol)
      .revenue_growth_pct(self.revenue_growth_pct)
      .ebitda_pct(self.ebitda_pct)
      .depreciation_and_amortization_pct(self.depreciation_and_amortization_pct)
      .cash_and_short_term_investments_pct(self.cash_and_short_term_investments_pct)
      .receivables_pct(self.receivables_pct)
      .inventories_pct(self.inventories_pct)
      .payable_pct(self.payable_pct)
      .ebit_pct(self.ebit_pct)
      .capital_expenditure_pct(self.capital_expenditure_pct)
      .operating_cash_flow_pct(self.operating_cash_flow_pct)
      .selling_general_and_administrative_expenses_pct(self.selling_general_and_administrative_expenses_pct)
      .tax_rate(self.tax_rate)
      .long_term_growth_rate(self.long_term_growth_rate)
      .cost_of_debt(self.cost_of_debt)
      .cost_of_equity(self.cost_of_equity)
      .market_risk_premium(self.market_risk_premium)
      .beta(self.beta)
      .risk_free_rate(self.risk_free_rate)
      .build();
    let data = ctx.client.custom_discounted_cash_flow(params).await?;
    crate::output::output_json(&data)
  }
}

#[derive(Args, Debug, Clone)]
pub struct CustomLeveredArgs {
  #[arg(long, required = true, help = "Ticker symbol (e.g., AAPL)")]
  pub symbol: String,

  #[arg(long, help = "Revenue growth rate as percentage (e.g., 0.05 for 5%)")]
  pub revenue_growth_pct: Option<f64>,

  #[arg(long, help = "EBITDA as percentage of revenue")]
  pub ebitda_pct: Option<f64>,

  #[arg(long, help = "D&A as percentage of revenue")]
  pub depreciation_and_amortization_pct: Option<f64>,

  #[arg(long, help = "Cash & short-term investments as percentage of revenue")]
  pub cash_and_short_term_investments_pct: Option<f64>,

  #[arg(long, help = "Accounts receivable as percentage of revenue")]
  pub receivables_pct: Option<f64>,

  #[arg(long, help = "Inventories as percentage of revenue")]
  pub inventories_pct: Option<f64>,

  #[arg(long, help = "Accounts payable as percentage of revenue")]
  pub payable_pct: Option<f64>,

  #[arg(long, help = "EBIT as percentage of revenue")]
  pub ebit_pct: Option<f64>,

  #[arg(long, help = "Capital expenditure as percentage of revenue")]
  pub capital_expenditure_pct: Option<f64>,

  #[arg(long, help = "Operating cash flow as percentage of revenue")]
  pub operating_cash_flow_pct: Option<f64>,

  #[arg(long, help = "SG&A as percentage of revenue")]
  pub selling_general_and_administrative_expenses_pct: Option<f64>,

  #[arg(long, help = "Effective tax rate (e.g., 0.21 for 21%)")]
  pub tax_rate: Option<f64>,

  #[arg(long, help = "Terminal/perpetuity growth rate (e.g., 0.03 for 3%)")]
  pub long_term_growth_rate: Option<f64>,

  #[arg(long, help = "Pre-tax cost of debt (e.g., 0.04 for 4%)")]
  pub cost_of_debt: Option<f64>,

  #[arg(long, help = "Cost of equity from CAPM (e.g., 0.10 for 10%)")]
  pub cost_of_equity: Option<f64>,

  #[arg(long, help = "Equity risk premium (e.g., 0.055 for 5.5%)")]
  pub market_risk_premium: Option<f64>,

  #[arg(long, help = "Company beta (systematic risk relative to market)")]
  pub beta: Option<f64>,

  #[arg(long, help = "Risk-free rate, typically 10Y treasury yield")]
  pub risk_free_rate: Option<f64>,
}

impl CustomLeveredArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::dcf::CustomDcfInput::builder()
      .symbol(&self.symbol)
      .revenue_growth_pct(self.revenue_growth_pct)
      .ebitda_pct(self.ebitda_pct)
      .depreciation_and_amortization_pct(self.depreciation_and_amortization_pct)
      .cash_and_short_term_investments_pct(self.cash_and_short_term_investments_pct)
      .receivables_pct(self.receivables_pct)
      .inventories_pct(self.inventories_pct)
      .payable_pct(self.payable_pct)
      .ebit_pct(self.ebit_pct)
      .capital_expenditure_pct(self.capital_expenditure_pct)
      .operating_cash_flow_pct(self.operating_cash_flow_pct)
      .selling_general_and_administrative_expenses_pct(self.selling_general_and_administrative_expenses_pct)
      .tax_rate(self.tax_rate)
      .long_term_growth_rate(self.long_term_growth_rate)
      .cost_of_debt(self.cost_of_debt)
      .cost_of_equity(self.cost_of_equity)
      .market_risk_premium(self.market_risk_premium)
      .beta(self.beta)
      .risk_free_rate(self.risk_free_rate)
      .build();
    let data = ctx.client.custom_levered_discounted_cash_flow(params).await?;
    crate::output::output_json(&data)
  }
}
