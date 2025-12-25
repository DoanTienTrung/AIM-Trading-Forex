use crate::slint_generatedAppWindow::{AppWindow, HeaderData};
use crate::tasks::task_manager::{register_task, TaskHandle};
use aim_data::aim::{fetch_stock_by_gics_data, StockByGics};
use slint::{Color, ComponentHandle, SharedString};
use std::collections::HashMap;

// Map backend industry_name to UI sector property name
fn map_industry_to_sector(industry: &str) -> Option<&'static str> {
    match industry {
        "Tài chính" => Some("finance_data"),
        "Bất động sản" => Some("bds_data"),
        "Công nghiệp" => Some("Industry_data"),
        "Nguyên vật liệu" => Some("material_data"),
        "Tiêu dùng thiết yếu" => Some("basic_goods_data"),
        "Tiêu dùng không thiết yếu" => Some("advanced_goods_data"),
        "Công nghệ thông tin" => Some("tech_data"),
        "Tiện ích" => Some("facility_data"),
        "Năng lượng" => Some("energy_data"),
        _ => None,
    }
}

fn color_for_percent_change(
    price: f32,
    per_change: f32,
    ceil_price: f32,
    floor_price: f32,
) -> Color {
    if per_change == 0.0 {
        Color::from_rgb_u8(204, 204, 0) // yellow
    } else if per_change < 0.0 {
        if price <= floor_price {
            Color::from_rgb_u8(23, 162, 184) // #17A2B8 blue
        } else {
            Color::from_rgb_u8(220, 53, 69) // #DC3545 red
        }
    } else if price >= ceil_price {
        Color::from_rgb_u8(156, 39, 176) // #9C27B0 purple
    } else {
        Color::from_rgb_u8(40, 167, 69) // #28A745 green
    }
}

fn to_header_data(stock: &StockByGics) -> HeaderData {
    HeaderData {
        symbol: SharedString::from(stock.stock_code.clone()),
        percent_change: stock.per_change as f32,
        price: stock.last_price as f32,
        total_val: (stock.total_val as f64 / 1_000_000_000.0) as f32,
        color: color_for_percent_change(
            stock.last_price as f32,
            stock.per_change as f32,
            stock.ceiling_price as f32,
            stock.floor_price as f32,
        ),
    }
}

pub async fn spawn_heat_map_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "dashboard.heat_map".to_string(),
        tx,
        "Heat Map Data Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
        loop {
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("Cache storage task status changed to: {:?}", status);
                    task_status = status;
                }
            }
            if task_status != crate::tasks::task_manager::TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }
            match fetch_stock_by_gics_data().await {
                Ok(stocks) => {
                    let mut sector_map: HashMap<&str, Vec<HeaderData>> = HashMap::new();
                    for stock in stocks {
                        if let Some(sector) = map_industry_to_sector(stock.industry_name.as_str()) {
                            sector_map
                                .entry(sector)
                                .or_default()
                                .push(to_header_data(&stock));
                        }
                    }
                    for data in sector_map.values_mut() {
                        data.sort_by(|a, b| {
                            b.total_val
                                .partial_cmp(&a.total_val)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    }
                    // Move only Vec<HeaderData> across threads, not ModelRc
                    let finance_data = sector_map.remove("finance_data").unwrap_or_default();
                    let bds_data = sector_map.remove("bds_data").unwrap_or_default();
                    let industry_data = sector_map.remove("Industry_data").unwrap_or_default();
                    let material_data = sector_map.remove("material_data").unwrap_or_default();
                    let basic_goods_data =
                        sector_map.remove("basic_goods_data").unwrap_or_default();
                    let advanced_goods_data =
                        sector_map.remove("advanced_goods_data").unwrap_or_default();
                    let tech_data = sector_map.remove("tech_data").unwrap_or_default();
                    let facility_data = sector_map.remove("facility_data").unwrap_or_default();
                    let energy_data = sector_map.remove("energy_data").unwrap_or_default();
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        let heatmap_data = crate::slint_generatedAppWindow::HeatMapData {
                            finance_data: slint::ModelRc::new(slint::VecModel::from(finance_data)),
                            bds_data: slint::ModelRc::new(slint::VecModel::from(bds_data)),
                            industry_data: slint::ModelRc::new(slint::VecModel::from(
                                industry_data,
                            )),
                            material_data: slint::ModelRc::new(slint::VecModel::from(
                                material_data,
                            )),
                            basic_goods_data: slint::ModelRc::new(slint::VecModel::from(
                                basic_goods_data,
                            )),
                            advanced_goods_data: slint::ModelRc::new(slint::VecModel::from(
                                advanced_goods_data,
                            )),
                            tech_data: slint::ModelRc::new(slint::VecModel::from(tech_data)),
                            facility_data: slint::ModelRc::new(slint::VecModel::from(
                                facility_data,
                            )),
                            energy_data: slint::ModelRc::new(slint::VecModel::from(energy_data)),
                        };
                        ui.set_heatmap_data(heatmap_data);
                    });
                }
                Err(e) => {
                    log::error!("Failed to fetch stock by gics data: {}", e);
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    task_handle
}
