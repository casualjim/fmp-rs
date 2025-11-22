use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::primitives::{FmpDate, FmpDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrowdfundingCampaign {
  pub cik: String,
  pub company_name: String,
  pub date: Option<FmpDate>,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub form_type: String,
  pub form_signification: String,
  pub name_of_issuer: String,
  pub legal_status_form: String,
  pub jurisdiction_organization: String,
  pub issuer_street: String,
  pub issuer_city: String,
  pub issuer_state_or_country: String,
  pub issuer_zip_code: String,
  pub issuer_website: String,
  pub intermediary_company_name: String,
  pub intermediary_commission_cik: String,
  pub intermediary_commission_file_number: String,
  pub compensation_amount: String,
  pub financial_interest: String,
  pub security_offered_type: String,
  pub security_offered_other_description: String,
  pub number_of_security_offered: f64,
  pub offering_price: f64,
  pub offering_amount: f64,
  pub over_subscription_accepted: String,
  pub over_subscription_allocation_type: String,
  pub maximum_offering_amount: f64,
  pub offering_deadline_date: FmpDate,
  pub current_number_of_employees: f64,
  pub total_asset_most_recent_fiscal_year: f64,
  pub total_asset_prior_fiscal_year: f64,
  pub cash_and_cash_equi_valent_most_recent_fiscal_year: f64,
  pub cash_and_cash_equi_valent_prior_fiscal_year: f64,
  pub accounts_receivable_most_recent_fiscal_year: f64,
  pub accounts_receivable_prior_fiscal_year: f64,
  pub short_term_debt_most_recent_fiscal_year: f64,
  pub short_term_debt_prior_fiscal_year: f64,
  pub long_term_debt_most_recent_fiscal_year: f64,
  pub long_term_debt_prior_fiscal_year: f64,
  pub revenue_most_recent_fiscal_year: f64,
  pub revenue_prior_fiscal_year: f64,
  pub cost_goods_sold_most_recent_fiscal_year: f64,
  pub cost_goods_sold_prior_fiscal_year: f64,
  pub taxes_paid_most_recent_fiscal_year: f64,
  pub taxes_paid_prior_fiscal_year: f64,
  pub net_income_most_recent_fiscal_year: f64,
  pub net_income_prior_fiscal_year: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrowdfundingSearchResult {
  pub cik: String,
  pub name: String,
  pub date: Option<FmpDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityOffering {
  pub cik: String,
  pub company_name: String,
  pub date: FmpDate,
  pub filing_date: FmpDate,
  pub accepted_date: FmpDateTime,
  pub form_type: String,
  pub form_signification: String,
  pub entity_name: String,
  pub issuer_street: String,
  pub issuer_city: String,
  pub issuer_state_or_country: String,
  pub issuer_state_or_country_description: String,
  pub issuer_zip_code: String,
  pub issuer_phone_number: String,
  pub jurisdiction_of_incorporation: String,
  pub entity_type: String,
  pub incorporated_within_five_years: Option<bool>,
  pub year_of_incorporation: String,
  pub related_person_first_name: String,
  pub related_person_last_name: String,
  pub related_person_street: String,
  pub related_person_city: String,
  pub related_person_state_or_country: String,
  pub related_person_state_or_country_description: String,
  pub related_person_zip_code: String,
  pub related_person_relationship: String,
  pub industry_group_type: String,
  pub revenue_range: Option<String>,
  pub federal_exemptions_exclusions: String,
  pub is_amendment: bool,
  pub date_of_first_sale: FmpDate,
  pub duration_of_offering_is_more_than_year: bool,
  pub securities_offered_are_of_equity_type: bool,
  pub is_business_combination_transaction: bool,
  pub minimum_investment_accepted: f64,
  pub total_offering_amount: f64,
  pub total_amount_sold: f64,
  pub total_amount_remaining: f64,
  pub has_non_accredited_investors: bool,
  pub total_number_already_invested: f64,
  pub sales_commissions: f64,
  pub finders_fees: f64,
  pub gross_proceeds_used: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityOfferingSearchResult {
  pub cik: String,
  pub name: String,
  pub date: FmpDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct CrowdfundingLatestParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CrowdfundingSearchParams {
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CrowdfundingCikParams {
  pub cik: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(into, strip_option)))]
#[serde(rename_all = "camelCase")]
pub struct EquityLatestParams {
  pub page: Option<u32>,
  pub limit: Option<u32>,
  pub cik: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct EquitySearchParams {
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct EquityCikParams {
  pub cik: String,
}
