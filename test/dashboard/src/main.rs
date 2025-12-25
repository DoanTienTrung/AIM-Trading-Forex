use crate::tasks::{
    spawn_abnormal_trade_task, spawn_heat_map_task, spawn_icb_index_task, spawn_overall_index_task,
    spawn_sjc_price_task, spawn_stock_influence_task, spawn_trading_volume_task,
};

// Include all Slint UI modules
slint::include_modules!();

mod tasks;

#[tokio::main]
async fn main() {
    let ui = slint_generatedAppWindow::AppWindow::new().unwrap();

    spawn_abnormal_trade_task(&ui).await; // update AbnormalTrade UI component
    spawn_heat_map_task(&ui).await; // update HeatMap UI component
    spawn_icb_index_task(&ui).await; // update ICBIndex UI component
    spawn_overall_index_task(&ui).await; // update OverallIndex UI component
    spawn_sjc_price_task(&ui).await; // update SJC price data for goods UI component
    spawn_stock_influence_task(&ui).await;
    spawn_trading_volume_task(&ui).await; // update TradingVolume UI component

    // Set up window close handler
    ui.window().on_close_requested(|| {
        log::info!("Closing the application...");
        std::process::exit(0);
    });

    // Run the UI main loop
    ui.run().unwrap();
}
