// use std::time::Duration;

// use slint::ComponentHandle;

// use crate::{aim_data, AppWindow};

// pub fn spawn_world_index_task(ui: &AppWindow) {
//     let ui_handle = ui.as_weak();
//     tokio::spawn(async move {
//         loop {
//             if let Ok(btc_price) = aim_data::get_btc_price().await {
//                 // Update the BTC price in the UI
//                 let _ = ui_handle.upgrade_in_event_loop(move |ui| {
//                     let btc_price =
//                         format!("{},{:.2}", (btc_price / 1000.0) as u32, btc_price % 1000.0);
//                     ui.set_btc_price(btc_price.into());
//                 });
//             } else {
//                 let _ = ui_handle.upgrade_in_event_loop(move |ui| {
//                     // Update the world index data in the UI
//                     ui.set_btc_price("null".into());
//                 });
//             }
//             tokio::time::sleep(Duration::from_secs(1)).await; // Update every 60 seconds
//         }
//     });
// }
