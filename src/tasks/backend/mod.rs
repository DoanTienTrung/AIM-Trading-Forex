mod finance_list;
pub use finance_list::FinanceListExt;

use aim_data::aim::{FinanceSheetData, FinancialData, FinancialDetail};

/// Convert FinancialData to FinanceSheetData format for overview sheet
pub fn convert_financial_data_to_overview(
    financial_data: Vec<FinancialData>,
) -> Vec<Vec<FinanceSheetData>> {
    if financial_data.is_empty() {
        return Vec::new();
    }

    // Define metric groups with their respective metrics
    let metric_groups = vec![
        (
            "Cân đối kế toán", // Balance Sheet
            vec![
                ("Tổng tài sản", "total_asset"),
                ("Nợ phải trả", "total_debt"),
                ("Tổng hàng tồn kho", "total_inventory"),
            ],
        ),
        (
            "Kết quả kinh doanh", // Income Statement
            vec![
                ("Doanh thu thuần", "net_sale"),
                ("Lợi nhuận sau thuế", "profit_after_tax"),
                ("EBITDA", "ebitda"),
            ],
        ),
        (
            "Tỷ lệ định giá", // Valuation Ratios
            vec![
                ("P/E", "pe"),
                ("P/S", "ps"),
                ("P/B", "pb"),
                ("EV/EBITDA", "ev_over_ebitda"),
            ],
        ),
        (
            "Chỉ số hiệu quả", // Performance Metrics
            vec![
                ("EPS", "basic_eps"),
                ("BVPS", "book_value_per_share"),
                ("Dividend Yield", "dividend_yield"),
            ],
        ),
        (
            "Tỷ suất sinh lời", // Profitability Ratios
            vec![
                ("ROE", "roe"),
                ("ROA", "roa"),
                ("ROIC", "sector_roic"),
                ("ROCE", "sector_roce"),
            ],
        ),
        (
            "Tỷ lệ lợi nhuận", // Margin Ratios
            vec![
                ("Gross Margin", "gross_margin"),
                ("Operating Margin", "operating_margin"),
                ("Net Profit Margin", "pre_tax_margin"),
            ],
        ),
        (
            "Tăng trưởng", // Growth Metrics
            vec![
                ("Tăng trưởng tài sản (QoQ)", "current_asset_growth_qoq"),
                ("Tăng trưởng doanh thu", "sale_growth"),
                ("Tăng trưởng EPS", "basic_eps_growth"),
            ],
        ),
        (
            "Kế hoạch", // Planning Metrics
            vec![
                ("Kế hoạch Doanh thu", "planning_revenue"),
                ("Kế hoạch LN trước thuế", "planning_profit_before_tax"),
                ("Kế hoạch LN sau thuế", "planning_profit_after_tax"),
                ("Kế hoạch EPS", "planning_eps"),
                ("Kế hoạch cổ tức", "planning_cash_dividend"),
            ],
        ),
        (
            "Điểm số đánh giá", // Scoring Metrics
            vec![
                ("F-score (Piotroski)", "piotroski_f_score"),
                ("Z-score (Altman)", "manufacturing_z_score"),
            ],
        ),
    ];

    // Group financial data by period (quarter + year)
    let mut periods_data: std::collections::HashMap<String, &FinancialData> =
        std::collections::HashMap::new();
    for financial_entry in &financial_data {
        let period_key = format!("Q{}{}", financial_entry.quarter, financial_entry.year);
        periods_data.insert(period_key, financial_entry);
    }

    // Sort periods in descending order (most recent first)
    let mut sorted_periods: Vec<(String, &FinancialData)> = periods_data.into_iter().collect();
    sorted_periods.sort_by(|a, b| {
        // Extract year and quarter for comparison
        let parse_period = |period: &str| -> (i32, i32) {
            if let Some(q_pos) = period.find('Q') {
                let quarter_str = &period[q_pos + 1..q_pos + 2];
                let year_str = &period[q_pos + 2..];
                let quarter = quarter_str.parse::<i32>().unwrap_or(0);
                let year = year_str.parse::<i32>().unwrap_or(0);
                (year, quarter)
            } else {
                (0, 0)
            }
        };

        let (year_a, quarter_a) = parse_period(&a.0);
        let (year_b, quarter_b) = parse_period(&b.0);

        // Sort by year descending, then by quarter descending
        match year_b.cmp(&year_a) {
            std::cmp::Ordering::Equal => quarter_b.cmp(&quarter_a),
            other => other,
        }
    });

    // Create the structure expected by FinanceListExt::from_data
    // This should be Vec<Vec<FinanceSheetData>> where each inner Vec represents one period
    let mut result = Vec::new();

    // Create a structure for each period (now in sorted order)
    for (period_name, financial_entry) in sorted_periods {
        let mut period_data = Vec::new();
        let mut id_counter = 1i64;

        // Add grouped metrics for this period
        for (group_name, metrics) in &metric_groups {
            // Add group header
            period_data.push(FinanceSheetData {
                id: id_counter,
                name: Some(group_name.to_string()),
                parent_id: Some(-1), // Top level group
                expanded: Some(true),
                level: Some(1),
                field: None, // Group headers don't have fields
                period: Some(period_name.clone()),
                year: Some(financial_entry.year),
                quarter: Some(financial_entry.quarter),
                value: None, // Group headers don't have values
                symbol: Some(financial_entry.symbol.clone()),
            });

            let group_parent_id = id_counter;
            id_counter += 1;

            // Add metrics within this group
            for (metric_name, field_name) in metrics {
                let value = get_financial_value(&financial_entry.financial_values, field_name);
                period_data.push(FinanceSheetData {
                    id: id_counter,
                    name: Some(metric_name.to_string()),
                    parent_id: Some(group_parent_id), // Child of group
                    expanded: Some(true),
                    level: Some(2),
                    field: Some(field_name.to_string()),
                    period: Some(period_name.clone()),
                    year: Some(financial_entry.year),
                    quarter: Some(financial_entry.quarter),
                    value,
                    symbol: Some(financial_entry.symbol.clone()),
                });
                id_counter += 1;
            }
        }

        result.push(period_data);
    }

    result
}

/// Helper function to extract financial values
fn get_financial_value(financial_values: &FinancialDetail, field_name: &str) -> Option<f64> {
    match field_name {
        "total_asset" => financial_values.total_asset.map(|v| v as f64),
        "total_debt" => financial_values.total_debt.map(|v| v as f64),
        "total_inventory" => financial_values.total_inventory.map(|v| v as f64),
        "net_sale" => financial_values.net_sale.map(|v| v as f64),
        "profit_after_tax" => financial_values.profit_after_tax.map(|v| v as f64),
        "pe" => financial_values.pe,
        "ps" => financial_values.ps,
        "pb" => financial_values.pb,
        "ebitda" => financial_values.ebitda.map(|v| v as f64),
        "ev_over_ebitda" => financial_values.ev_over_ebitda,
        "basic_eps" => financial_values.basic_eps,
        "book_value_per_share" => financial_values.book_value_per_share,
        "dividend_yield" => financial_values.dividend_yield,
        "roe" => financial_values.roe,
        "roa" => financial_values.roa,
        "sector_roic" => financial_values.sector_roic,
        "sector_roce" => financial_values.sector_roce,
        "gross_margin" => financial_values.gross_margin,
        "operating_margin" => financial_values.operating_margin,
        "pre_tax_margin" => financial_values.pre_tax_margin,
        "current_asset_growth_qoq" => financial_values.current_asset_growth_qoq,
        "sale_growth" => financial_values.sale_growth,
        "basic_eps_growth" => financial_values.basic_eps_growth,
        "planning_revenue" => financial_values.planning_revenue,
        "planning_profit_before_tax" => financial_values
            .planning_profit_before_tax
            .map(|v| v as f64),
        "planning_profit_after_tax" => financial_values.planning_profit_after_tax.map(|v| v as f64),
        "planning_eps" => financial_values.planning_eps,
        "planning_cash_dividend" => financial_values.planning_cash_dividend,
        "piotroski_f_score" => financial_values.piotroski_f_score.map(|v| v as f64),
        "manufacturing_z_score" => financial_values.manufacturing_z_score,
        _ => None,
    }
}
