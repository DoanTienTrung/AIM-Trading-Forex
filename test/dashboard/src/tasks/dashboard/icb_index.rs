use crate::create_simple_task;
use crate::slint_generatedAppWindow::ICBRow;
use aim_data::aim::{IcbIndex, fetch_icb_index_data};
use slint::{ModelRc, VecModel};

fn convert_icb_index_to_ui_row(api_data: Vec<IcbIndex>) -> ModelRc<ICBRow> {
    let allowed_codes = [
        "10", "15", "20", "30", "35", "40", "45", "50", "55", "60", "65",
    ];
    let ui_data: Vec<ICBRow> = api_data
        .iter()
        .filter(|item| allowed_codes.contains(&item.icb_code.as_str()))
        .map(|item| ICBRow {
            sector: item.icb_name.clone().into(),
            percent: if item.index_prev != 0.0 {
                ((item.index_close - item.index_prev) / item.index_prev * 100.0) as f32
            } else {
                0.0
            },
            gtgd: (item.value as f32) / 1_000_000_000.0,
            nn_sell: (item.sell_foreign_value as f32) / 1_000_000_000.0,
            nn_buy: (item.buy_foreign_value as f32) / 1_000_000_000.0,
        })
        .collect();

    let model = VecModel::from(ui_data);
    ModelRc::new(model)
}

// Use the generic simple task pattern from tasks/mod.rs
create_simple_task!(
    spawn_icb_index_task,
    "dashboard.icb_index",
    "ICB Index Data",
    fetch_icb_index_data,
    set_icb_index_data,
    Vec<IcbIndex>,
    convert_icb_index_to_ui_row,
    1000 // Update every 1 second for ICB index data
);
