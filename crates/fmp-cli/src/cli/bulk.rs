use clap::{Args, Subcommand, ValueEnum};
use eyre::Result;
use fmp::BulkApi;
use futures::StreamExt;
use serde::Serialize;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use super::Context;

#[derive(ValueEnum, Debug, Clone, Default)]
pub enum OutputFormat {
  #[default]
  Json,
  Csv,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BulkArgs {
  /// Full company profiles for all symbols (part 0-3; use --all for all parts)
  Profile(PartArgs),
  /// ETF holdings for all ETFs (part 0-3; use --all for all parts)
  EtfHolder(PartArgs),
  /// End-of-day prices for all symbols for a given date
  Eod(EodArgs),
  /// Earnings surprises for all symbols for a given year
  EarningsSurprises(EarningsSurprisesArgs),
  /// Income statements for all symbols for a given year and period
  IncomeStatement(YearPeriodArgs),
  /// Balance sheet statements for all symbols for a given year and period
  BalanceSheetStatement(YearPeriodArgs),
  /// Cash flow statements for all symbols for a given year and period
  CashFlowStatement(YearPeriodArgs),
  /// Income statement growth for all symbols for a given year and period
  IncomeStatementGrowth(YearPeriodArgs),
  /// Balance sheet statement growth for all symbols for a given year and period
  BalanceSheetStatementGrowth(YearPeriodArgs),
  /// Cash flow statement growth for all symbols for a given year and period
  CashFlowStatementGrowth(YearPeriodArgs),
  /// Trailing twelve months key metrics for all symbols
  KeyMetricsTtm(NoArgs),
  /// Trailing twelve months ratios for all symbols
  RatiosTtm(NoArgs),
  /// Financial scores (Altman Z, Piotroski) for all symbols
  Scores(NoArgs),
  /// Stock ratings for all symbols
  Rating(NoArgs),
  /// Upgrades/downgrades consensus for all symbols
  UpgradesDowngradesConsensus(NoArgs),
  /// Price target summary for all symbols
  PriceTargetSummary(NoArgs),
  /// Stock peers for all symbols
  Peers(NoArgs),
  /// DCF valuations for all symbols
  Dcf(NoArgs),
}

impl BulkArgs {
  pub async fn handle(&self, ctx: &Context) -> Result<()> {
    match self {
      Self::Profile(args) => args.handle_profile(ctx).await,
      Self::EtfHolder(args) => args.handle_etf_holder(ctx).await,
      Self::Eod(args) => args.handle(ctx).await,
      Self::EarningsSurprises(args) => args.handle(ctx).await,
      Self::IncomeStatement(args) => args.handle_income_statement(ctx).await,
      Self::BalanceSheetStatement(args) => args.handle_balance_sheet_statement(ctx).await,
      Self::CashFlowStatement(args) => args.handle_cash_flow_statement(ctx).await,
      Self::IncomeStatementGrowth(args) => args.handle_income_statement_growth(ctx).await,
      Self::BalanceSheetStatementGrowth(args) => args.handle_balance_sheet_statement_growth(ctx).await,
      Self::CashFlowStatementGrowth(args) => args.handle_cash_flow_statement_growth(ctx).await,
      Self::KeyMetricsTtm(args) => stream_and_print(ctx.client.key_metrics_ttm(()).await?, &args.format.format).await,
      Self::RatiosTtm(args) => stream_and_print(ctx.client.ratios_ttm(()).await?, &args.format.format).await,
      Self::Scores(args) => stream_and_print(ctx.client.scores(()).await?, &args.format.format).await,
      Self::Rating(args) => stream_and_print(ctx.client.rating(()).await?, &args.format.format).await,
      Self::UpgradesDowngradesConsensus(args) => {
        stream_and_print(ctx.client.upgrades_downgrades_consensus(()).await?, &args.format.format).await
      }
      Self::PriceTargetSummary(args) => {
        stream_and_print(ctx.client.price_target_summary(()).await?, &args.format.format).await
      }
      Self::Peers(args) => stream_and_print(ctx.client.peers(()).await?, &args.format.format).await,
      Self::Dcf(args) => stream_and_print(ctx.client.dcf(()).await?, &args.format.format).await,
    }
  }
}

/// Consume a stream, printing each record in the chosen format.
async fn stream_and_print<T>(stream: impl futures::Stream<Item = fmp::FmpResult<T>>, fmt: &OutputFormat) -> Result<()>
where
  T: Serialize,
{
  futures::pin_mut!(stream);
  match fmt {
    OutputFormat::Json => {
      while let Some(record) = stream.next().await {
        println!("{}", serde_json::to_string(&record?)?);
      }
    }
    OutputFormat::Csv => {
      let mut wtr = csv::Writer::from_writer(std::io::stdout());
      while let Some(record) = stream.next().await {
        wtr.serialize(&record?)?;
      }
      wtr.flush()?;
    }
  }
  Ok(())
}

/// For part-based endpoints: fetch a single part or all 4 parts with 60s pacing.
async fn fetch_parts<F, Fut, T>(fetch: F, part: u8, all: bool, fmt: &OutputFormat) -> Result<()>
where
  F: Fn(u8) -> Fut,
  Fut: std::future::Future<Output = fmp::FmpResult<futures::stream::BoxStream<'static, fmp::FmpResult<T>>>>,
  T: Serialize,
{
  if all {
    for p in 0u8..4 {
      let start = Instant::now();
      stream_and_print(fetch(p).await?, fmt).await?;
      if p < 3 {
        let elapsed = start.elapsed();
        let pace = Duration::from_secs(60);
        if elapsed < pace {
          sleep(pace - elapsed).await;
        }
      }
    }
  } else {
    stream_and_print(fetch(part).await?, fmt).await?;
  }
  Ok(())
}

/// Shared output format flag, mixed into each args struct via `#[command(flatten)]`.
#[derive(Args, Debug, Clone)]
pub struct FormatArgs {
  /// Output format: `json` (NDJSON, one record per line) or `csv` (with header row)
  #[arg(long, value_enum, default_value = "json")]
  pub format: OutputFormat,
}

#[derive(Args, Debug, Clone)]
pub struct PartArgs {
  /// Part number (0–3)
  #[arg(long, value_parser = clap::value_parser!(u8).range(0..=3), default_value_t = 0)]
  pub part: u8,

  /// Fetch all 4 parts sequentially, pacing to respect the 60 s rate limit
  #[arg(long, conflicts_with = "part")]
  pub all: bool,

  #[command(flatten)]
  pub output: FormatArgs,
}

impl PartArgs {
  async fn handle_profile(&self, ctx: &Context) -> Result<()> {
    let client = &ctx.client;
    fetch_parts(
      |p| client.profile(fmp::types::bulk::PartParams { part: p }),
      self.part,
      self.all,
      &self.output.format,
    )
    .await
  }

  async fn handle_etf_holder(&self, ctx: &Context) -> Result<()> {
    let client = &ctx.client;
    fetch_parts(
      |p| client.etf_holder(fmp::types::bulk::PartParams { part: p }),
      self.part,
      self.all,
      &self.output.format,
    )
    .await
  }
}

#[derive(Args, Debug, Clone)]
pub struct EodArgs {
  /// Date in YYYY-MM-DD format
  #[arg(long)]
  pub date: jiff::civil::Date,

  #[command(flatten)]
  pub output: FormatArgs,
}

impl EodArgs {
  async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::bulk::EodParams {
      date: fmp::FmpDate(self.date),
    };
    stream_and_print(ctx.client.eod(params).await?, &self.output.format).await
  }
}

#[derive(Args, Debug, Clone)]
pub struct EarningsSurprisesArgs {
  /// Fiscal year (e.g. 2024)
  #[arg(long)]
  pub year: i32,

  #[command(flatten)]
  pub output: FormatArgs,
}

impl EarningsSurprisesArgs {
  async fn handle(&self, ctx: &Context) -> Result<()> {
    let params = fmp::types::bulk::EarningsSurpriseParams { year: self.year };
    stream_and_print(ctx.client.earnings_surprises(params).await?, &self.output.format).await
  }
}

#[derive(Args, Debug, Clone)]
pub struct YearPeriodArgs {
  /// Fiscal year (e.g. 2024)
  #[arg(long)]
  pub year: i32,

  /// Fiscal period: Q1, Q2, Q3, Q4, or FY
  #[arg(long, value_parser = ["Q1", "Q2", "Q3", "Q4", "FY"])]
  pub period: String,

  #[command(flatten)]
  pub output: FormatArgs,
}

impl YearPeriodArgs {
  fn params(&self) -> fmp::types::bulk::YearPeriodParams {
    fmp::types::bulk::YearPeriodParams {
      year: self.year,
      period: self.period.clone(),
    }
  }

  async fn handle_income_statement(&self, ctx: &Context) -> Result<()> {
    stream_and_print(ctx.client.income_statement(self.params()).await?, &self.output.format).await
  }

  async fn handle_balance_sheet_statement(&self, ctx: &Context) -> Result<()> {
    stream_and_print(
      ctx.client.balance_sheet_statement(self.params()).await?,
      &self.output.format,
    )
    .await
  }

  async fn handle_cash_flow_statement(&self, ctx: &Context) -> Result<()> {
    stream_and_print(
      ctx.client.cash_flow_statement(self.params()).await?,
      &self.output.format,
    )
    .await
  }

  async fn handle_income_statement_growth(&self, ctx: &Context) -> Result<()> {
    stream_and_print(
      ctx.client.income_statement_growth(self.params()).await?,
      &self.output.format,
    )
    .await
  }

  async fn handle_balance_sheet_statement_growth(&self, ctx: &Context) -> Result<()> {
    stream_and_print(
      ctx.client.balance_sheet_statement_growth(self.params()).await?,
      &self.output.format,
    )
    .await
  }

  async fn handle_cash_flow_statement_growth(&self, ctx: &Context) -> Result<()> {
    stream_and_print(
      ctx.client.cash_flow_statement_growth(self.params()).await?,
      &self.output.format,
    )
    .await
  }
}

/// Args for no-parameter endpoints — only carries the output format flag.
#[derive(Args, Debug, Clone)]
pub struct NoArgs {
  #[command(flatten)]
  pub format: FormatArgs,
}
