use crate::create_simple_task;
use crate::slint_generatedAppWindow::InfluenceData;
use aim_data::aim::{TopStockInfluencer, fetch_top_stock_influencer_data};
use slint::{ModelRc, VecModel};

// Convert API TopStockInfluencer data to UI InfluenceData format
fn convert_stock_influence_to_ui_data(api_data: Vec<TopStockInfluencer>) -> ModelRc<InfluenceData> {
    let mut ui_data: Vec<InfluenceData> = api_data
        .iter()
        .filter(|item| item.cat_id == 1)
        .map(|item| InfluenceData {
            symbol: item.stock_code.clone().into(),
            value: item.influence_index as f32,
        })
        .collect();

    ui_data.sort_by(|a, b| {
        b.value
            .partial_cmp(&a.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let model = VecModel::from(ui_data);
    ModelRc::new(model)
}

// Use the generic simple task pattern from tasks/mod.rs
create_simple_task!(
    spawn_stock_influence_task,
    "dashboard.stock_influence",
    "Stock Influence Data",
    fetch_top_stock_influencer_data,
    set_stock_influence_data,
    Vec<TopStockInfluencer>,
    convert_stock_influence_to_ui_data,
    2000 // Update every 2 seconds for stock influence data
);
