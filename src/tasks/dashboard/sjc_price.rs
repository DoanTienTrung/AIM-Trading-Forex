use crate::create_simple_task;
use crate::slint_generatedAppWindow::GoodsData;
use aim_data::aim::{fetch_sjc_price_data, SjcPriceData};
use slint::{ModelRc, VecModel};

fn convert_sjc_price_to_ui_data(api_data: Vec<SjcPriceData>) -> ModelRc<GoodsData> {
    use std::collections::HashSet;

    let mut seen_gold_types = HashSet::new();
    let goods_data: Vec<GoodsData> = api_data
        .iter()
        .filter(|item| {
            // Only include if we haven't seen this gold_type before
            seen_gold_types.insert(item.gold_type.clone())
        })
        .map(|item| {
            // Format prices with thousands separator
            let buy_price = format_price(&item.buy_price);
            let sell_price = format_price(&item.sell_price);

            GoodsData {
                name: item.gold_type.clone().into(),
                buy_price: buy_price.into(),
                sell_price: sell_price.into(),
            }
        })
        .collect();

    let model = VecModel::from(goods_data);
    ModelRc::new(model)
}

fn format_price(price_str: &str) -> String {
    // Try to parse the price and format it with thousands separator
    if let Ok(price) = price_str.replace(",", "").parse::<u64>() {
        // Manual thousands separator formatting
        let price_str = price.to_string();
        let mut result = String::new();
        let chars: Vec<char> = price_str.chars().collect();

        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i).is_multiple_of(3) {
                result.push(',');
            }
            result.push(*ch);
        }
        result
    } else {
        price_str.to_string()
    }
}

// Use the generic simple task pattern from tasks/mod.rs
create_simple_task!(
    spawn_sjc_price_task,
    "dashboard.sjc_price",
    "SJC Price Data",
    fetch_sjc_price_data,
    set_good_data,
    Vec<SjcPriceData>,
    convert_sjc_price_to_ui_data,
    30000 // Update every 30 seconds for gold price data
);
