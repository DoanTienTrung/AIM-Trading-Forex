// Include all Slint UI modules
slint::include_modules!();

// Import required modules
mod task_manager;
mod tasks;
use crate::{
    slint_generatedAppWindow::StockData as SlintStockData,
    tasks::{
        build_stream_paths_for_range, sort_market_watch, sort_stocks, spawn_cache_storage_task,
        ChartMetaData, ALL_STOCK_LIST,
    },
};
use aim_chart::Chart;
use aim_data::{get_company_info, get_market_watch, get_quote};
use dirs_next::cache_dir;
use slint::{Model, ModelRc, SharedString, VecModel};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use aim_data::aim::CorrelationMatrixAPI;

// Import task functions
use tasks::{
    convert_to_stock_data, spawn_abnormal_trade_task, spawn_balance_sheet_task,
    spawn_chart_update_task, spawn_company_profile_task, spawn_data_update_task,
    spawn_heat_map_task, spawn_icb_index_task, spawn_mini_chart_hnx30_task,
    spawn_mini_chart_hnxindex_task, spawn_mini_chart_vn30_task, spawn_mini_chart_vnindex_task,
    spawn_overall_index_task, spawn_sjc_price_task, spawn_stock_influence_task,
    spawn_stock_update_task, spawn_trading_volume_task, spawn_ui_chart_task,
    spawn_finance_report_task, spawn_finance_pdf_selected_task, render_pdf_to_png_paths,
    spawn_rsi_task,
    register_rsi_sort_handler,
    spawn_top10_market_cap_task, spawn_icb_index_filtered_task,
    spawn_forex_task
};
use aim_data::aim::fetch_finance_report_pdf;
// use crate::tasks::render_pdf_to_png_paths;


static MY_STOCK_LIST: [&str; 1] = ["AAA"];


/// Main entry point of the application
/// This function sets up the trading application with real-time stock data visualization
// #[tokio::main]
// async fn main() {
//     env_logger::Builder::new()
//         .filter_level(log::LevelFilter::Warn)
//         .init();

//     let ui = slint_generatedAppWindow::AppWindow::new().unwrap();
//     register_rsi_sort_handler(&ui);
//     let _rsi_task = spawn_rsi_task(&ui).await;
//     ui.run().unwrap();
// }

#[tokio::main]
async fn main() {
    //env_logger::init();
    #[cfg(target_os = "windows")]
    unsafe {
        winapi::um::wincon::FreeConsole(); // This detaches the console from the application
    }
    // Initialize logging with Info level
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Error)
        .init();

    let base_cache = cache_dir().expect("Could not find cache directory");
    let app_cache_dir = base_cache.join("Aim");
    std::fs::create_dir_all(&app_cache_dir).unwrap();
    let cache_file: PathBuf = app_cache_dir.join("cache.bin");
    let user_list: PathBuf = app_cache_dir.join("user_list.json");

    // Fetch initial chart data for default stock (AAA)
    let chart_data = get_quote(&["AAA"], "ONE_DAY", None, None).await.unwrap();
    let company_info = get_company_info("AAA").await.unwrap();
    let company_info = aim_chart::CompanyInfo {
        roe: company_info.data.company_financial_ratio.ratio[0]
            .roe
            .unwrap_or(0.0),
        roa: company_info.data.company_financial_ratio.ratio[0]
            .roa
            .unwrap_or(0.0),
        pe: company_info.data.company_financial_ratio.ratio[0]
            .pe
            .unwrap_or(0.0),
        pb: company_info.data.company_financial_ratio.ratio[0]
            .pb
            .unwrap_or(0.0),
        eps: company_info.data.company_financial_ratio.ratio[0]
            .eps
            .unwrap_or(0.0),
    };

    // Create a thread-safe chart container with initial chart
    let chart_metadata = if std::fs::metadata(&cache_file).is_ok() {
        ChartMetaData::load(&cache_file)
    } else {
        ChartMetaData::new(vec![Chart::new_default(
            "AAA".to_string(),
            chart_data.0[0].clone(),
            company_info,
        )])
    };
    let chart = Arc::new(Mutex::new(chart_metadata));

    let init_data = match get_market_watch(&["AAA"]).await {
        Ok(stock_list_data) => {
            let stock_data: Vec<SlintStockData> = stock_list_data
                .0
                .iter()
                .map(convert_to_stock_data)
                .collect();
            stock_data[0].clone()
        }
        Err(e) => {
            log::error!("Failed to fetch stock data: {e}.");
            SlintStockData::default()
        }
    };

    // Initialize the main UI window
    let ui = slint_generatedAppWindow::AppWindow::new().unwrap();

    // ------------------------------
    // Khởi tạo dữ liệu Streamgraph phân bổ vốn hóa theo ngành
    // ------------------------------
    let make_timeline = |w1: (f32, f32, f32, f32), w2: (f32, f32, f32, f32), w3: (f32, f32, f32, f32)| {
        vec![
            SectorTimePoint {
                label: SharedString::from("Dữ liệu tuần 1"),
                finance: w1.0,
                real_estate: w1.1,
                consumer: w1.2,
                others: w1.3,
            },
            SectorTimePoint {
                label: SharedString::from("Dữ liệu tuần 2"),
                finance: w2.0,
                real_estate: w2.1,
                consumer: w2.2,
                others: w2.3,
            },
            SectorTimePoint {
                label: SharedString::from("Dữ liệu tuần 3"),
                finance: w3.0,
                real_estate: w3.1,
                consumer: w3.2,
                others: w3.3,
            },
        ]
    };

    let mut ranges: Vec<SectorRangeSnapshot> = Vec::new();

    // 1T
    {
        // Tuần 1 lấy đúng % hiển thị, tuần 2–3 dao động mạnh hơn quanh trung bình
        let base = (42.5_f32, 18.2_f32, 15.8_f32, 23.5_f32); // dùng cho label hiển thị
        let tl = make_timeline(
            (42.5, 18.2, 15.8, 23.5), // tuần 1
            (47.0, 15.0, 13.0, 25.0), // tuần 2 – tài chính tăng mạnh, BĐS & tiêu dùng giảm
            (38.0, 22.0, 18.0, 22.0), // tuần 3 – tài chính giảm, BĐS & tiêu dùng tăng
        ); // mỗi tuần tổng ≈ 100
        let timeline_model: ModelRc<SectorTimePoint> = ModelRc::new(VecModel::from(tl));
        let mut range = SectorRangeSnapshot {
            label: SharedString::from("1T"),
            finance_path: SharedString::default(),
            real_estate_path: SharedString::default(),
            consumer_path: SharedString::default(),
            others_path: SharedString::default(),
            finance_percent: base.0,
            real_estate_percent: base.1,
            consumer_percent: base.2,
            others_percent: base.3,
            timeline: timeline_model,
        };
        let (f, re, c, o) = build_stream_paths_for_range(&range, 800.0, 600.0);
        range.finance_path = f.into();
        range.real_estate_path = re.into();
        range.consumer_path = c.into();
        range.others_path = o.into();
        ranges.push(range);
    }

    // 3T
    {
        let base = (41.3_f32, 19.0_f32, 16.2_f32, 23.5_f32);
        let tl = make_timeline(
            (41.3, 19.0, 16.2, 23.5),
            (36.0, 23.0, 18.5, 22.5),
            (45.0, 15.0, 14.0, 26.0),
        );
        let timeline_model: ModelRc<SectorTimePoint> = ModelRc::new(VecModel::from(tl));
        let mut range = SectorRangeSnapshot {
            label: SharedString::from("3T"),
            finance_path: SharedString::default(),
            real_estate_path: SharedString::default(),
            consumer_path: SharedString::default(),
            others_path: SharedString::default(),
            finance_percent: base.0,
            real_estate_percent: base.1,
            consumer_percent: base.2,
            others_percent: base.3,
            timeline: timeline_model,
        };
        let (f, re, c, o) = build_stream_paths_for_range(&range, 800.0, 600.0);
        range.finance_path = f.into();
        range.real_estate_path = re.into();
        range.consumer_path = c.into();
        range.others_path = o.into();
        ranges.push(range);
    }

    // 1N
    {
        let base = (40.4_f32, 20.1_f32, 15.0_f32, 24.5_f32);
        let tl = make_timeline(
            (40.4, 20.1, 15.0, 24.5),
            (44.0, 17.0, 14.0, 25.0),
            (37.0, 23.0, 17.5, 22.5),
        );
        let timeline_model: ModelRc<SectorTimePoint> = ModelRc::new(VecModel::from(tl));
        let mut range = SectorRangeSnapshot {
            label: SharedString::from("1N"),
            finance_path: SharedString::default(),
            real_estate_path: SharedString::default(),
            consumer_path: SharedString::default(),
            others_path: SharedString::default(),
            finance_percent: base.0,
            real_estate_percent: base.1,
            consumer_percent: base.2,
            others_percent: base.3,
            timeline: timeline_model,
        };
        let (f, re, c, o) = build_stream_paths_for_range(&range, 800.0, 600.0);
        range.finance_path = f.into();
        range.real_estate_path = re.into();
        range.consumer_path = c.into();
        range.others_path = o.into();
        ranges.push(range);
    }

    // Tất cả
    {
        let base = (39.8_f32, 21.0_f32, 14.8_f32, 24.4_f32);
        let tl = make_timeline(
            (39.8, 21.0, 14.8, 24.4),
            (35.0, 24.5, 17.0, 23.5),
            (43.0, 18.5, 13.0, 25.5),
        );
        let timeline_model: ModelRc<SectorTimePoint> = ModelRc::new(VecModel::from(tl));
        let mut range = SectorRangeSnapshot {
            label: SharedString::from("Tất cả"),
            finance_path: SharedString::default(),
            real_estate_path: SharedString::default(),
            consumer_path: SharedString::default(),
            others_path: SharedString::default(),
            finance_percent: base.0,
            real_estate_percent: base.1,
            consumer_percent: base.2,
            others_percent: base.3,
            timeline: timeline_model,
        };
        let (f, re, c, o) = build_stream_paths_for_range(&range, 800.0, 600.0);
        range.finance_path = f.into();
        range.real_estate_path = re.into();
        range.consumer_path = c.into();
        range.others_path = o.into();
        ranges.push(range);
    }

    ui.set_quantitative_sector_ranges(ModelRc::new(VecModel::from(ranges)));

    // Initialize page-aware task manager
    task_manager::initialize_page_manager(&ui).await;
    log::info!("Page-aware task manager initialized");

    let default_user_list = Arc::new(Mutex::new(
        MY_STOCK_LIST
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    ));
    // Initialize the symbol list with initial values
    let symbol_list = if std::fs::metadata(&user_list).is_ok() {
        if let Ok(custom_list_data) = std::fs::read_to_string(&user_list) {
            if let Ok(custom_list) = serde_json::from_str::<Vec<String>>(&custom_list_data) {
                Arc::new(Mutex::new(custom_list))
            } else {
                log::error!("Failed to parse custom_list.json");
                default_user_list.clone()
            }
        } else {
            default_user_list.clone()
        }
    } else {
        default_user_list
    };

    ui.set_current_stock(init_data);

    // Set up callback for adding symbols
    let symbol_list_clone = Arc::clone(&symbol_list);
    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();
    ui.on_add_stock(move |group_name: SharedString, symbol: SharedString| {
        if group_name == "MY LIST" {
            let symbol = symbol.to_uppercase();
            let symbol_list_clone = symbol_list_clone.clone();
            ui_handle.unwrap().set_is_list_in_update(true);
            let ui_handle = ui_handle.clone();
            tokio::spawn(async move {
                if !ALL_STOCK_LIST.contains(&symbol.as_str()) {
                    log::error!("Failed to add stock: {symbol} - not found in ALL_STOCK_LIST");
                } else {
                    let mut list = symbol_list_clone.lock().await;
                    if !list.contains(&symbol.to_string()) {
                        list.push(symbol.to_string());
                    } else {
                        log::warn!("Stock {symbol} already exists in the list {list:?}");
                    }
                }
                ui_handle.upgrade_in_event_loop(move |ui| {
                    ui.set_is_list_in_update(false);
                })
            });
        }
    });

    // Set up callback for removing symbols
    let symbol_list_clone = Arc::clone(&symbol_list);
    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();
    ui.on_remove_stock(move |group_name: SharedString, symbol: SharedString| {
        if group_name == "MY LIST" {
            let symbol = symbol.to_uppercase();
            let symbol_list_clone = symbol_list_clone.clone();
            ui_handle.unwrap().set_is_list_in_update(true);
            let ui_handle = ui_handle.clone();
            tokio::spawn(async move {
                let mut list = symbol_list_clone.lock().await;
                list.retain(|s| s != &symbol.to_string());
                ui_handle.upgrade_in_event_loop(move |ui| {
                    ui.set_is_list_in_update(false);
                })
            });
        }
    });

    // Set up callback for toggling group expansion
    ui.on_toggle_group(move |group_idx: i32| {
        log::info!("Toggling group {group_idx}");
        // For now, just log the group toggle - you can implement actual state management here
        // In a real implementation, you might want to store group expansion state
    });

    // Set up callback for switching watchlists
    let ui_handle_clone = ui.as_weak();
    ui.on_switch_list(move |list_name: slint::SharedString| {
        log::info!("Switching to watchlist: {list_name}");
        let ui_handle = ui_handle_clone.clone();

        // Trigger data update with new category filter
        tokio::spawn(async move {
            // You can implement specific logic here to filter stocks by category
            // For now, we'll just log the switch
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                ui.set_is_list_in_update(true);
                // In a real implementation, you would filter the data here
                ui.set_is_list_in_update(false);
            });
        });
    });

    let ui_handle = ui.as_weak();
    ui.on_sort_stocks(move |sort_type| {
        let ui_handle_clone = ui_handle.clone();
        tokio::spawn(async move {
            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                let stock_groups = ui.get_stock_groups();
                let stock_groups_vec: Vec<_> = (0..stock_groups.row_count())
                    .map(|i| stock_groups.row_data(i).unwrap())
                    .collect();
                let updated_groups = sort_stocks(&stock_groups_vec, sort_type);

                // Update the UI with sorted groups
                ui.set_stock_groups(slint::ModelRc::new(VecModel::from(updated_groups)));
            });
        });
    });

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

    let ui_handle_selected_pdf = ui.as_weak();
    ui.on_report_selected(move |report_id| {
        let ui_handle_clone = ui_handle_selected_pdf.clone();
        tokio::spawn(async move {
            // Log
            log::info!("[LOI]🟢 User selected report_id = {}", report_id);

            // 🟡 Báo cho UI biết đang tải
            let _ = ui_handle_clone.upgrade_in_event_loop(|ui| {
                ui.set_is_loading(true);
            });

            // Gọi API và render ngay ở đây (bỏ qua task nền)
            match fetch_finance_report_pdf(&report_id).await {
                Ok(pdf) => {
                    log::info!("[LOI]📄 Đã tải PDF báo cáo cho report_id={} -> {}", report_id, pdf.file_path);

                    match render_pdf_to_png_paths(&pdf.file_path).await {
                        Ok(png_paths) => {
                            log::info!("[LOI]🖼️ Render PDF -> PNG thành công: {} trang", png_paths.len());
                            for (i, path) in png_paths.iter().enumerate() {
                                log::debug!("[LOI] ↳ Trang {} -> {}", i + 1, path);
                            }

                            // Đưa kết quả lên UI thread
                            let _ = ui_handle_clone.upgrade_in_event_loop(move |ui| {
                                let mut slint_images: Vec<slint::Image> = Vec::new();

                                for p in png_paths.iter() {
                                    match slint::Image::load_from_path(std::path::Path::new(p)) {
                                        Ok(img) => {
                                            slint_images.push(img);
                                            log::debug!("[LOI]✅ Load ảnh thành công: {}", p);
                                        }
                                        Err(_) => {
                                            log::warn!("[LOI]⚠️ Không thể load ảnh {}", p);
                                        }
                                    }
                                }

                                let model: slint::ModelRc<slint::Image> =
                                    slint::ModelRc::new(slint::VecModel::from(slint_images));

                                ui.set_pdf_pages(model);
                                ui.set_selected_report_id(report_id.clone());
                                ui.set_is_loading(false); // 🟢 Tắt loading

                                log::info!("[LOI]✅ UI đã cập nhật báo cáo PDF cho report_id={}", report_id);
                            });
                        }
                        Err(e) => {
                            log::error!("[LOI]❌ Lỗi khi render PDF -> PNG: {:?}", e);
                            let _ = ui_handle_clone.upgrade_in_event_loop(|ui| ui.set_is_loading(false));
                        }
                    }
                }
                Err(e) => {
                    log::error!("❌ Lỗi khi tải PDF từ API cho report_id={}: {:?}", report_id, e);
                    let _ = ui_handle_clone.upgrade_in_event_loop(|ui| ui.set_is_loading(false));
                }
            }

        });
    });

    register_rsi_sort_handler(&ui);


    // Spawn all the tasks
    let _ui_chart_handle = spawn_ui_chart_task(Arc::clone(&chart), &ui).await;
    // If you only want to read the chart data, you can pass a reference to the Arc<Mutex<ChartMetaData>>
    // Spawn cache storage task with task manager
    let _cache_handle =
        spawn_cache_storage_task(Arc::clone(&chart), Arc::clone(&symbol_list)).await;
    let _stock_update_handles = spawn_stock_update_task(Arc::clone(&chart), &ui).await;
    let _chart_update_handle = spawn_chart_update_task(Arc::clone(&chart)).await;
    let _data_update_handle = spawn_data_update_task(&ui, Arc::clone(&symbol_list)).await;
    let _balance_sheet_handles = spawn_balance_sheet_task(&ui).await;
    let _company_profile_handles = spawn_company_profile_task(&ui).await;
    let _mini_vnindex_handle = spawn_mini_chart_vnindex_task(&ui).await;
    let _mini_vn30_handle = spawn_mini_chart_vn30_task(&ui).await;
    let _mini_hnx30_handle = spawn_mini_chart_hnx30_task(&ui).await;
    let _mini_hnxindex_handle = spawn_mini_chart_hnxindex_task(&ui).await;
    // spawn_world_index_task(&ui);
    let _stock_influence_handle = spawn_stock_influence_task(&ui).await;
    let _overall_index_handle = spawn_overall_index_task(&ui).await; // update OverallIndex UI component
    let _heat_map_handle = spawn_heat_map_task(&ui).await; // update HeatMap UI component
    let _icb_index_handle = spawn_icb_index_task(&ui).await; // update ICBIndex UI component
    let _abnormal_trade_handle = spawn_abnormal_trade_task(&ui).await; // update AbnormalTrade UI component
    let _trading_volume_handle = spawn_trading_volume_task(&ui).await; // update TradingVolume UI component
    let _sjc_price_handle = spawn_sjc_price_task(&ui).await; // update SJC price data for goods UI component
    // Gọi task xử lý Finance Report
    let _finance_report_task = spawn_finance_report_task(&ui).await; // update finance report data for goods UI component
    let _finance_pdf_selected_task = spawn_finance_pdf_selected_task(&ui).await; // update finance report data for goods UI component

    // Spawn return matrix cache preload task
    let return_matrix_cache = tasks::spawn_return_matrix_cache_task().await;
    log::info!("Return matrix cache task spawned");

    // MP layout
    let _rsi_task = spawn_rsi_task(&ui).await;

    // Forex
    let _forex_task = spawn_forex_task(&ui).await;
    log::info!("Forex task spawned");

    // Gọi task fetch dữ liệu Market Cap cho Quantitative Analysis
    let _top10_market_cap_handle = spawn_top10_market_cap_task(&ui).await; // fetch Top 10 Market Cap data
    let _icb_index_filtered_handle = spawn_icb_index_filtered_task(&ui).await; // fetch ICB Index filtered (industry_code 2 digits)

    // Xử lý callback thêm tag cho phân tích định lượng
    let ui_tags_handle = ui.as_weak();
    ui.on_on_add_tag(move |tag: slint::SharedString| {
        let _ = ui_tags_handle.upgrade_in_event_loop(move |ui| {
            let tags = ui.get_stock_tags();
            let tag_str = tag.to_string().to_uppercase();
            let mut tags_vec: Vec<_> = (0..tags.row_count()).map(|i| tags.row_data(i).unwrap()).collect();
            if !tags_vec.iter().any(|t| t == &tag_str) {
                tags_vec.push(slint::SharedString::from(tag_str));
                ui.set_stock_tags(slint::ModelRc::new(slint::VecModel::from(tags_vec)));
            }
        });
    });

    // Xử lý callback xóa tag cho phân tích định lượng
    let ui_tags_handle = ui.as_weak();
    ui.on_on_remove_tag(move |index: i32| {
        let _ = ui_tags_handle.upgrade_in_event_loop(move |ui| {
            let tags = ui.get_stock_tags();
            let idx = index as usize;
            let mut tags_vec: Vec<_> = (0..tags.row_count()).map(|i| tags.row_data(i).unwrap()).collect();
            if idx < tags_vec.len() {
                tags_vec.remove(idx);
                ui.set_stock_tags(slint::ModelRc::new(slint::VecModel::from(tags_vec)));
            }
        });
    });
    //Xu lý callback tính toán ma trận tương quan
    let ui_correlation_handle = ui.as_weak();
    ui.on_calculateCorrelation(move |type_str: slint::SharedString| {
        let ui_handle = ui_correlation_handle.clone();

        // Kích hoạt loading ngay trong UI thread
        let _ = ui_handle.upgrade_in_event_loop(|ui| {
            ui.set_is_loading(true);
        });

        // Clone dạng String để chuyển vào tokio task (Send)
        let mut type_str = type_str.to_string();
        if type_str == "VN30" {
            type_str = "vn30-correlation".to_string();
        } else if type_str == "BANK" {
            type_str = "bank-correlation".to_string();
        } 
        else {
            type_str = "securities-correlation".to_string();
        }

        // Spawn thread tính toán
        tokio::spawn(async move {
            match aim_data::aim::fetch_correlation_matrix(&type_str).await {
                Ok(correlation_data) => {
                    log::info!("Fetched correlation matrix data: {} entries", correlation_data.len());

                    let (correlation_rows, correlation_columns) =
                        parse_correlation_matrix(&correlation_data);

                    let rows_vec = correlation_rows;
                    let cols_vec = correlation_columns;

                    let ui_handle2 = ui_handle.clone();
                    let _ = ui_handle2.upgrade_in_event_loop(move |ui| {
                        let row_model = slint::VecModel::from(
                            rows_vec.iter().map(|row| {
                                slint_generatedAppWindow::CorrelationRow {
                                    ticker: slint::SharedString::from(row.ticker.clone()),
                                    values: slint::ModelRc::new(slint::VecModel::from(row.values.clone())),
                                }
                            }).collect::<Vec<_>>()
                        );
                        let col_shared: Vec<slint::SharedString> = cols_vec.iter().map(|s| slint::SharedString::from(s.clone())).collect();
                        let col_model = slint::VecModel::from(col_shared);

                        ui.set_correlation_rows(slint::ModelRc::new(row_model));
                        ui.set_correlation_columns(slint::ModelRc::new(col_model));

                        ui.set_is_loading(false);
                    });
                }

                Err(e) => {
                    log::error!("Error fetching correlation matrix: {:?}", e);

                    let ui_handle2 = ui_handle.clone();
                    let _ = ui_handle2.upgrade_in_event_loop(|ui| {
                        ui.set_is_loading(false);
                    });
                }
            }
        });
    });
    //Xu lý callback tính toán ma trận lợi nhuận từ cache
    let ui_return_matrix_handle = ui.as_weak();
    let return_matrix_cache_clone = Arc::clone(&return_matrix_cache);
    ui.on_getReturnMatrix(move |period: slint::SharedString, group: slint::SharedString, month: slint::SharedString| {
        let ui_handle = ui_return_matrix_handle.clone();
        let cache = Arc::clone(&return_matrix_cache_clone);

        // Clone dạng String để chuyển vào tokio task (Send)
        let period_str = period.to_string();
        let group_str = group.to_string();
        let month_str = month.to_string();

        // Spawn thread lấy dữ liệu từ cache
        tokio::spawn(async move {
            // Try to get from cache first
            if let Some(cached_data) = tasks::get_cached_return_matrix(&cache, &period_str, &group_str, &month_str).await {
                log::info!("Return matrix data loaded from cache: {}_{} ({})", period_str, group_str, month_str);
                
                let rows_vec = cached_data.rows;
                
                let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                    let row_model = slint::VecModel::from(
                        rows_vec.iter().map(|row| {
                            slint_generatedAppWindow::ReturnRowData {
                                time: slint::SharedString::from(row.ticker.clone()),
                                values: slint::ModelRc::new(slint::VecModel::from(row.values.clone())),
                            }
                        }).collect::<Vec<_>>()
                    );

                    ui.set_return_row_data(slint::ModelRc::new(row_model));
                    log::info!("Updated return matrix with {} rows from cache", rows_vec.len());
                });
            } else {
                // If not in cache, fetch from API and update cache
                log::warn!("Return matrix data not in cache, fetching from API: {}_{} ({})", period_str, group_str, month_str);
                
                let _ = ui_handle.upgrade_in_event_loop(|ui| {
                    ui.set_is_loading(true);
                });

                match aim_data::aim::fetch_return_matrix(&period_str, &group_str).await {
                    Ok(return_data) => {
                        let (return_rows, return_columns) =
                            tasks::parse_return_matrix(&return_data, &period_str, &month_str);

                        // Update cache
                        let key = tasks::ReturnMatrixKey {
                            period: period_str.clone(),
                            group: group_str.clone(),
                            month: if period_str == "daily" { month_str.clone() } else { String::new() },
                        };
                        let cache_value = tasks::ReturnMatrixCache {
                            rows: return_rows.clone(),
                            columns: return_columns,
                        };
                        cache.write().await.insert(key, cache_value);

                        let rows_vec = return_rows;

                        let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                            let row_model = slint::VecModel::from(
                                rows_vec.iter().map(|row| {
                                    slint_generatedAppWindow::ReturnRowData {
                                        time: slint::SharedString::from(row.ticker.clone()),
                                        values: slint::ModelRc::new(slint::VecModel::from(row.values.clone())),
                                    }
                                }).collect::<Vec<_>>()
                            );

                            ui.set_return_row_data(slint::ModelRc::new(row_model));
                            log::info!("Updated return matrix with {} rows", rows_vec.len());
                            ui.set_is_loading(false);
                        });
                    }
                    Err(e) => {
                        log::error!("Error fetching return matrix: {:?}", e);
                        let _ = ui_handle.upgrade_in_event_loop(|ui| {
                            ui.set_is_loading(false);
                        });
                    }
                }
            }
        });
    });

    // Start active page monitoring after all tasks are spawned
    task_manager::start_page_monitoring(&ui).await;
    log::info!("Active page monitoring started - tasks will automatically pause/resume based on UI navigation");

    // Set up window close handler
    ui.window().on_close_requested(|| {
        log::info!("Closing the application...");
        std::process::exit(0);
    });

    // Run the UI main loop
    ui.run().unwrap();
}

/// Converts user-friendly interval strings to API interval constants
///
/// # Arguments
/// * `interval` - The interval string (e.g., "1m", "5m", "1H", "1D")
///
/// # Returns
/// The corresponding API interval constant
fn interval_to_constant(interval: &str) -> &'static str {
    match interval {
        "1m" | "5m" | "15m" | "30m" => "ONE_MINUTE",
        "1H" | "2H" | "4H" => "ONE_HOUR",
        "1D" | "2D" | "3D" => "ONE_DAY",
        "1W" | "2W" | "1M" => "ONE_WEEK",
        _ => "ONE_DAY",
    }
}

#[derive(Clone)]
pub struct CorrelationRowPure {
    pub ticker: String,
    pub values: Vec<f32>,
}

// Parse API correlation data into matrix format for Slint UI
fn parse_correlation_matrix(api_data: &[CorrelationMatrixAPI])
    -> (Vec<CorrelationRowPure>, Vec<String>)
{
    use std::collections::{BTreeSet, HashMap};

    // Collect all unique tickers
    let mut tickers_set = BTreeSet::new();
    for entry in api_data {
        tickers_set.insert(entry.symbol_a.clone());
        tickers_set.insert(entry.symbol_b.clone());
    }
    let tickers: Vec<String> = tickers_set.into_iter().collect();

    // Columns: PURE Vec<String>
    let columns = tickers.clone();

    // Build index
    let mut ticker_index = HashMap::new();
    for (i, t) in tickers.iter().enumerate() {
        ticker_index.insert(t.clone(), i);
    }


    // PURE matrix
    let mut matrix = vec![vec![0.0f32; tickers.len()]; tickers.len()];

    for entry in api_data {
        let i = ticker_index[&entry.symbol_a];
        let j = ticker_index[&entry.symbol_b];
        let rounded = (entry.correlation_value * 100.0).round() / 100.0;
        matrix[i][j] = rounded;
        matrix[j][i] = rounded; // Ensure symmetry
    }

    // Set diagonal to 1 (self-correlation)
    for i in 0..tickers.len() {
        matrix[i][i] = 1.0;
    }

    // PURE rows
    let rows: Vec<CorrelationRowPure> = tickers
        .iter()
        .enumerate()
        .map(|(i, t)| CorrelationRowPure {
            ticker: t.clone(),
            values: matrix[i].clone(),
        })
        .collect();

    (rows, columns)
}
