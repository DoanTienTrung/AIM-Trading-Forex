use std::cell::RefCell;
use std::rc::Rc;

use crate::tasks::task_manager::{register_task, TaskStatus};
use crate::AppWindow;
use aim_data::aim::{fetch_rsi14_data, RsiData}; // Import RsiData explicitly
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use crate::slint_generatedAppWindow::{
    RsiData as SlintRsiData, 
    CoinData as SlintCoinData,
    MaData as SlintMaData 
};
use rand::Rng; // For dummy data generation

// Helper function to generate dummy data for debugging/weekends
fn generate_dummy_data() -> Vec<RsiData> {
    let symbols = vec![
        "ACB", "BCM", "BID", "BVH", "CTG", "FPT", "GAS", "GVR", "HDB", "HPG", 
        "MBB", "MSN", "MWG", "PLX", "POW", "SAB", "SHB", "SSB", "SSI", "STB", 
        "TCB", "TPB", "VCB", "VHM", "VIB", "VIC", "VJC", "VNM", "VPB", "VRE"
    ];
    let mut rng = rand::rng();
    
    symbols.into_iter().map(|sym| {
        RsiData {
            symbol: sym.to_string(),
            value: Some(rng.random_range(20.0..80.0)),
            price: Some(rng.random_range(10.0..100.0)),
            vhtt: Some(rng.random_range(1.0..1000.0)), // Billions
            timestamp: 0,
            interval: "D".to_string(),
        }
    }).collect()
}

pub async fn spawn_rsi_task(ui: &AppWindow) -> crate::tasks::task_manager::TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle =
        register_task("chart.quantitative.mp".to_string(), tx, "Quantitative MP (RSI & MA)".to_string()).await;

    tokio::spawn(async move {
        let mut task_status = TaskStatus::Running;

        loop {
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("Quantitative MP task status changed to: {:?}", status);
                    task_status = status;
                }
            }

            if task_status != TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            // Fetch data from API
            let fetched_result = fetch_rsi14_data().await;
            
            // Logic: If API returns empty (or fails), use dummy data for Debugging/Weekend
            let final_data = match fetched_result {
                Ok(data) => {
                    if data.is_empty() {
                        log::warn!("RSI14 API returned empty. Using Dummy Data for Debugging.");
                        generate_dummy_data()
                    } else {
                        log::info!("Fetched {} RSI14 records", data.len());
                        data
                    }
                },
                Err(e) => {
                    log::error!("Failed to fetch RSI14 data: {}. Using Dummy Data.", e);
                    generate_dummy_data()
                }
            };

            // --- DATA PROCESSING ---

            // 1. RSI List Data (Left Table)
            let rsi_list_data: Vec<SlintRsiData> = final_data.iter().map(|d| {
                    SlintRsiData {
                    symbol: d.symbol.clone().into(),
                    rsi14: d.value.map(|v| format!("{:.2}", v)).unwrap_or_default().into(),
                    price: d.price.map(|v| format!("{:.2}", v)).unwrap_or_default().into(),
                    vhtt: d.vhtt.map(|v| format!("{:.2} B", v)).unwrap_or_default().into(),
                }
            }).collect();

            // 2. MA List Data (Bottom Left Table) - Using same data as RSI List for now
            let ma_table_list_data: Vec<SlintRsiData> = final_data.iter().map(|d| {
                    SlintRsiData {
                    symbol: d.symbol.clone().into(),
                    rsi14: d.value.map(|v| format!("{:.2}", v)).unwrap_or_default().into(),
                    price: d.price.map(|v| format!("{:.2}", v)).unwrap_or_default().into(),
                    vhtt: d.vhtt.map(|v| format!("{:.2} B", v)).unwrap_or_default().into(),
                }
            }).collect();

            // Common Logic for Market Cap (X-Axis)
            // Range: Min 10B -> Max 1,000,000B
            let max_log = 6.0; 
            let min_log = 1.0; 

            // 3. RSI Heatmap (Top Right Chart)
            let coin_list_data: Vec<SlintCoinData> = final_data.iter().map(|d| {
                let rsi = d.value.unwrap_or(50.0) as f32;
                let vhtt = d.vhtt.unwrap_or(0.0);
                
                // Y-AXIS: RSI (Linear 0-100) -> Slint 1.0-0.0
                let y_pos = 1.0 - (rsi / 100.0);

                // X-AXIS: Market Cap (Logarithmic)
                let safe_vhtt = if vhtt < 1.0 { 1.0 } else { vhtt }; 
                let cap_log = safe_vhtt.log10() as f32;
                let normalized_cap = (cap_log - min_log) / (max_log - min_log);
                let x_pos = 1.0 - normalized_cap;

                SlintCoinData {
                    symbol: d.symbol.clone().into(),
                    rsi: rsi,
                    x: x_pos.max(0.02).min(0.98), 
                    y: y_pos.max(0.05).min(0.95), 
                }
            }).collect();

            // 4. MA Heatmap (Bottom Right Chart)
            let ma_list_data: Vec<SlintMaData> = final_data.iter().enumerate().map(|(i, d)| {
                // X-AXIS: Same as RSI Chart
                let vhtt = d.vhtt.unwrap_or(0.0);
                let safe_vhtt = if vhtt < 1.0 { 1.0 } else { vhtt }; 
                let cap_log = safe_vhtt.log10() as f32;
                let normalized_cap = (cap_log - min_log) / (max_log - min_log);
                let x_pos = 1.0 - normalized_cap;

                // Y-AXIS: Price vs MA Deviation
                // Chart Scale: 2.0 (Top) to 0.0 (Bottom), Center is 1.0
                // We generate a synthetic deviation for now
                let mut rng = rand::rng();
                let synthetic_ma_deviation = rng.random_range(0.0..2.0); // Random deviation between 0.0 and 2.0

                // Map deviation (0.0 - 2.0) to Slint Y (1.0 - 0.0)
                let y_val = synthetic_ma_deviation as f32;
                let y_pos = 1.0 - (y_val / 2.0); 

                // COLOR LOGIC PROXY:
                // Map Y-axis position (Deviation) to a 0-100 scale to drive color.
                // High Deviation (2.0) -> Proxy 100 -> Red
                // Low Deviation (0.0) -> Proxy 0 -> Green
                let color_proxy_rsi = (y_val / 2.0) * 100.0;

                SlintMaData {
                    symbol: d.symbol.clone().into(),
                    rsi: color_proxy_rsi, // Pass proxy RSI to color the dot correctly (High=Red, Low=Green)
                    x: x_pos.max(0.02).min(0.98),
                    y: y_pos.max(0.05).min(0.95),
                }
            }).collect();

            // Update UI in one go
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                ui.set_rsi_list(ModelRc::new(VecModel::from(rsi_list_data)));
                ui.set_coin_list(ModelRc::new(VecModel::from(coin_list_data)));
                ui.set_ma_list(ModelRc::new(VecModel::from(ma_list_data)));
                //ui.set_ma_table_list(ModelRc::new(VecModel::from(ma_table_list_data))); // Update MA Table
            });

            // Refresh rate
            tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        }
    });

    task_handle
}

pub fn register_rsi_sort_handler(ui: &AppWindow) {
    log::warn!("Registering RSI sort handler");
    let ui_weak = ui.as_weak();
    
    let sort_ascending = Rc::new(RefCell::new(false));

    ui.on_sort_rsi14(move || {
        log::warn!("RSI sort handler triggered");
        let ui = ui_weak.unwrap();
        
        let model_rc = ui.get_rsi_list();
        let mut items: Vec<SlintRsiData> = model_rc.iter().collect();
        let mut asc = sort_ascending.borrow_mut();
        *asc = !*asc;
        let is_asc = *asc;

        items.sort_by(|a, b| {
            let val_a = a.rsi14.as_str().parse::<f64>().unwrap_or(0.0);
            let val_b = b.rsi14.as_str().parse::<f64>().unwrap_or(0.0);
            
            if is_asc {
                val_a.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal)
            } else {
                val_b.partial_cmp(&val_a).unwrap_or(std::cmp::Ordering::Equal)
            }
        });

        ui.set_rsi_list(ModelRc::new(VecModel::from(items)));
        log::warn!("RSI List sorted. Ascending: {}", is_asc);
    });
}