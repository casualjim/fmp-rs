#!/usr/bin/env bash
# Fetches one real API response per unique response type and saves it as a
# test fixture under tests/fixtures/. Run once; commit the results.
#
# Usage:
#   FMP_API_KEY=your_key bash scripts/fetch-fixtures.sh
#   FMP_API_KEY=your_key bash scripts/fetch-fixtures.sh quotes  # single section
#
# Requires: curl, jq

set -euo pipefail

BASE="https://financialmodelingprep.com/stable"
KEY="${FMP_API_KEY:?set FMP_API_KEY before running}"
DIR="tests/fixtures"
SECTION="${1:-all}"

mkdir -p "$DIR"

# Trim array responses to 2 items; pass objects through unchanged.
trim() { jq 'if type == "array" then .[0:2] else . end'; }

ok=0; fail=0; skip=0

fetch() {
    local name=$1 url=$2
    if [[ -f "$DIR/$name.json" && "${FORCE:-}" != "1" ]]; then
        echo "  skip  $name  (exists; set FORCE=1 to overwrite)"
        (( skip++ )) || true
        return
    fi
    local tmp
    tmp=$(mktemp)
    local http_code
    http_code=$(curl -sL -o "$tmp" -w "%{http_code}" "$url")
    if [[ "$http_code" == "200" ]]; then
        trim < "$tmp" > "$DIR/$name.json"
        echo "  ok    $name"
        (( ok++ )) || true
    else
        echo "  FAIL  $name  (HTTP $http_code): $(cat "$tmp")"
        (( fail++ )) || true
    fi
    rm -f "$tmp"
}

run_section() {
    local s=$1
    [[ "$SECTION" == "all" || "$SECTION" == "$s" ]]
}

# ─── QUOTES ──────────────────────────────────────────────────────────────────
if run_section quotes; then
echo "── quotes ──"
fetch stock_quote             "$BASE/quote?symbol=AAPL&apikey=$KEY"
fetch stock_quote_short       "$BASE/quote-short?symbol=AAPL&apikey=$KEY"
fetch aftermarket_trade       "$BASE/aftermarket-trade?symbol=AAPL&apikey=$KEY"
fetch aftermarket_quote       "$BASE/aftermarket-quote?symbol=AAPL&apikey=$KEY"
fetch stock_price_change      "$BASE/stock-price-change?symbol=AAPL&apikey=$KEY"
fi

# ─── CHART ───────────────────────────────────────────────────────────────────
if run_section chart; then
echo "── chart ──"
fetch chart_light             "$BASE/historical-price-eod/light?symbol=AAPL&from=2024-01-01&to=2024-01-31&apikey=$KEY"
fetch chart_full              "$BASE/historical-price-eod/full?symbol=AAPL&from=2024-01-01&to=2024-01-31&apikey=$KEY"
fetch chart_unadjusted        "$BASE/historical-price-eod/non-split-adjusted?symbol=AAPL&from=2024-01-01&to=2024-01-31&apikey=$KEY"
fetch chart_intraday          "$BASE/historical-chart/1hour?symbol=AAPL&from=2024-01-02&to=2024-01-03&apikey=$KEY"
fi

# ─── COMPANY ─────────────────────────────────────────────────────────────────
if run_section company; then
echo "── company ──"
fetch company_profile         "$BASE/profile?symbol=AAPL&apikey=$KEY"
fetch company_notes           "$BASE/company-notes?symbol=AAPL&apikey=$KEY"
fetch stock_peers             "$BASE/stock-peers?symbol=AAPL&apikey=$KEY"
fetch delisted_companies      "$BASE/delisted-companies?limit=2&apikey=$KEY"
fetch employee_count          "$BASE/employee-count?symbol=AAPL&limit=2&apikey=$KEY"
fetch market_cap              "$BASE/market-capitalization?symbol=AAPL&apikey=$KEY"
fetch share_float             "$BASE/shares-float?symbol=AAPL&apikey=$KEY"
fetch merger_acquisition      "$BASE/mergers-acquisitions-latest?limit=2&apikey=$KEY"
fetch company_executive       "$BASE/key-executives?symbol=AAPL&apikey=$KEY"
fetch executive_compensation  "$BASE/governance-executive-compensation?symbol=AAPL&apikey=$KEY"
fetch executive_compensation_benchmark "$BASE/executive-compensation-benchmark?symbol=AAPL&apikey=$KEY"
fi

# ─── STATEMENTS ──────────────────────────────────────────────────────────────
if run_section statements; then
echo "── statements ──"
fetch income_statement        "$BASE/income-statement?symbol=AAPL&limit=2&apikey=$KEY"
fetch balance_sheet           "$BASE/balance-sheet-statement?symbol=AAPL&limit=2&apikey=$KEY"
fetch cash_flow               "$BASE/cash-flow-statement?symbol=AAPL&limit=2&apikey=$KEY"
fetch latest_financial        "$BASE/latest-financial-statements?limit=2&apikey=$KEY"
fetch income_statement_ttm    "$BASE/income-statement-ttm?symbol=AAPL&apikey=$KEY"
fetch balance_sheet_ttm       "$BASE/balance-sheet-statement-ttm?symbol=AAPL&apikey=$KEY"
fetch cash_flow_ttm           "$BASE/cash-flow-statement-ttm?symbol=AAPL&apikey=$KEY"
fetch income_growth           "$BASE/income-statement-growth?symbol=AAPL&limit=2&apikey=$KEY"
fetch balance_sheet_growth    "$BASE/balance-sheet-statement-growth?symbol=AAPL&limit=2&apikey=$KEY"
fetch cash_flow_growth        "$BASE/cash-flow-statement-growth?symbol=AAPL&limit=2&apikey=$KEY"
fetch financial_growth        "$BASE/financial-growth?symbol=AAPL&limit=2&apikey=$KEY"
fetch financial_report_dates  "$BASE/financial-reports-dates?symbol=AAPL&apikey=$KEY"
fetch financial_report_10k    "$BASE/financial-reports-json?symbol=AAPL&year=2024&period=FY&apikey=$KEY"
fetch revenue_product_seg     "$BASE/revenue-product-segmentation?symbol=AAPL&apikey=$KEY"
fetch revenue_geo_seg         "$BASE/revenue-geographic-segmentation?symbol=AAPL&apikey=$KEY"
fetch income_as_reported      "$BASE/income-statement-as-reported?symbol=AAPL&limit=1&apikey=$KEY"
fetch balance_sheet_as_reported "$BASE/balance-sheet-statement-as-reported?symbol=AAPL&limit=1&apikey=$KEY"
fetch cash_flow_as_reported   "$BASE/cash-flow-statement-as-reported?symbol=AAPL&limit=1&apikey=$KEY"
fetch key_metrics             "$BASE/key-metrics?symbol=AAPL&limit=2&apikey=$KEY"
fetch ratios                  "$BASE/ratios?symbol=AAPL&limit=2&apikey=$KEY"
# key_metrics_ttm already exists
fetch ratios_ttm              "$BASE/ratios-ttm?symbol=AAPL&apikey=$KEY"
fetch financial_scores        "$BASE/financial-scores?symbol=AAPL&apikey=$KEY"
fetch owner_earnings          "$BASE/owner-earnings?symbol=AAPL&apikey=$KEY"
fi

# ─── CALENDAR ────────────────────────────────────────────────────────────────
if run_section calendar; then
echo "── calendar ──"
fetch dividends               "$BASE/dividends?symbol=AAPL&limit=2&apikey=$KEY"
fetch earnings                "$BASE/earnings?symbol=AAPL&limit=2&apikey=$KEY"
fetch ipo                     "$BASE/ipos-calendar?from=2024-01-01&to=2024-03-31&apikey=$KEY"
fetch ipo_disclosure          "$BASE/ipos-disclosure?from=2024-01-01&to=2024-03-31&apikey=$KEY"
fetch ipo_prospectus          "$BASE/ipos-prospectus?from=2024-01-01&to=2024-03-31&apikey=$KEY"
fetch stock_split             "$BASE/splits?symbol=AAPL&limit=2&apikey=$KEY"
fi

# ─── NEWS ─────────────────────────────────────────────────────────────────────
if run_section news; then
echo "── news ──"
fetch fmp_article             "$BASE/fmp-articles?limit=2&apikey=$KEY"
fetch stock_news              "$BASE/news/stock-latest?limit=2&apikey=$KEY"
fi

# ─── ANALYST ─────────────────────────────────────────────────────────────────
if run_section analyst; then
echo "── analyst ──"
fetch analyst_estimate        "$BASE/analyst-estimates?symbol=AAPL&period=annual&limit=2&apikey=$KEY"
fetch ratings_snapshot        "$BASE/ratings-snapshot?symbol=AAPL&limit=2&apikey=$KEY"
fetch ratings_historical      "$BASE/ratings-historical?symbol=AAPL&limit=2&apikey=$KEY"
fetch price_target_summary    "$BASE/price-target-summary?symbol=AAPL&apikey=$KEY"
fetch price_target_consensus  "$BASE/price-target-consensus?symbol=AAPL&apikey=$KEY"
fetch price_target_news       "$BASE/price-target-latest-news?limit=2&apikey=$KEY"
fetch stock_grade             "$BASE/grades?symbol=AAPL&limit=2&apikey=$KEY"
fetch stock_grade_historical  "$BASE/grades-historical?symbol=AAPL&limit=2&apikey=$KEY"
fetch stock_grade_summary     "$BASE/grades-consensus?symbol=AAPL&apikey=$KEY"
fetch stock_grade_news        "$BASE/grades-latest-news?limit=2&apikey=$KEY"
fi

# ─── SEARCH ──────────────────────────────────────────────────────────────────
if run_section search; then
echo "── search ──"
fetch search_symbol           "$BASE/search-symbol?query=AAPL&limit=2&apikey=$KEY"
fetch search_name             "$BASE/search-name?query=Apple&limit=2&apikey=$KEY"
fetch search_cik              "$BASE/search-cik?cik=0000320193&apikey=$KEY"
fetch search_cusip            "$BASE/search-cusip?cusip=037833100&apikey=$KEY"
fetch search_isin             "$BASE/search-isin?isin=US0378331005&apikey=$KEY"
fetch stock_screener          "$BASE/company-screener?exchange=NASDAQ&limit=2&apikey=$KEY"
fetch exchange_variants       "$BASE/search-exchange-variants?symbol=AAPL&apikey=$KEY"
fi

# ─── MARKET PERFORMANCE ──────────────────────────────────────────────────────
if run_section market_performance; then
echo "── market_performance ──"
fetch biggest_gainers         "$BASE/biggest-gainers?apikey=$KEY"
fetch biggest_losers          "$BASE/biggest-losers?apikey=$KEY"
fetch most_actives            "$BASE/most-actives?apikey=$KEY"
fetch sector_performance      "$BASE/sector-performance-snapshot?date=2024-12-31&apikey=$KEY"
fetch industry_performance    "$BASE/industry-performance-snapshot?date=2024-12-31&apikey=$KEY"
fetch sector_pe               "$BASE/sector-pe-snapshot?date=2024-12-31&apikey=$KEY"
fetch industry_pe             "$BASE/industry-pe-snapshot?date=2024-12-31&apikey=$KEY"
fi

# ─── CRYPTO ──────────────────────────────────────────────────────────────────
if run_section crypto; then
echo "── crypto ──"
fetch cryptocurrency          "$BASE/cryptocurrency-list?symbol=BTCUSD&apikey=$KEY"
fetch crypto_quote            "$BASE/quote?symbol=BTCUSD&apikey=$KEY"
fetch crypto_quote_short      "$BASE/quote-short?symbol=BTCUSD&apikey=$KEY"
fetch crypto_chart_light      "$BASE/historical-price-eod/light?symbol=BTCUSD&from=2024-01-01&to=2024-01-07&apikey=$KEY"
fetch crypto_chart_full       "$BASE/historical-price-eod/full?symbol=BTCUSD&from=2024-01-01&to=2024-01-07&apikey=$KEY"
fetch crypto_chart_intraday   "$BASE/historical-chart/1hour?symbol=BTCUSD&from=2024-01-02&to=2024-01-03&apikey=$KEY"
fi

# ─── FOREX ───────────────────────────────────────────────────────────────────
if run_section forex; then
echo "── forex ──"
fetch forex_pair              "$BASE/forex-list?apikey=$KEY"
fetch forex_quote             "$BASE/quote?symbol=EURUSD&apikey=$KEY"
fetch forex_quote_short       "$BASE/quote-short?symbol=EURUSD&apikey=$KEY"
fetch forex_chart_light       "$BASE/historical-price-eod/light?symbol=EURUSD&from=2024-01-01&to=2024-01-07&apikey=$KEY"
fetch forex_chart_full        "$BASE/historical-price-eod/full?symbol=EURUSD&from=2024-01-01&to=2024-01-07&apikey=$KEY"
fetch forex_chart_intraday    "$BASE/historical-chart/1hour?symbol=EURUSD&from=2024-01-02&to=2024-01-03&apikey=$KEY"
fi

# ─── INDEXES ─────────────────────────────────────────────────────────────────
if run_section indexes; then
echo "── indexes ──"
fetch index_item              "$BASE/index-list?symbol=%5EGSPC&apikey=$KEY"
fetch index_quote             "$BASE/quote?symbol=%5EGSPC&apikey=$KEY"
fetch index_quote_short       "$BASE/quote-short?symbol=%5EGSPC&apikey=$KEY"
fetch index_chart_light       "$BASE/historical-price-eod/light?symbol=%5EGSPC&from=2024-01-01&to=2024-01-07&apikey=$KEY"
fetch index_chart_full        "$BASE/historical-price-eod/full?symbol=%5EGSPC&from=2024-01-01&to=2024-01-07&apikey=$KEY"
fetch index_chart_intraday    "$BASE/historical-chart/1hour?symbol=%5EGSPC&from=2024-01-02&to=2024-01-03&apikey=$KEY"
fi

# ─── ECONOMICS ───────────────────────────────────────────────────────────────
if run_section economics; then
echo "── economics ──"
fetch treasury_rate           "$BASE/treasury-rates?from=2024-01-01&to=2024-01-31&apikey=$KEY"
fetch economic_indicator      "$BASE/economic-indicators?name=GDP&from=2024-01-01&to=2024-12-31&apikey=$KEY"
fetch economic_calendar       "$BASE/economic-calendar?from=2024-01-01&to=2024-01-15&apikey=$KEY"
fetch market_risk_premium     "$BASE/market-risk-premium?apikey=$KEY"
fi

# ─── ESG ─────────────────────────────────────────────────────────────────────
if run_section esg; then
echo "── esg ──"
fetch esg_disclosure          "$BASE/esg-disclosures?symbol=AAPL&apikey=$KEY"
fetch esg_rating              "$BASE/esg-ratings?symbol=AAPL&apikey=$KEY"
fetch esg_benchmark           "$BASE/esg-benchmark?year=2023&apikey=$KEY"
fi

# ─── TECHNICAL INDICATORS ────────────────────────────────────────────────────
if run_section technical; then
echo "── technical ──"
TECH="symbol=AAPL&periodLength=14&timeframe=1hour&from=2024-01-02&to=2024-01-31&apikey=$KEY"
fetch sma_indicator           "$BASE/technical-indicators/sma?$TECH"
fetch ema_indicator           "$BASE/technical-indicators/ema?$TECH"
fetch wma_indicator           "$BASE/technical-indicators/wma?$TECH"
fetch dema_indicator          "$BASE/technical-indicators/dema?$TECH"
fetch tema_indicator          "$BASE/technical-indicators/tema?$TECH"
fetch rsi_indicator           "$BASE/technical-indicators/rsi?$TECH"
fetch stddev_indicator        "$BASE/technical-indicators/standarddeviation?$TECH"
fetch williams_indicator      "$BASE/technical-indicators/williams?$TECH"
fetch adx_indicator           "$BASE/technical-indicators/adx?$TECH"
fi

# ─── DCF ─────────────────────────────────────────────────────────────────────
if run_section dcf; then
echo "── dcf ──"
fetch dcf_valuation           "$BASE/discounted-cash-flow?symbol=AAPL&apikey=$KEY"
fetch dcf_levered             "$BASE/levered-discounted-cash-flow?symbol=AAPL&apikey=$KEY"
fetch dcf_custom              "$BASE/custom-discounted-cash-flow?symbol=AAPL&apikey=$KEY"
fi

# ─── COT ─────────────────────────────────────────────────────────────────────
if run_section cot; then
echo "── cot ──"
fetch cot_report              "$BASE/commitment-of-traders-report?symbol=GC&from=2024-01-01&to=2024-03-31&apikey=$KEY"
fetch cot_analysis            "$BASE/commitment-of-traders-analysis?symbol=GC&from=2024-01-01&to=2024-03-31&apikey=$KEY"
fi

# ─── INSIDER TRADES ──────────────────────────────────────────────────────────
if run_section insider; then
echo "── insider ──"
fetch insider_trading         "$BASE/insider-trading/latest?limit=2&apikey=$KEY"
fetch insider_reporting_name  "$BASE/insider-trading/reporting-name?name=Cook&apikey=$KEY"
fetch insider_transaction_type "$BASE/insider-trading-transaction-type?symbol=AAPL&apikey=$KEY"
fetch insider_statistics      "$BASE/insider-trading/statistics?symbol=AAPL&apikey=$KEY"
fetch acquisition_ownership   "$BASE/acquisition-of-beneficial-ownership?symbol=AAPL&limit=2&apikey=$KEY"
fi

# ─── MARKET HOURS ────────────────────────────────────────────────────────────
if run_section market_hours; then
echo "── market_hours ──"
fetch exchange_market_hours   "$BASE/exchange-market-hours?exchange=NYSE&apikey=$KEY"
fetch holidays_by_exchange    "$BASE/holidays-by-exchange?exchange=NYSE&from=2024-01-01&to=2024-12-31&apikey=$KEY"
fi

# ─── SEC FILINGS ─────────────────────────────────────────────────────────────
if run_section sec_filings; then
echo "── sec_filings ──"
fetch sec_filing              "$BASE/sec-filings-search/symbol?symbol=AAPL&from=2024-01-01&to=2024-12-31&limit=2&apikey=$KEY"
fetch sec_company_search      "$BASE/sec-filings-company-search/symbol?symbol=AAPL&apikey=$KEY"
fetch sec_company_profile     "$BASE/sec-profile?symbol=AAPL&apikey=$KEY"
fetch industry_classification "$BASE/standard-industrial-classification-list?sic_code=3674&apikey=$KEY"
fi

# ─── FORM 13F ────────────────────────────────────────────────────────────────
if run_section form13f; then
echo "── form13f ──"
CIK="0001067983"  # Berkshire Hathaway
fetch inst_ownership_latest   "$BASE/institutional-ownership/latest?limit=2&apikey=$KEY"
fetch inst_ownership_extract  "$BASE/institutional-ownership/extract?cik=$CIK&year=2024&quarter=1&apikey=$KEY"
fetch inst_ownership_dates    "$BASE/institutional-ownership/dates?cik=$CIK&apikey=$KEY"
fetch inst_ownership_holder   "$BASE/institutional-ownership/holder-performance-summary?cik=$CIK&apikey=$KEY"
fetch inst_ownership_industry "$BASE/institutional-ownership/industry-summary?year=2024&quarter=1&apikey=$KEY"
fi

# ─── FUND ────────────────────────────────────────────────────────────────────
if run_section fund; then
echo "── fund ──"
fetch etf_holding             "$BASE/etf/holdings?symbol=SPY&apikey=$KEY"
fetch etf_info                "$BASE/etf/info?symbol=SPY&apikey=$KEY"
fetch etf_country             "$BASE/etf/country-weightings?symbol=SPY&apikey=$KEY"
fetch etf_asset_exposure      "$BASE/etf/asset-exposure?symbol=SPY&apikey=$KEY"
fetch etf_sector              "$BASE/etf/sector-weightings?symbol=SPY&apikey=$KEY"
fetch fund_disclosure_holder  "$BASE/funds/disclosure-holders-latest?symbol=SPY&apikey=$KEY"
fetch fund_disclosure_dates   "$BASE/funds/disclosure-dates?symbol=SPY&apikey=$KEY"
fi

# ─── GOVERNMENT TRADING ──────────────────────────────────────────────────────
if run_section government; then
echo "── government ──"
fetch senate_trade            "$BASE/senate-trades?symbol=AAPL&apikey=$KEY"
fetch house_trade             "$BASE/house-trades?symbol=AAPL&apikey=$KEY"
fi

# ─── FUNDRAISERS ─────────────────────────────────────────────────────────────
if run_section fundraisers; then
echo "── fundraisers ──"
fetch crowdfunding            "$BASE/crowdfunding-offerings-latest?limit=2&apikey=$KEY"
fetch equity_offering         "$BASE/fundraising-latest?limit=2&apikey=$KEY"
fi

# ─── EARNINGS TRANSCRIPT ─────────────────────────────────────────────────────
if run_section transcript; then
echo "── transcript ──"
fetch transcript_latest       "$BASE/earning-call-transcript-latest?limit=2&apikey=$KEY"
fetch transcript              "$BASE/earning-call-transcript?symbol=AAPL&year=2024&quarter=1&apikey=$KEY"
fetch transcript_dates        "$BASE/earning-call-transcript-dates?symbol=AAPL&apikey=$KEY"
fi

# ─── DIRECTORY ───────────────────────────────────────────────────────────────
if run_section directory; then
echo "── directory ──"
fetch available_exchanges     "$BASE/available-exchanges?apikey=$KEY"
fetch available_sectors       "$BASE/available-sectors?apikey=$KEY"
fetch available_industries    "$BASE/available-industries?apikey=$KEY"
fetch available_countries     "$BASE/available-countries?apikey=$KEY"
fetch cik_list                "$BASE/cik-list?limit=2&apikey=$KEY"
fi

# ─── COMMODITY ───────────────────────────────────────────────────────────────
if run_section commodity; then
echo "── commodity ──"
fetch commodity               "$BASE/commodities-list?apikey=$KEY"
fi

echo ""
echo "done — ok=$ok  fail=$fail  skip=$skip"
