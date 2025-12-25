use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyFinancialRatio {
    // #[serde(rename = "ticker")]
    // pub ticker: String,
    // #[serde(rename = "yearReport")]
    // pub year_report: i32,
    // #[serde(rename = "lengthReport")]
    // pub length_report: i32,
    // #[serde(rename = "updateDate")]
    // pub update_date: i64,
    // #[serde(rename = "revenue")]
    // pub revenue: f64,
    // #[serde(rename = "revenueGrowth")]
    // pub revenue_growth: Option<f64>,
    // #[serde(rename = "netProfit")]
    // pub net_profit: f64,
    // #[serde(rename = "netProfitGrowth")]
    // pub net_profit_growth: Option<f64>,
    // #[serde(rename = "ebitMargin")]
    // pub ebit_margin: Option<f64>,
    #[serde(rename = "roe")]
    pub roe: Option<f64>,
    #[serde(rename = "roic")]
    pub roic: Option<f64>,
    #[serde(rename = "roa")]
    pub roa: Option<f64>,
    #[serde(rename = "pe")]
    pub pe: Option<f64>,
    #[serde(rename = "pb")]
    pub pb: Option<f64>,
    #[serde(rename = "eps")]
    pub eps: Option<f64>,
    // #[serde(rename = "currentRatio")]
    // pub current_ratio: f64,
    // #[serde(rename = "cashRatio")]
    // pub cash_ratio: f64,
    // #[serde(rename = "quickRatio")]
    // pub quick_ratio: f64,
    // #[serde(rename = "interestCoverage")]
    // pub interest_coverage: Option<f64>,
    // #[serde(rename = "ae")]
    // pub ae: f64,
    // #[serde(rename = "netProfitMargin")]
    // pub net_profit_margin: f64,
    // #[serde(rename = "grossMargin")]
    // pub gross_margin: f64,
    // #[serde(rename = "ev")]
    // pub ev: f64,
    // #[serde(rename = "issueShare")]
    // pub issue_share: f64,
    // #[serde(rename = "ps")]
    // pub ps: f64,
    // #[serde(rename = "pcf")]
    // pub pcf: f64,
    // #[serde(rename = "bvps")]
    // pub bvps: f64,
    // #[serde(rename = "evPerEbitda")]
    // pub ev_per_ebitda: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyFinancialRatioPeriod {
    #[serde(rename = "ratio")]
    pub ratio: Vec<CompanyFinancialRatio>,
    #[serde(rename = "period")]
    pub period: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyInfo {
    #[serde(rename = "data")]
    pub data: CompanyFinancialRatioData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyFinancialRatioData {
    #[serde(rename = "CompanyFinancialRatio")]
    pub company_financial_ratio: CompanyFinancialRatioPeriod,
}
