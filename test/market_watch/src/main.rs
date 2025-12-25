// Include all Slint UI modules
slint::include_modules!();
use slint::{Model, VecModel};

use crate::tasks::{sort_market_watch, spawn_data_update_task, spawn_mini_chart_hnx30_task, spawn_mini_chart_hnxindex_task, spawn_mini_chart_vn30_task, spawn_mini_chart_vnindex_task};

mod tasks;
mod aim_chart;
mod aim_data;

#[tokio::main]
async fn main() {
    // Initialize logging with Info level
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Error)
        .init();

    let ui = slint_generatedAppWindow::AppWindow::new().unwrap();

    let ui_handle_market_watch = ui.as_weak();
    ui.on_sort_market_watch(move |sort_column| {
        let ui_handle_clone = ui_handle_market_watch.clone();
        tokio::spawn(async move {
            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                let market_data = ui.get_market_watch_data();
                let market_data_vec: Vec<_> = (0..market_data.row_count())
                    .map(|i| market_data.row_data(i).unwrap())
                    .collect();

                let sort_ascending = ui.get_market_watch_sort_ascending();
                let show_percentage = ui.get_market_watch_show_percentage();
                let sorted_data = sort_market_watch(
                    &market_data_vec,
                    sort_column,
                    sort_ascending,
                    show_percentage,
                );

                // Update the UI with sorted data
                ui.set_market_watch_data(slint::ModelRc::new(VecModel::from(sorted_data)));
            });
        });
    });
    spawn_data_update_task(&ui);
    spawn_mini_chart_vnindex_task(&ui);
    spawn_mini_chart_vn30_task(&ui);
    spawn_mini_chart_hnx30_task(&ui);
    spawn_mini_chart_hnxindex_task(&ui);
    // Set up window close handler
    ui.window().on_close_requested(|| {
        log::info!("Closing the application...");
        std::process::exit(0);
    });

    // Run the UI main loop
    ui.run().unwrap();
}