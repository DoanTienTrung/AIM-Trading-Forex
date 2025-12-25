use crate::create_simple_task;
use crate::slint_generatedAppWindow::IndexRow;
use aim_data::aim::{fetch_exchange_index_data, ExchangeIndex};
use slint::{ModelRc, VecModel};

fn convert_exchange_index_to_index_row(api_data: Vec<ExchangeIndex>) -> ModelRc<IndexRow> {
    let ui_data: Vec<IndexRow> = api_data
        .iter()
        .map(|item| IndexRow {
            name: item.index_id.clone().to_uppercase().into(),
            score: item.index_value as f32,
            change: item.change.unwrap_or(0.0) as f32,
            change_percent: item.change_percent.unwrap_or(0.0) as f32,
            advances: item.advances,
            ceiling: item.ceiling,
            nochanges: item.nochanges,
            declines: item.declines,
            floor: item.floor,
            volume: (item.all_value.unwrap_or(0) / 1_000_000_000) as i32,
        })
        .collect();

    let model = VecModel::from(ui_data);
    ModelRc::new(model)
}

// Use the generic simple task pattern from tasks/mod.rs
create_simple_task!(
    spawn_overall_index_task,
    "dashboard.overall_index",
    "Overall Index Data",
    fetch_exchange_index_data,
    set_overall_index_data,
    Vec<ExchangeIndex>,
    convert_exchange_index_to_index_row,
    5000 // Update every 5 seconds for overall index data
);
