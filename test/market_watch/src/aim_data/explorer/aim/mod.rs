#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StockByGics {
    pub stock_code: String,
    pub basic_price: i64,
    pub ceiling_price: i64,
    pub floor_price: i64,
    pub open_price: i64,
    pub close_price: i64,
    pub last_price: i64,
    pub change: i64,
    pub per_change: f64,
    pub reverse_per_change: f64,
    pub total_vol: i64,
    pub total_val: i64,
    pub total_vol_matching: i64,
    pub total_val_matching: i64,
    pub total_vol_put: i64,
    pub total_val_put: i64,
    pub vhtt: f64,
    pub industry_name: String,
    pub sub_industry_name: String,
    pub catid: i32,
    pub stockname: String,
    pub diviend: i64,
    pub foreign_buy_val: i64,
    pub net_foreign_buy_val: i64,
    pub foreign_sell_val: i64,
    pub net_foreign_sell_val: i64,
    pub foreign_buy_vol: i64,
    pub net_foreign_buy_vol: i64,
    pub foreign_sell_vol: i64,
    pub net_foreign_sell_vol: i64,
    pub td_buy_val: i64,
    pub td_sell_val: i64,
    pub td_net_buy_val: i64,
    pub td_net_sell_val: i64,
    pub td_buy_vol: i64,
    pub td_sell_vol: i64,
    pub td_net_buy_vol: i64,
    pub td_net_sell_vol: i64,
    pub t_buy_vol: i64,
    pub outstanding_buy_vol: i64,
    pub net_buy_vol: i64,
    pub t_sell_vol: i64,
    pub outstanding_sell_vol: i64,
    pub net_sell_vol: i64,
    pub revenue_quarter: f64,
    pub revenue_year: f64,
    pub profit_quarter: f64,
    pub profit_year: f64,
    pub total_assets_quarter: f64,
    pub total_assets_year: f64,
    pub owner_equity_quarter: f64,
    pub owner_equity_year: f64,
    pub revenue_4quarter: f64,
    pub profit_4quarter: f64,
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_stock_by_gics() {
        let data = r#"[
            {
                "stock_code": "A32",
                "basic_price": 36900,
                "ceiling_price": 42400,
                "floor_price": 31400,
                "open_price": 0,
                "close_price": 36900,
                "last_price": 36900,
                "change": 0,
                "per_change": 0.0,
                "reverse_per_change": 0.0,
                "total_vol": 0,
                "total_val": 0,
                "total_vol_matching": 0,
                "total_val_matching": 0,
                "total_vol_put": 0,
                "total_val_put": 0,
                "vhtt": 250.92,
                "industry_name": "Tiêu dùng không thiết yếu",
                "sub_industry_name": "",
                "catid": 3,
                "stockname": "CTCP 32",
                "diviend": 2200,
                "foreign_buy_val": 0,
                "net_foreign_buy_val": 0,
                "foreign_sell_val": 0,
                "net_foreign_sell_val": 0,
                "foreign_buy_vol": 0,
                "net_foreign_buy_vol": 0,
                "foreign_sell_vol": 0,
                "net_foreign_sell_vol": 0,
                "td_buy_val": 0,
                "td_sell_val": 0,
                "td_net_buy_val": 0,
                "td_net_sell_val": 0,
                "td_buy_vol": 0,
                "td_sell_vol": 0,
                "td_net_buy_vol": 0,
                "td_net_sell_vol": 0,
                "t_buy_vol": 400,
                "outstanding_buy_vol": 400,
                "net_buy_vol": 0,
                "t_sell_vol": 1200,
                "outstanding_sell_vol": 0,
                "net_sell_vol": 800,
                "revenue_quarter": 0.0,
                "revenue_year": 0.0,
                "profit_quarter": 0.0,
                "profit_year": 0.0,
                "total_assets_quarter": 0.0,
                "total_assets_year": 0.0,
                "owner_equity_quarter": 0.0,
                "owner_equity_year": 0.0,
                "revenue_4quarter": 0.0,
                "profit_4quarter": 0.0
            }
        ]"#;
        let stocks: Vec<StockByGics> = serde_json::from_str(data).unwrap();
        assert_eq!(stocks.len(), 1);
        assert_eq!(stocks[0].stock_code, "A32");
        assert_eq!(stocks[0].basic_price, 36900);
        assert_eq!(stocks[0].industry_name, "Tiêu dùng không thiết yếu");
        assert_eq!(stocks[0].catid, 3);
    }
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExchangeIndex {
    pub exchange: String,
    #[serde(rename = "indexId")]
    pub index_id: String,
    #[serde(rename = "indexValue")]
    pub index_value: f64,
    #[serde(rename = "prevIndexValue")]
    pub prev_index_value: f64,
    pub time: i64,
    pub advances: i32,
    #[serde(rename = "allQty")]
    pub all_qty: i64,
    #[serde(rename = "allValue")]
    pub all_value: i64,
    pub ceiling: i32,
    #[serde(rename = "chartHigh")]
    pub chart_high: f64,
    #[serde(rename = "chartLow")]
    pub chart_low: f64,
    pub declines: i32,
    #[serde(rename = "firstM1Seq")]
    pub first_m1_seq: i32,
    pub floor: i32,
    #[serde(rename = "lastM1Seq")]
    pub last_m1_seq: i32,
    pub nochanges: i32,
    #[serde(rename = "timeMaker")]
    pub time_maker: i64,
    #[serde(rename = "totalQtty")]
    pub total_qtty: i64,
    #[serde(rename = "totalQttyPT")]
    pub total_qtty_pt: i64,
    #[serde(rename = "totalValue")]
    pub total_value: i64,
    #[serde(rename = "totalValuePT")]
    pub total_value_pt: i64,
    pub change: f64,
    #[serde(rename = "changePercent")]
    pub change_percent: f64,
    #[serde(rename = "chartOpen")]
    pub chart_open: f64,
    pub label: String,
    #[serde(rename = "exchangeLabel")]
    pub exchange_label: String,
    #[serde(rename = "totalBuyForeignQtty")]
    pub total_buy_foreign_qtty: i64,
    #[serde(rename = "totalSellForeignQtty")]
    pub total_sell_foreign_qtty: i64,
}
use reqwest::Client;

const TOKEN: &str = "SUPER_SECRET_ADMIN_TOKEN";

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FinanceSheetData {
    pub id: i64,
    pub name: Option<String>,
    pub parent_id: Option<i64>,
    pub expanded: Option<bool>,
    pub level: Option<i32>,
    pub field: Option<String>,
    pub period: Option<String>,
    pub year: Option<i32>,
    pub quarter: Option<i32>,
    pub value: Option<f64>,
    pub symbol: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FinancialData {
    pub symbol: String,
    pub year: i32,
    pub quarter: i32,
    pub company_type: String,
    pub icb_code: Option<String>,
    pub icb_name: Option<String>,
    pub financial_values: FinancialDetail,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InstitutionData {
    pub institution_id: i64,
    pub symbol: String,
    pub icb_code: Option<String>,
    pub company_name: String,
    pub short_name: String,
    pub international_name: String,
    pub head_quarters: String,
    pub phone: String,
    pub fax: Option<String>,
    pub email: String,
    pub web_address: Option<String>,
    pub overview: String,
    pub history: String,
    pub business_areas: String,
    pub employees: i32,
    pub branches: Option<String>,
    pub establishment_date: Option<String>,
    pub business_license_number: String,
    pub date_of_issue: String,
    pub tax_id_number: String,
    pub charter_capital: f64,
    pub date_of_listing: Option<String>,
    pub exchange: Option<String>,
    pub initial_listing_price: Option<f64>,
    pub listing_volume: Option<f64>,
    pub state_ownership: f64,
    pub foreign_ownership: f64,
    pub other_ownership: f64,
    pub is_listed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Subsidiary {
    pub institution_id: i64,
    pub father_symbol: String,
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub company_name: String,
    pub short_name: Option<String>,
    pub international_name: String,
    pub company_profile: Option<String>,
    pub r#type: i32,
    pub ownership: f64,
    pub shares: f64,
    pub is_listed: bool,
    pub charter_capital: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SharedHolder {
    pub id: i64,
    pub ticker: String,
    pub majorholderid: Option<i64>,
    pub individualholderid: Option<i64>,
    pub institutionholderid: Option<i64>,
    pub institutionholdersymbol: Option<String>,
    pub institutionholderexchange: Option<String>,
    pub name: String,
    pub position: Option<String>,
    pub shares: f64,
    pub ownership: f64,
    pub isorganization: bool,
    pub isforeigner: bool,
    pub isfounder: bool,
    pub reported_at: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FinancialDetail {
    #[serde(rename = "TotalAsset")]
    pub total_asset: Option<i64>,
    #[serde(rename = "TotalDebt")]
    pub total_debt: Option<i64>,
    #[serde(rename = "NetSale")]
    pub net_sale: Option<i64>,
    #[serde(rename = "GrossProfit")]
    pub gross_profit: Option<i64>,
    #[serde(rename = "ProfitAfterTax")]
    pub profit_after_tax: Option<i64>,
    #[serde(rename = "PB")]
    pub pb: Option<f64>,
    #[serde(rename = "PE")]
    pub pe: Option<f64>,
    #[serde(rename = "PS")]
    pub ps: Option<f64>,
    #[serde(rename = "BasicEPS")]
    pub basic_eps: Option<f64>,
    #[serde(rename = "BookValuePerShare")]
    pub book_value_per_share: Option<f64>,
    #[serde(rename = "DividendYield")]
    pub dividend_yield: Option<f64>,
    #[serde(rename = "ROA")]
    pub roa: Option<f64>,
    #[serde(rename = "ROE")]
    pub roe: Option<f64>,
    #[serde(rename = "GrossMargin")]
    pub gross_margin: Option<f64>,
    #[serde(rename = "OperatingMargin")]
    pub operating_margin: Option<f64>,
    #[serde(rename = "CurrentAssetGrowth_QoQ")]
    pub current_asset_growth_qoq: Option<f64>,
    #[serde(rename = "SaleGrowth")]
    pub sale_growth: Option<f64>,
    #[serde(rename = "BasicEPSGrowth")]
    pub basic_eps_growth: Option<f64>,
    #[serde(rename = "PlanningProfitAfterTax")]
    pub planning_profit_after_tax: Option<i64>,
    #[serde(rename = "PlanningProfitBeforeTax")]
    pub planning_profit_before_tax: Option<i64>,
    #[serde(rename = "PlanningEPS")]
    pub planning_eps: Option<f64>,
    #[serde(rename = "PlanningCashDividend")]
    pub planning_cash_dividend: Option<f64>,
    #[serde(rename = "EBITDA")]
    pub ebitda: Option<i64>,
    #[serde(rename = "EVOverEBITDA")]
    pub ev_over_ebitda: Option<f64>,
    #[serde(rename = "TotalInventory")]
    pub total_inventory: Option<i64>,
    #[serde(rename = "SectorROIC")]
    pub sector_roic: Option<f64>,
    #[serde(rename = "SectorROCE")]
    pub sector_roce: Option<f64>,
    #[serde(rename = "PreTaxMargin")]
    pub pre_tax_margin: Option<f64>,
    #[serde(rename = "PlanningRevenue")]
    pub planning_revenue: Option<f64>,
    #[serde(rename = "PiotroskiFScore")]
    pub piotroski_f_score: Option<i64>,
    #[serde(rename = "ManufacturingZScore")]
    pub manufacturing_z_score: Option<f64>,
    // #[serde(rename = "PlanningStockDividend")]
    // pub planning_stock_dividend: Option<f64>,
    // #[serde(rename = "ROS")]
    // pub ros: Option<f64>,
    // #[serde(rename = "AccumulatedUndistributedProfitLoss")]
    // pub accumulated_undistributed_profit_loss: Option<i64>,
    // #[serde(rename = "AvgCustomerLoan")]
    // pub avg_customer_loan: Option<i64>,
    // #[serde(rename = "AvgFixedAsset")]
    // pub avg_fixed_asset: Option<i64>,
    // #[serde(rename = "AvgMarketCapInPeriod")]
    // pub avg_market_cap_in_period: Option<i64>,
    // #[serde(rename = "AvgPriceInPeriod")]
    // pub avg_price_in_period: Option<f64>,
    // #[serde(rename = "AvgShareInPeriod")]
    // pub avg_share_in_period: Option<i64>,
    // #[serde(rename = "AvgTotalAccountReceivable")]
    // pub avg_total_account_receivable: Option<i64>,
    // #[serde(rename = "AvgTotalAsset")]
    // pub avg_total_asset: Option<i64>,
    // #[serde(rename = "AvgTotalCapital")]
    // pub avg_total_capital: Option<i64>,
    // #[serde(rename = "AvgTotalEquity")]
    // pub avg_total_equity: Option<i64>,
    // #[serde(rename = "AvgTotalInterestBearingDebt")]
    // pub avg_total_interest_bearing_debt: Option<i64>,
    // #[serde(rename = "AvgTotalInterestEarningAsset")]
    // pub avg_total_interest_earning_asset: Option<i64>,
    // #[serde(rename = "BadDebt")]
    // pub bad_debt: Option<i64>,
    // #[serde(rename = "BasicEPSGrowth_03Yr")]
    // pub basic_eps_growth_03yr: Option<f64>,
    // #[serde(rename = "BasicEPSGrowth_LFY")]
    // pub basic_eps_growth_lfy: Option<f64>,
    // #[serde(rename = "BasicEPSGrowth_TTM")]
    // pub basic_eps_growth_ttm: Option<f64>,
    // #[serde(rename = "BorrowingFromOtherCreditInstitution")]
    // pub borrowing_from_other_credit_institution: Option<i64>,
    // #[serde(rename = "CAPEX")]
    // pub capex: Option<i64>,
    // #[serde(rename = "CAR")]
    // pub car: Option<f64>,
    // #[serde(rename = "CIR")]
    // pub cir: Option<f64>,
    // #[serde(rename = "CLR")]
    // pub clr: Option<f64>,
    // #[serde(rename = "COF")]
    // pub cof: Option<f64>,
    // #[serde(rename = "CapitalGainsYield")]
    // pub capital_gains_yield: Option<f64>,
    // #[serde(rename = "CashAndCashEquivalentAtTheBeginningOfPeriod")]
    // pub cash_and_cash_equivalent_at_the_beginning_of_period: Option<i64>,
    // #[serde(rename = "CashAndCashEquivalentAtTheEndOfPeriod")]
    // pub cash_and_cash_equivalent_at_the_end_of_period: Option<i64>,
    // #[serde(rename = "CashDividend")]
    // pub cash_dividend: Option<f64>,
    // #[serde(rename = "CashGoldJewelry")]
    // pub cash_gold_jewelry: Option<i64>,
    // #[serde(rename = "CashflowFromFinancingActivity")]
    // pub cashflow_from_financing_activity: Option<i64>,
    // #[serde(rename = "CashflowFromInvestingActivity")]
    // pub cashflow_from_investing_activity: Option<i64>,
    // #[serde(rename = "CashflowFromOperatingActivity")]
    // pub cashflow_from_operating_activity: Option<i64>,
    // #[serde(rename = "CharterCapital")]
    // pub charter_capital: Option<i64>,
    // #[serde(rename = "CompanyType")]
    // pub company_type: Option<String>,
    // #[serde(rename = "ConstructionInvestmentCapital")]
    // pub construction_investment_capital: Option<i64>,
    // #[serde(rename = "ContributedCapitalOfShareHolder")]
    // pub contributed_capital_of_share_holder: Option<i64>,
    // #[serde(rename = "CorporateIncomeTax")]
    // pub corporate_income_tax: Option<i64>,
    // #[serde(rename = "CostToAsset")]
    // pub cost_to_asset: Option<f64>,
    // #[serde(rename = "CreditLossProvision_CUM")]
    // pub credit_loss_provision_cum: Option<i64>,
    // #[serde(rename = "CreditLossProvision_TTM")]
    // pub credit_loss_provision_ttm: Option<i64>,
    // #[serde(rename = "CreditRiskProvisionsExpense")]
    // pub credit_risk_provisions_expense: Option<i64>,
    // #[serde(rename = "CurrentAssetGrowth")]
    // pub current_asset_growth: Option<f64>,
    // #[serde(rename = "CurrentAssetGrowth_03Yr")]
    // pub current_asset_growth_03yr: Option<f64>,
    // #[serde(rename = "CurrentAssetGrowth_LFY")]
    // pub current_asset_growth_lfy: Option<f64>,
    // #[serde(rename = "CurrentAssetGrowth_YoY")]
    // pub current_asset_growth_yoy: Option<f64>,
    // #[serde(rename = "CustomerLoan")]
    // pub customer_loan: Option<i64>,
    // #[serde(rename = "CustomerLoanAfterProvision")]
    // pub customer_loan_after_provision: Option<i64>,
    // #[serde(rename = "DebtToGovernmentAndStateBank")]
    // pub debt_to_government_and_state_bank: Option<i64>,
    // #[serde(rename = "DepositAndBorrowingFromOtherCreditInstitution")]
    // pub deposit_and_borrowing_from_other_credit_institution: Option<i64>,
    // #[serde(rename = "DepositAtAndLoanToOtherCreditInstitution")]
    // pub deposit_at_and_loan_to_other_credit_institution: Option<i64>,
    // #[serde(rename = "DepositAtCreditInstitution")]
    // pub deposit_at_credit_institution: Option<i64>,
    // #[serde(rename = "DepositAtOtherCreditInstitution")]
    // pub deposit_at_other_credit_institution: Option<i64>,
    // #[serde(rename = "DepositAtStateBank")]
    // pub deposit_at_state_bank: Option<i64>,
    // #[serde(rename = "DepositFromOtherCreditInstitution")]
    // pub deposit_from_other_credit_institution: Option<i64>,
    // #[serde(rename = "DepositOfCustomer")]
    // pub deposit_of_customer: Option<i64>,
    // #[serde(rename = "DepreciationAndAmortization")]
    // pub depreciation_and_amortization: Option<i64>,
    // #[serde(rename = "DerivativeFinancialInstrumentAndOtherFinancialDebt")]
    // pub derivative_financial_instrument_and_other_financial_debt: Option<i64>,
    // #[serde(rename = "DerivativeFinancialInstrumentsAndOtherFinancialAsset")]
    // pub derivative_financial_instruments_and_other_financial_asset: Option<i64>,
    // #[serde(rename = "DilutedEPS")]
    // pub diluted_eps: Option<f64>,
    // #[serde(rename = "DilutedEPSGrowth")]
    // pub diluted_eps_growth: Option<f64>,
    // #[serde(rename = "DilutedEPSGrowth_03Yr")]
    // pub diluted_eps_growth_03yr: Option<f64>,
    // #[serde(rename = "DilutedEPSGrowth_LFY")]
    // pub diluted_eps_growth_lfy: Option<f64>,
    // #[serde(rename = "DilutedEPSGrowth_TTM")]
    // pub diluted_eps_growth_ttm: Option<f64>,
    // #[serde(rename = "DoubtfulDebt")]
    // pub doubtful_debt: Option<i64>,
    // #[serde(rename = "EffectOfForeignExchangeDifference")]
    // pub effect_of_foreign_exchange_difference: Option<i64>,
    // #[serde(rename = "EquityAttributableToNonControllingShareholder")]
    // pub equity_attributable_to_non_controlling_shareholder: Option<i64>,
    // #[serde(rename = "EquityGrowth")]
    // pub equity_growth: Option<f64>,
    // #[serde(rename = "EquityGrowth_03Yr")]
    // pub equity_growth_03yr: Option<f64>,
    // #[serde(rename = "EquityGrowth_LFY")]
    // pub equity_growth_lfy: Option<f64>,
    // #[serde(rename = "EquityGrowth_QoQ")]
    // pub equity_growth_qoq: Option<f64>,
    // #[serde(rename = "EquityGrowth_YoY")]
    // pub equity_growth_yoy: Option<f64>,
    // #[serde(rename = "EquitySurplus")]
    // pub equity_surplus: Option<i64>,
    // #[serde(rename = "EquityToAsset")]
    // pub equity_to_asset: Option<f64>,
    // #[serde(rename = "EquityToDeposit")]
    // pub equity_to_deposit: Option<f64>,
    // #[serde(rename = "EquityToLoan")]
    // pub equity_to_loan: Option<f64>,
    // #[serde(rename = "ExchangeDifference")]
    // pub exchange_difference: Option<i64>,
    // #[serde(rename = "FinancingCapitalInvestmentTrustRiskBearingLoan")]
    // pub financing_capital_investment_trust_risk_bearing_loan: Option<i64>,
    // #[serde(rename = "FixedAsset")]
    // pub fixed_asset: Option<i64>,
    // #[serde(rename = "FixedAssetGrowth")]
    // pub fixed_asset_growth: Option<f64>,
    // #[serde(rename = "FixedAssetGrowth_03Yr")]
    // pub fixed_asset_growth_03yr: Option<f64>,
    // #[serde(rename = "FixedAssetGrowth_LFY")]
    // pub fixed_asset_growth_lfy: Option<f64>,
    // #[serde(rename = "FixedAssetGrowth_QoQ")]
    // pub fixed_asset_growth_qoq: Option<f64>,
    // #[serde(rename = "FixedAssetGrowth_YoY")]
    // pub fixed_asset_growth_yoy: Option<f64>,
    // #[serde(rename = "FixedAssetTurnover")]
    // pub fixed_asset_turnover: Option<f64>,
    // #[serde(rename = "FundOfFinancialInstitution")]
    // pub fund_of_financial_institution: Option<i64>,
    // #[serde(rename = "GrossProfit_CUM")]
    // pub gross_profit_cum: Option<i64>,
    // #[serde(rename = "GrossProfit_TTM")]
    // pub gross_profit_ttm: Option<i64>,
    // #[serde(rename = "ICBCode")]
    // pub icb_code: Option<String>,
    // #[serde(rename = "ICBName")]
    // pub icb_name: Option<String>,
    // #[serde(rename = "InTangibleAsset")]
    // pub in_tangible_asset: Option<i64>,
    // #[serde(rename = "IncomeFromAssociatedCapital")]
    // pub income_from_associated_capital: Option<i64>,
    // #[serde(rename = "InterestAndSimilarExpense")]
    // pub interest_and_similar_expense: Option<i64>,
    // #[serde(rename = "InterestAndSimilarIncome")]
    // pub interest_and_similar_income: Option<i64>,
    // #[serde(rename = "InterestAndSimilarIncome_CUM")]
    // pub interest_and_similar_income_cum: Option<i64>,
    // #[serde(rename = "InterestAndSimilarIncome_TTM")]
    // pub interest_and_similar_income_ttm: Option<i64>,
    // #[serde(rename = "InvestmentCapitalOfShareHolder")]
    // pub investment_capital_of_share_holder: Option<i64>,
    // #[serde(rename = "InvestmentRealEstate")]
    // pub investment_real_estate: Option<i64>,
    // #[serde(rename = "InvestmentSecurities")]
    // pub investment_securities: Option<i64>,
    // #[serde(rename = "InvestmentSecuritiesAvailableForSale")]
    // pub investment_securities_available_for_sale: Option<i64>,
    // #[serde(rename = "InvestmentSecuritiesHeldToMaturity")]
    // pub investment_securities_held_to_maturity: Option<i64>,
    // #[serde(rename = "IssuingValuablePaper")]
    // pub issuing_valuable_paper: Option<i64>,
    // #[serde(rename = "LAR")]
    // pub lar: Option<f64>,
    // #[serde(rename = "LDR")]
    // pub ldr: Option<f64>,
    // #[serde(rename = "LoanToOtherCreditInstitution")]
    // pub loan_to_other_credit_institution: Option<i64>,
    // #[serde(rename = "LoanlossReservesToLoan")]
    // pub loanloss_reserves_to_loan: Option<f64>,
    // #[serde(rename = "LoanlossReservesToNPL")]
    // pub loanloss_reserves_to_npl: Option<f64>,
    // #[serde(rename = "LongTermInvestmentCapitalContribution")]
    // pub long_term_investment_capital_contribution: Option<i64>,
    // #[serde(rename = "MarketCapAtPeriodEnd")]
    // pub market_cap_at_period_end: Option<i64>,
    // #[serde(rename = "NIM")]
    // pub nim: Option<f64>,
    // #[serde(rename = "NPLToLoan")]
    // pub npl_to_loan: Option<f64>,
    // #[serde(rename = "NetInterestIncome")]
    // pub net_interest_income: Option<i64>,
    // #[serde(rename = "NetInterestIncome_CUM")]
    // pub net_interest_income_cum: Option<i64>,
    // #[serde(rename = "NetInterestIncome_TTM")]
    // pub net_interest_income_ttm: Option<i64>,
    // #[serde(rename = "NetProfitFromOperatingActivity")]
    // pub net_profit_from_operating_activity: Option<i64>,
    // #[serde(rename = "NetProfitFromOperatingActivity_CUM")]
    // pub net_profit_from_operating_activity_cum: Option<i64>,
    // #[serde(rename = "NetProfitFromOperatingActivity_TTM")]
    // pub net_profit_from_operating_activity_ttm: Option<i64>,
    // #[serde(rename = "NetProfitFromServiceActivity")]
    // pub net_profit_from_service_activity: Option<i64>,
    // #[serde(rename = "NetProfitFromTradingActivitiesInForeignExchangeAndGold")]
    // pub net_profit_from_trading_activities_in_foreign_exchange_and_gold: Option<i64>,
    // #[serde(rename = "NetProfitFromTradingActivityInForeignExchangeAndGold")]
    // pub net_profit_from_trading_activity_in_foreign_exchange_and_gold: Option<i64>,
    // #[serde(rename = "NetProfitFromTradingOfInvestmentSecurities")]
    // pub net_profit_from_trading_of_investment_securities: Option<i64>,
    // #[serde(rename = "NetProfitFromTradingOfTradingSecurities")]
    // pub net_profit_from_trading_of_trading_securities: Option<i64>,
    // #[serde(rename = "NetSale_CUM")]
    // pub net_sale_cum: Option<i64>,
    // #[serde(rename = "NetSale_TTM")]
    // pub net_sale_ttm: Option<i64>,
    // #[serde(rename = "NonControllingShareholderProfitAfterTax")]
    // pub non_controlling_shareholder_profit_after_tax: Option<i64>,
    // #[serde(rename = "OtherAsset")]
    // pub other_asset: Option<i64>,
    // #[serde(rename = "OtherDebt")]
    // pub other_debt: Option<i64>,
    // #[serde(rename = "OtherEquity")]
    // pub other_equity: Option<i64>,
    // #[serde(rename = "OtherFund")]
    // pub other_fund: Option<i64>,
    // #[serde(rename = "OtherNetProfit")]
    // pub other_net_profit: Option<i64>,
    // #[serde(rename = "PaidInCapital")]
    // pub paid_in_capital: Option<i64>,
    // #[serde(rename = "ParentCompanyShareholderProfitAfterTax")]
    // pub parent_company_shareholder_profit_after_tax: Option<i64>,
    // #[serde(rename = "ParentCompanyShareholderProfitAfterTaxGrowth")]
    // pub parent_company_shareholder_profit_after_tax_growth: Option<f64>,
    // #[serde(rename = "ParentCompanyShareholderProfitAfterTax_CUM")]
    // pub parent_company_shareholder_profit_after_tax_cum: Option<i64>,
    // #[serde(rename = "PayoutRatio")]
    // pub payout_ratio: Option<f64>,
    // #[serde(rename = "PreferedStock")]
    // pub prefered_stock: Option<i64>,
    // #[serde(rename = "PriceAtPeriodEnd")]
    // pub price_at_period_end: Option<i64>,
    // #[serde(rename = "ProfitAfterIncomeTaxOfBank")]
    // pub profit_after_income_tax_of_bank: Option<i64>,
    // #[serde(rename = "ProfitAfterIncomeTax_CUM")]
    // pub profit_after_income_tax_cum: Option<i64>,
    // #[serde(rename = "ProfitAfterTaxGrowth")]
    // pub profit_after_tax_growth: Option<f64>,
    // #[serde(rename = "ProfitAfterTaxGrowth_03Yr")]
    // pub profit_after_tax_growth_03yr: Option<f64>,
    // #[serde(rename = "ProfitAfterTaxGrowth_LFY")]
    // pub profit_after_tax_growth_lfy: Option<f64>,
    // #[serde(rename = "ProfitAfterTaxGrowth_MRQ")]
    // pub profit_after_tax_growth_mrq: Option<f64>,
    // #[serde(rename = "ProfitAfterTaxGrowth_MRQ2")]
    // pub profit_after_tax_growth_mrq2: Option<f64>,
    // #[serde(rename = "ProfitAfterTaxGrowth_TTM")]
    // pub profit_after_tax_growth_ttm: Option<f64>,
    // #[serde(rename = "ProfitAfterTax_CUM")]
    // pub profit_after_tax_cum: Option<i64>,
    // #[serde(rename = "ProfitAfterTax_TTM")]
    // pub profit_after_tax_ttm: Option<i64>,
    // #[serde(rename = "ProfitBeforeIncomeTax")]
    // pub profit_before_income_tax: Option<i64>,
    // #[serde(rename = "ProfitBeforeTax")]
    // pub profit_before_tax: Option<i64>,
    // #[serde(rename = "ProfitBeforeTaxGrowth")]
    // pub profit_before_tax_growth: Option<f64>,
    // #[serde(rename = "ProfitBeforeTaxGrowth_03Yr")]
    // pub profit_before_tax_growth_03yr: Option<f64>,
    // #[serde(rename = "ProfitBeforeTaxGrowth_LFY")]
    // pub profit_before_tax_growth_lfy: Option<f64>,
    // #[serde(rename = "ProfitBeforeTaxGrowth_MRQ")]
    // pub profit_before_tax_growth_mrq: Option<f64>,
    // #[serde(rename = "ProfitBeforeTaxGrowth_MRQ2")]
    // pub profit_before_tax_growth_mrq2: Option<f64>,
    // #[serde(rename = "ProfitBeforeTaxGrowth_TTM")]
    // pub profit_before_tax_growth_ttm: Option<f64>,
    // #[serde(rename = "ProfitGrowth")]
    // pub profit_growth: Option<f64>,
    // #[serde(rename = "ProfitGrowth_03Yr")]
    // pub profit_growth_03yr: Option<f64>,
    // #[serde(rename = "ProfitGrowth_LFY")]
    // pub profit_growth_lfy: Option<f64>,
    // #[serde(rename = "ProfitGrowth_MRQ")]
    // pub profit_growth_mrq: Option<f64>,
    // #[serde(rename = "ProfitGrowth_MRQ2")]
    // pub profit_growth_mrq2: Option<f64>,
    // #[serde(rename = "ProfitGrowth_TTM")]
    // pub profit_growth_ttm: Option<f64>,
    // #[serde(rename = "PropertyRevaluationDifference")]
    // pub property_revaluation_difference: Option<i64>,
    // #[serde(rename = "ProvisionChargesToLoan")]
    // pub provision_charges_to_loan: Option<f64>,
    // #[serde(rename = "ProvisionForCustomerLoanLoss")]
    // pub provision_for_customer_loan_loss: Option<i64>,
    // #[serde(rename = "Quarter")]
    // pub quarter: Option<i32>,
    // #[serde(rename = "ReceivableTurnover")]
    // pub receivable_turnover: Option<f64>,
    // #[serde(rename = "RetentionRatio")]
    // pub retention_ratio: Option<f64>,
    // #[serde(rename = "SaleGrowth_03Yr")]
    // pub sale_growth_03yr: Option<f64>,
    // #[serde(rename = "SaleGrowth_LFY")]
    // pub sale_growth_lfy: Option<f64>,
    // #[serde(rename = "SalePerShare")]
    // pub sale_per_share: Option<f64>,
    // #[serde(rename = "SalesGrowth_MRQ")]
    // pub sales_growth_mrq: Option<f64>,
    // #[serde(rename = "SalesGrowth_MRQ2")]
    // pub sales_growth_mrq2: Option<f64>,
    // #[serde(rename = "SalesGrowth_TTM")]
    // pub sales_growth_ttm: Option<f64>,
    // #[serde(rename = "SectorAvgEPS")]
    // pub sector_avg_eps: Option<f64>,
    // #[serde(rename = "SectorBadDebt")]
    // pub sector_bad_debt: Option<i64>,
    // #[serde(rename = "SectorBorrowingFromOtherCreditInstitution")]
    // pub sector_borrowing_from_other_credit_institution: Option<i64>,
    // #[serde(rename = "SectorCAR")]
    // pub sector_car: Option<f64>,
    // #[serde(rename = "SectorCIR")]
    // pub sector_cir: Option<f64>,
    // #[serde(rename = "SectorCLR")]
    // pub sector_clr: Option<f64>,
    // #[serde(rename = "SectorCOF")]
    // pub sector_cof: Option<f64>,
    // #[serde(rename = "SectorCharterCapital")]
    // pub sector_charter_capital: Option<i64>,
    // #[serde(rename = "SectorCostToAsset")]
    // pub sector_cost_to_asset: Option<f64>,
    // #[serde(rename = "SectorCreditRiskProvisionsExpense")]
    // pub sector_credit_risk_provisions_expense: Option<i64>,
    // #[serde(rename = "SectorCustomerLoan")]
    // pub sector_customer_loan: Option<i64>,
    // #[serde(rename = "SectorCustomerLoanAfterProvision")]
    // pub sector_customer_loan_after_provision: Option<i64>,
    // #[serde(rename = "SectorDebtToGovernmentAndStateBank")]
    // pub sector_debt_to_government_and_state_bank: Option<i64>,
    // #[serde(rename = "SectorDepositAndBorrowingFromOtherCreditInstitution")]
    // pub sector_deposit_and_borrowing_from_other_credit_institution: Option<i64>,
    // #[serde(rename = "SectorDepositAtAndLoanToOtherCreditInstitution")]
    // pub sector_deposit_at_and_loan_to_other_credit_institution: Option<i64>,
    // #[serde(rename = "SectorDepositAtCreditInstitution")]
    // pub sector_deposit_at_credit_institution: Option<i64>,
    // #[serde(rename = "SectorDepositAtStateBank")]
    // pub sector_deposit_at_state_bank: Option<i64>,
    // #[serde(rename = "SectorDepositFromOtherCreditInstitution")]
    // pub sector_deposit_from_other_credit_institution: Option<i64>,
    // #[serde(rename = "SectorDepositOfCustomer")]
    // pub sector_deposit_of_customer: Option<i64>,
    // #[serde(rename = "SectorDoubtfulDebt")]
    // pub sector_doubtful_debt: Option<i64>,
    // #[serde(rename = "SectorEPS")]
    // pub sector_eps: Option<f64>,
    // #[serde(rename = "SectorEquityToLoan")]
    // pub sector_equity_to_loan: Option<f64>,
    // #[serde(rename = "SectorFixedAsset")]
    // pub sector_fixed_asset: Option<i64>,
    // #[serde(rename = "SectorInTangibleAsset")]
    // pub sector_in_tangible_asset: Option<i64>,
    // #[serde(rename = "SectorInterestAndSimilarExpense")]
    // pub sector_interest_and_similar_expense: Option<i64>,
    // #[serde(rename = "SectorInterestAndSimilarIncome")]
    // pub sector_interest_and_similar_income: Option<i64>,
    // #[serde(rename = "SectorInvestmentSecurities")]
    // pub sector_investment_securities: Option<i64>,
    // #[serde(rename = "SectorInvestmentSecuritiesAvailableForSale")]
    // pub sector_investment_securities_available_for_sale: Option<i64>,
    // #[serde(rename = "SectorInvestmentSecuritiesHeldToMaturity")]
    // pub sector_investment_scurities_held_to_maturity: Option<i64>,
    // #[serde(rename = "SectorIssuingValuablePaper")]
    // pub sector_issuing_valuable_paper: Option<i64>,
    // #[serde(rename = "SectorLAR")]
    // pub sector_lar: Option<f64>,
    // #[serde(rename = "SectorLDR")]
    // pub sector_ldr: Option<f64>,
    // #[serde(rename = "SectorLoanToOtherCreditInstitution")]
    // pub sector_loan_to_other_credit_institution: Option<i64>,
    // #[serde(rename = "SectorLoanlossReserveToLoan")]
    // pub sector_loanloss_reserve_to_loan: Option<f64>,
    // #[serde(rename = "SectorLoanlossReserveToNPL")]
    // pub sector_loanloss_reserve_to_npl: Option<f64>,
    // #[serde(rename = "SectorLoanlossReservesToLoan")]
    // pub sector_loanloss_reserves_to_loan: Option<f64>,
    // #[serde(rename = "SectorLoanlossReservesToNPL")]
    // pub sector_loanloss_reserves_to_npl: Option<f64>,
    // #[serde(rename = "SectorNIM")]
    // pub sector_nim: Option<f64>,
    // #[serde(rename = "SectorNPL")]
    // pub sector_npl: Option<i64>,
    // #[serde(rename = "SectorNPLToLoan")]
    // pub sector_npl_to_loan: Option<f64>,
    // #[serde(rename = "SectorNetInterestIncome")]
    // pub sector_net_interest_income: Option<i64>,
    // #[serde(rename = "SectorNetProfitFromOperatingActivity")]
    // pub sector_net_profit_from_operating_activity: Option<i64>,
    // #[serde(rename = "SectorNetProfitFromServiceActivity")]
    // pub sector_net_profit_from_service_activity: Option<i64>,
    // #[serde(rename = "SectorNetProfitFromTradingActivityInForeignExchangeAndGold")]
    // pub sector_net_profit_from_trading_activity_in_foreign_exchange_and_gold: Option<i64>,
    // #[serde(rename = "SectorNetProfitFromTradingOfInvestmentSecurities")]
    // pub sector_net_profit_from_trading_of_investment_securities: Option<i64>,
    // #[serde(rename = "SectorNetProfitFromTradingOfTradingSecurities")]
    // pub sector_net_profit_from_trading_of_trading_securities: Option<i64>,
    // #[serde(rename = "SectorNetSale")]
    // pub sector_net_sale: Option<i64>,
    // #[serde(rename = "SectorOtherNetProfit")]
    // pub sector_other_net_profit: Option<i64>,
    // #[serde(rename = "SectorPB")]
    // pub sector_pb: Option<f64>,
    // #[serde(rename = "SectorPE")]
    // pub sector_pe: Option<f64>,
    // #[serde(rename = "SectorPS")]
    // pub sector_ps: Option<f64>,
    // #[serde(rename = "SectorPaidInCapital")]
    // pub sector_paid_in_capital: Option<i64>,
    // #[serde(rename = "SectorParentCompanyShareholderProfitAfterTax")]
    // pub sector_parent_company_shareholder_profit_after_tax: Option<i64>,
    // #[serde(rename = "SectorProfitAfterTax")]
    // pub sector_profit_after_tax: Option<i64>,
    // #[serde(rename = "SectorProfitBeforeTax")]
    // pub sector_profit_before_tax: Option<i64>,
    // #[serde(rename = "SectorProvisionChargeToLoan")]
    // pub sector_provision_charge_to_loan: Option<f64>,
    // #[serde(rename = "SectorProvisionChargesToLoan")]
    // pub sector_provision_charges_to_loan: Option<f64>,
    // #[serde(rename = "SectorProvisionForCustomerLoanLoss")]
    // pub sector_provision_for_customer_loan_loss: Option<i64>,
    // #[serde(rename = "SectorROA")]
    // pub sector_roa: Option<f64>,
    // #[serde(rename = "SectorROE")]
    // pub sector_roe: Option<f64>,
    // #[serde(rename = "SectorROS")]
    // pub sector_ros: Option<f64>,
    // #[serde(rename = "SectorStandardDebt")]
    // pub sector_standard_debt: Option<i64>,
    // #[serde(rename = "SectorStockHolderEquity")]
    // pub sector_stock_holder_equity: Option<i64>,
    // #[serde(rename = "SectorSubstandardDebt")]
    // pub sector_substandard_debt: Option<i64>,
    // #[serde(rename = "SectorTangiblePB")]
    // pub sector_tangible_pb: Option<f64>,
    // #[serde(rename = "SectorTotalAsset")]
    // pub sector_total_asset: Option<i64>,
    // #[serde(rename = "SectorTotalInterestBearingDebt")]
    // pub sector_total_interest_bearing_debt: Option<i64>,
    // #[serde(rename = "SectorTotalInterestEarningAsset")]
    // pub sector_total_interest_earning_asset: Option<i64>,
    // #[serde(rename = "SectorTotalOperatingExpense")]
    // pub sector_total_operating_expense: Option<i64>,
    // #[serde(rename = "SectorTotalStockHolderEquity")]
    // pub sector_total_stock_holder_equity: Option<i64>,
    // #[serde(rename = "SectorWatchlistDebt")]
    // pub sector_watchlist_debt: Option<i64>,
    // #[serde(rename = "SectorYOEA")]
    // pub sector_yoea: Option<f64>,
    // #[serde(rename = "ShareAtPeriodEnd")]
    // pub share_at_period_end: Option<i64>,
    // #[serde(rename = "StandardDebt")]
    // pub standard_debt: Option<i64>,
    // #[serde(rename = "StockDividend")]
    // pub stock_dividend: Option<f64>,
    // #[serde(rename = "StockHolderEquity")]
    // pub stock_holder_equity: Option<i64>,
    // #[serde(rename = "SubstandardDebt")]
    // pub substandard_debt: Option<i64>,
    // #[serde(rename = "TangibleBookValuePerShare")]
    // pub tangible_book_value_per_share: Option<f64>,
    // #[serde(rename = "TangibleTPB")]
    // pub tangible_tpb: Option<f64>,
    // #[serde(rename = "TotalAssetGrowth")]
    // pub total_asset_growth: Option<f64>,
    // #[serde(rename = "TotalAssetGrowth_03Yr")]
    // pub total_asset_growth_03yr: Option<f64>,
    // #[serde(rename = "TotalAssetGrowth_LFY")]
    // pub total_asset_growth_lfy: Option<f64>,
    // #[serde(rename = "TotalAssetGrowth_QoQ")]
    // pub total_asset_growth_qoq: Option<f64>,
    // #[serde(rename = "TotalAssetGrowth_YoY")]
    // pub total_asset_growth_yoy: Option<f64>,
    // #[serde(rename = "TotalAssetTurnover")]
    // pub total_asset_turnover: Option<f64>,
    // #[serde(rename = "TotalDebtAndEquity")]
    // pub total_debt_and_equity: Option<i64>,
    // #[serde(rename = "TotalEquity")]
    // pub total_equity: Option<i64>,
    // #[serde(rename = "TotalInterestBearingDebt")]
    // pub total_interest_bearing_debt: Option<i64>,
    // #[serde(rename = "TotalInterestEarningAsset")]
    // pub total_interest_earning_asset: Option<i64>,
    // #[serde(rename = "TotalNonInterestBearingDebt")]
    // pub total_non_interest_bearing_debt: Option<i64>,
    // #[serde(rename = "TotalNonInterestEarningAsset")]
    // pub total_non_interest_earning_asset: Option<i64>,
    // #[serde(rename = "TotalOperatingExpense")]
    // pub total_operating_expense: Option<i64>,
    // #[serde(rename = "TotalOperatingExpense_CUM")]
    // pub total_operating_expense_cum: Option<i64>,
    // #[serde(rename = "TotalOperatingExpense_TTM")]
    // pub total_operating_expense_ttm: Option<i64>,
    // #[serde(rename = "TotalOperatingIncome")]
    // pub total_operating_income: Option<i64>,
    // #[serde(rename = "TotalOperatingIncome_CUM")]
    // pub total_operating_income_cum: Option<i64>,
    // #[serde(rename = "TotalOperatingIncome_TTM")]
    // pub total_operating_income_ttm: Option<i64>,
    // #[serde(rename = "TotalRevenue")]
    // pub total_revenue: Option<i64>,
    // #[serde(rename = "TotalStockHolderEquity")]
    // pub total_stock_holder_equity: Option<i64>,
    // #[serde(rename = "TotalStockReturn")]
    // pub total_stock_return: Option<f64>,
    // #[serde(rename = "TradingSecurities")]
    // pub trading_securities: Option<i64>,
    // #[serde(rename = "WatchlistDebt")]
    // pub watchlist_debt: Option<i64>,
    // #[serde(rename = "YOEA")]
    // pub yoea: Option<f64>,
    // #[serde(rename = "Year")]
    // pub year: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Officer {
    pub officer_id: i64,
    pub symbol: String,
    pub individual_id: i64,
    pub name: String,
    pub position_id: i64,
    pub position: String,
    pub is_foreigner: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InsiderTransaction {
    pub transaction_id: i64,
    pub major_holder_id: i64,
    pub individual_holder_id: Option<i64>,
    pub institution_holder_id: Option<i64>,
    pub institution_holder_symbol: Option<String>,
    pub institution_holder_exchange: Option<String>,
    pub name: String,
    pub position: Option<String>,
    pub symbol: String,
    #[serde(rename = "type")]
    pub transaction_type: i32, // 0 = sell, 1 = buy
    pub execution_volume: Option<f64>,
    pub execution_date: i64, // Unix timestamp
    pub start_date: i64,     // Unix timestamp
    pub end_date: i64,       // Unix timestamp
    pub registered_volume: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TopStockInfluencer {
    pub cat_id: i32,
    pub stock_id: i64,
    pub stock_code: String,
    pub influence_index: f64,
    pub close_index: f64,
    pub index_change: f64,
    pub index_per_change: f64,
    pub last_update: i64,
}

pub async fn fetch_balance_sheet_data(
    symbol: &str,
    period: &str,
) -> Result<Vec<FinanceSheetData>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/balance-sheet/{symbol}/{period}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    // let res = resp.json::<FinanceSheetData>().await?;
    let data: Vec<FinanceSheetData> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_cash_flow_gt_sheet_data(
    symbol: &str,
    period: &str,
) -> Result<Vec<FinanceSheetData>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/cash-flow-indirect/{symbol}/{period}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    // let res = resp.json::<FinanceSheetData>().await?;
    let data: Vec<FinanceSheetData> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_cash_flow_tt_sheet_data(
    symbol: &str,
    period: &str,
) -> Result<Vec<FinanceSheetData>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/cash-flow-direct/{symbol}/{period}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    // let res = resp.json::<FinanceSheetData>().await?;
    let data: Vec<FinanceSheetData> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_income_statement_sheet_data(
    symbol: &str,
    period: &str,
) -> Result<Vec<FinanceSheetData>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/income-statement/{symbol}/{period}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    // let res = resp.json::<FinanceSheetData>().await?;
    let data: Vec<FinanceSheetData> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_financial_data(symbol: &str) -> Result<Vec<FinancialData>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/financial-data/{symbol}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let data: Vec<FinancialData> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_sharedholder_data(symbol: &str) -> Result<Vec<SharedHolder>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/shareholder/{symbol}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let data: Vec<SharedHolder> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_institution_data(symbol: &str) -> Result<InstitutionData, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/institution-profile/{symbol}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let data: InstitutionData = resp.json().await?;
    Ok(data)
}

pub async fn fetch_subsidiaries_data(symbol: &str) -> Result<Vec<Subsidiary>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/subsidiaries/{symbol}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let mut data: Vec<Subsidiary> = resp.json().await?;

    // Remove duplicates based on institution_id, keeping the first occurrence
    let mut seen_ids = std::collections::HashSet::new();
    data.retain(|subsidiary| seen_ids.insert(subsidiary.institution_id));

    Ok(data)
}

pub async fn fetch_officers_data(symbol: &str) -> Result<Vec<Officer>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/officer/{symbol}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let data: Vec<Officer> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_insider_transactions_data(
    symbol: &str,
) -> Result<Vec<InsiderTransaction>, reqwest::Error> {
    let url = format!("http://103.48.84.52:4040/insider-transactions/{symbol}");
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let data: Vec<InsiderTransaction> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_top_stock_influencer_data() -> Result<Vec<TopStockInfluencer>, reqwest::Error> {
    let url = "http://103.48.84.52:4040/top-stock-influence";
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let data: Vec<TopStockInfluencer> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_exchange_index_data() -> Result<Vec<ExchangeIndex>, reqwest::Error> {
    let url = "http://103.48.84.52:4040/exchange-index";
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let data: Vec<ExchangeIndex> = resp.json().await?;
    Ok(data)
}

pub async fn fetch_stock_by_gics_data() -> Result<Vec<StockByGics>, reqwest::Error> {
    let url = "http://103.48.84.52:4040/stock-by-gics";
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {TOKEN}"))
        .send()
        .await?;

    let data: Vec<StockByGics> = resp.json().await?;
    Ok(data)
}
