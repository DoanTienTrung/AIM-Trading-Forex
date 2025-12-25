use crate::AppWindow;
use crate::tasks::task_manager::{register_task, TaskHandle};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use slint::{ComponentHandle, ModelRc, VecModel};
use slint::Image;

use aim_data::aim::{
    fetch_finance_report_list, fetch_strategy_report_list, fetch_finance_report_pdf,
    StockReport as ApiStockReport, StrategyReport as ApiStrategyReport, PdfReport,
};

use crate::slint_generatedAppWindow::{
    StockReport as UIStockReport, StrategyReport as UIStrategyReport,
};

/// 🔹 Chuyển đổi dữ liệu API sang UI model
fn convert_api_stock_report_to_ui(api: &ApiStockReport) -> UIStockReport {
    UIStockReport {
        code: api.code.clone().into(),
        name: api.name.clone().into(),
        recommend: api.recommend.clone().into(),
        target: api.target.clone().into(),
        upside: api.upside.clone().into(),
        date: api.date.clone().into(),
        report_id: api.report_id.clone().into(),
    }
}

fn convert_api_strategy_to_ui(api: &ApiStrategyReport) -> UIStrategyReport {
    UIStrategyReport {
        code: api.code.clone().into(),
        name: api.name.clone().into(),
        status: api.status.clone().into(),
        date: api.date.clone().into(),
    }
}

/// 🔸 Task chính: quản lý toàn bộ dữ liệu báo cáo tài chính
pub async fn spawn_finance_report_task(ui: &AppWindow) -> Vec<TaskHandle> {
    let mut handles = Vec::new();
    handles.push(spawn_stock_report_task(ui).await);
    handles.push(spawn_finance_pdf_task(ui).await);
    handles
}

/// 🧩 Task 1: Báo cáo cổ phiếu (StockReport)
async fn spawn_stock_report_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    let task_handle = register_task(
        "chart.finance_report.stock_reports".to_string(),
        tx,
        "Stock Report Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let cache = Arc::new(Mutex::new(Vec::<ApiStockReport>::new()));
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;

        loop {
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("Stock report task status changed: {:?}", status);
                    task_status = status;
                }
            }

            if task_status != crate::tasks::task_manager::TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                continue;
            }

            let mut data: Vec<ApiStockReport> = vec![];

            // Cache check
            {
                let cache_guard = cache.lock().await;
                if !cache_guard.is_empty() {
                    data = cache_guard.clone();
                }
            }

            if data.is_empty() {
                log::info!("📊 Fetching new Stock Reports...");
                match fetch_finance_report_list().await {
                    Ok(api_data) => {
                        log::info!("✅ API trả về {} bản ghi gốc", api_data.len());

                        // Ghi log chi tiết 3 phần tử đầu để kiểm tra cấu trúc
                        for (i, r) in api_data.iter().take(3).enumerate() {
                            log::info!(
                                "[DEBUG] Mẫu dữ liệu #{i}: symbol={:?}, title={:?}, date={:?}",
                                r.symbol, r.title, r.date
                            );
                        }

                        // Đếm số bản ghi thiếu trường quan trọng
                        let missing_symbol = api_data.iter().filter(|r| r.symbol.is_none()).count();
                        let missing_title = api_data.iter().filter(|r| r.title.is_none()).count();
                        let missing_date = api_data.iter().filter(|r| r.date.is_none()).count();

                        if missing_symbol > 0 || missing_title > 0 || missing_date > 0 {
                            log::warn!(
                                "⚠️ Dữ liệu thiếu trường: symbol={}, title={}, date={}",
                                missing_symbol,
                                missing_title,
                                missing_date
                            );
                        }

                        // Map dữ liệu API -> dữ liệu hiển thị
                        let mapped: Vec<ApiStockReport> = api_data
                            .into_iter()
                            .map(|r| ApiStockReport {
                                code: r.symbol.unwrap_or_else(|| "N/A".into()),
                                name: r.title.unwrap_or_else(|| "Không tiêu đề".into()),
                                recommend: r.source_name.unwrap_or_else(|| "-".into()),
                                target: "-".into(),
                                upside: "-".into(),
                                date: r.date.unwrap_or_default(),
                                report_id: r.report_id.map(|id| id.to_string()).unwrap_or_default(),
                            })
                            .collect();

                        log::info!("🧭 Đã map {} bản ghi hợp lệ sang UI", mapped.len());

                        data = mapped.clone();
                        *cache.lock().await = mapped;
                    }
                    Err(e) => {
                        log::error!("❌ Failed to fetch stock reports: {e}");
                        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                        continue;
                    }
                }
            }



            let ui_reports: Vec<UIStockReport> =
                data.iter().map(convert_api_stock_report_to_ui).collect();

            let ui_handle_clone = ui_handle.clone();
            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                let model: ModelRc<UIStockReport> = ModelRc::new(VecModel::from(ui_reports));
                ui.set_report_list(model);
                log::info!("✅ Updated stock reports list");
            });

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    task_handle
}

/// 🧩 Task 2: Chiến lược đầu tư (StrategyReport)
async fn spawn_strategy_report_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    let task_handle = register_task(
        "chart.finance_report.strategy_reports".to_string(),
        tx,
        "Strategy Report Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let cache = Arc::new(Mutex::new(Vec::<ApiStrategyReport>::new()));
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;

        loop {
            if let Ok(status) = rx.try_recv() {
                task_status = status;
            }

            if task_status != crate::tasks::task_manager::TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                continue;
            }

            let mut data: Vec<ApiStrategyReport> = vec![];

            {
                let cache_guard = cache.lock().await;
                if !cache_guard.is_empty() {
                    data = cache_guard.clone();
                }
            }

            if data.is_empty() {
                log::info!("Fetching new Strategy Reports...");
                match fetch_strategy_report_list().await {
                    Ok(api_data) => {
                        data = api_data.clone();
                        *cache.lock().await = api_data;
                    }
                    Err(e) => {
                        log::error!("Failed to fetch strategy reports: {e}");
                        continue;
                    }
                }
            }

            let ui_reports: Vec<UIStrategyReport> =
                data.iter().map(convert_api_strategy_to_ui).collect();

            let ui_handle_clone = ui_handle.clone();
            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                let model = ModelRc::new(VecModel::from(ui_reports));
                ui.set_strategy_list(model);
                log::info!("✅ Updated strategy reports list");
            });

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    task_handle
}


// Helper: render PDF -> vec đường dẫn file PNG (trả về String paths)
pub async fn render_pdf_to_png_paths(pdf_path: &str) -> anyhow::Result<Vec<String>> {
    use pdfium_render::prelude::*;
    use std::env;
    use std::fs;
    use uuid::Uuid;

    log::info!("[LOI]🔧 render_pdf_to_png_paths() start for {}", pdf_path);

    // Bước 1: Kiểm tra file tồn tại
    if !std::path::Path::new(pdf_path).exists() {
        log::error!("[LOI]❌ File PDF không tồn tại: {}", pdf_path);
        anyhow::bail!("File PDF không tồn tại");
    }

    // Bước 2: Khởi tạo Pdfium
    log::info!("[LOI]🧩 Khởi tạo Pdfium...");
    let pdfium = match Pdfium::bind_to_system_library() {
        Ok(binding) => {
            log::info!("[LOI]✅ Đã bind Pdfium thành công");
            Pdfium::new(binding)
        }
        Err(e) => {
            log::error!("[LOI]❌ Không thể bind Pdfium: {:?}", e);
            anyhow::bail!("Không bind được Pdfium: {:?}", e);
        }
    };

    // Bước 3: Tải file PDF
    log::info!("[LOI]📄 Đang load file PDF: {}", pdf_path);
    let doc = match pdfium.load_pdf_from_file(pdf_path, None) {
        Ok(d) => {
            log::info!("[LOI]✅ Đã load PDF thành công, tổng {} trang", d.pages().len());
            d
        }
        Err(e) => {
            log::error!("[LOI]❌ Lỗi khi load PDF: {:?}", e);
            anyhow::bail!("Không load được PDF: {:?}", e);
        }
    };

    // Bước 4: Render từng trang
    let mut output_paths: Vec<String> = Vec::new();

    for (i, page) in doc.pages().iter().enumerate() {
        log::info!("[LOI]🖼️ Render trang {}", i + 1);

        match page.render_with_config(&PdfRenderConfig::new().set_target_width(1200)) {
            Ok(rendered) => {
                let dyn_img = rendered.as_image();
                let tmp_dir = env::temp_dir();
                let filename = format!("pdf_page_{}_{}.png", i + 1, uuid::Uuid::new_v4());
                let out_path = tmp_dir.join(filename);
                let out_path_str = out_path.to_string_lossy().to_string();

                if let Err(e) = dyn_img.save(&out_path_str) {
                    log::error!("[LOI]❌ Lưu ảnh PNG thất bại: {:?}", e);
                } else {
                    log::debug!("[LOI]💾 Đã lưu -> {}", out_path_str);
                    output_paths.push(out_path_str);
                }
            }
            Err(e) => {
                log::error!("[LOI]❌ Render trang {} thất bại: {:?}", i + 1, e);
            }
        }
    }

    // Bước 5: Tổng kết
    log::info!(
        "[LOI]✅ Hoàn tất render PDF -> PNG, tổng cộng {} trang hợp lệ",
        output_paths.len()
    );

    Ok(output_paths)
}


// Revised spawn_finance_pdf_task
pub async fn spawn_finance_pdf_task(ui: &AppWindow) -> TaskHandle {
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    let task_handle = register_task(
        "chart.finance_report.pdf".to_string(),
        tx,
        "Finance PDF Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
        let cache = Arc::new(Mutex::new(HashMap::<String, PdfReport>::new()));
        let current_stock = Arc::new(Mutex::new(String::from("72443")));
        let pre_stock = Arc::new(Mutex::new(String::new()));

        loop {
            if let Ok(status) = rx.try_recv() {
                task_status = status;
            }

            if task_status != crate::tasks::task_manager::TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                continue;
            }

            // Lấy mã cổ phiếu hiện tại từ UI — chạy upgrade_in_event_loop để đọc UI (đọc chuỗi là OK)
            let current_stock_clone = Arc::clone(&current_stock);
            let ui_handle_clone_for_read = ui_handle.clone();
            // Note: upgrade_in_event_loop is ok for reading values if called from any thread,
            // but to be safe we use invoke_from_event_loop for a read -> we can also use upgrade_in_event_loop
            // here as previously, but keep pattern consistent:
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone_for_read.upgrade() {
                    let ui_stock = ui.get_current_stock().symbol;
                    // write into async Mutex via an async spawn — we can spawn a local tokio task
                    let cs = current_stock_clone.clone();
                    // spawn a tokio task to set the value (safe)
                    let ui_stock_str = ui_stock.to_string();
                    tokio::spawn(async move {
                        let mut s = cs.lock().await;
                        *s = ui_stock_str;
                    });
                }
            });

            // wait briefly to ensure current_stock updated (or read directly through a sync mechanism)
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;

            let symbol = current_stock.lock().await.clone();
            let mut pdf_data: Option<PdfReport> = None;

            {
                let cache_guard = cache.lock().await;
                if let Some(cached) = cache_guard.get(&symbol) {
                    pdf_data = Some(cached.clone());
                }
            }

            let mut prev = pre_stock.lock().await;
            if *prev != symbol {
                *prev = symbol.clone();

                if pdf_data.is_none() {
                    match fetch_finance_report_pdf(&symbol).await {
                        Ok(api_data) => {
                            pdf_data = Some(api_data.clone());
                            cache.lock().await.insert(symbol.clone(), api_data);
                        }
                        Err(e) => {
                            log::error!("❌ Failed to fetch PDF for {symbol}: {e}");
                            continue;
                        }
                    }
                }

                if let Some(data) = pdf_data {
                    // 🔹 Render PDF -> danh sách đường dẫn PNG (thao tác heavy, chạy ở background)
                    match render_pdf_to_png_paths(&data.file_path).await {
                        Ok(png_paths) => {
                            // png_paths: Vec<String> (Send) -> an toàn gửi vào UI thread
                            let ui_handle_clone = ui_handle.clone();
                            let symbol_clone = symbol.clone();
                            let png_paths_clone = png_paths.clone();

                            // invoke_from_event_loop sẽ chạy closure trên UI thread.
                            // Trong closure, load từng PNG thành slint::Image (Image::load_from_path)
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(ui) = ui_handle_clone.upgrade() {
                                    let mut slint_images: Vec<slint::Image> = Vec::new();
                                    for p in png_paths_clone.iter() {
                                        match slint::Image::load_from_path(std::path::Path::new(p)) {
                                            Ok(img) => slint_images.push(img),
                                            Err(err) => {
                                                log::warn!("⚠️ Failed to load image {}: {:?}", p, err);
                                            }
                                        }
                                    }
                                    // Wrap into ModelRc và set lên UI
                                    let model: ModelRc<Image> = ModelRc::new(VecModel::from(slint_images));
                                    ui.set_pdf_pages(model);
                                    log::info!("✅ Loaded and rendered PDF for {}", symbol_clone);
                                } else {
                                    log::warn!("UI handle could not be upgraded to set PDF pages");
                                }
                            });
                        }
                        Err(e) => {
                            log::error!("❌ Failed to render PDF for {symbol}: {e}");
                        }
                    }
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    task_handle
}

pub async fn spawn_finance_pdf_selected_task(ui: &AppWindow) -> TaskHandle {
    use crate::tasks::task_manager::{TaskStatus, register_task};
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::Mutex;

    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(5);

    let task_handle = register_task(
        "chart.finance_report_selected.pdf".to_string(),
        tx,
        "Finance PDF selected Fetcher".to_string(),
    )
    .await;

    tokio::spawn(async move {
        let mut task_status = TaskStatus::Running;
        let cache = Arc::new(Mutex::new(HashMap::<String, PdfReport>::new()));
        let current_report_id = Arc::new(Mutex::new(String::new()));
        let prev_report_id = Arc::new(Mutex::new(String::new()));

        loop {
            // 📩 Kiểm tra tín hiệu start/stop từ TaskManager
            if let Ok(status) = rx.try_recv() {
                task_status = status;
            }

            if task_status != TaskStatus::Running {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                continue;
            }

            // 🧭 Đọc report_id hiện tại từ UI (qua invoke_from_event_loop)
            let ui_handle_clone = ui_handle.clone();
            let report_ref = Arc::clone(&current_report_id);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone.upgrade() {
                    let report_id = ui.get_selected_report_id(); // ví dụ: property <string> selected_report_id
                    let rid_str = report_id.to_string();
                    let r = report_ref.clone();
                    tokio::spawn(async move {
                        let mut r_lock = r.lock().await;
                        *r_lock = rid_str;
                    });
                }
            });

            // chờ UI cập nhật
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;

            let report_id = current_report_id.lock().await.clone();
            if report_id.is_empty() {
                // Không có report được chọn → không làm gì
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }

            let mut prev = prev_report_id.lock().await;
            if *prev == report_id {
                // Trùng report → bỏ qua
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }

            *prev = report_id.clone();

            // 🔎 Kiểm tra cache
            let mut pdf_data: Option<PdfReport> = {
                let c = cache.lock().await;
                c.get(&report_id).cloned()
            };

            // 🧠 Nếu chưa có cache → fetch từ API
            if pdf_data.is_none() {
                match fetch_finance_report_pdf(&report_id).await {
                    Ok(api_data) => {
                        pdf_data = Some(api_data.clone());
                        cache.lock().await.insert(report_id.clone(), api_data);
                    }
                    Err(e) => {
                        log::error!("❌ Failed to fetch PDF for report {}: {}", report_id, e);
                        continue;
                    }
                }
            }

            // 🖼️ Render PDF -> PNGs -> push vào UI
            if let Some(data) = pdf_data {
                match render_pdf_to_png_paths(&data.file_path).await {
                    Ok(png_paths) => {
                        let ui_handle_clone = ui_handle.clone();
                        let report_id_clone = report_id.clone();
                        let pngs = png_paths.clone();

                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_handle_clone.upgrade() {
                                let mut slint_images: Vec<slint::Image> = Vec::new();
                                for p in pngs.iter() {
                                    match slint::Image::load_from_path(std::path::Path::new(p)) {
                                        Ok(img) => slint_images.push(img),
                                        Err(err) => {
                                            log::warn!("⚠️ Failed to load image {}: {:?}", p, err);
                                        }
                                    }
                                }
                                let model: ModelRc<Image> = ModelRc::new(VecModel::from(slint_images));
                                ui.set_pdf_pages(model);
                                log::info!("✅ Rendered PDF for report {}", report_id_clone);
                            }
                        });
                    }
                    Err(e) => log::error!("❌ Failed to render PDF for report {}: {}", report_id, e),
                }
            }

            // delay nhẹ để tránh spam
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    task_handle
}
