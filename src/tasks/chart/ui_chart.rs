use crate::slint_generatedAppWindow::AppWindow;
use crate::tasks::task_manager::{register_task, TaskHandle};
use crate::tasks::ChartMetaData;
use aim_chart::UiData;
use slint::ComponentHandle;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Converts UI data from Slint to chart rendering format
fn convert_ui_data_to_chart_data(ui_data: crate::slint_generatedAppWindow::UiData) -> UiData {
    // Map Slint's MouseType to aim_chart's MouseType explicitly to avoid requiring a From/Into impl.
    let mouse_type = match ui_data.r#type {
        crate::slint_generatedAppWindow::MouseType::Move => aim_chart::MouseType::Move,
        crate::slint_generatedAppWindow::MouseType::Draw => aim_chart::MouseType::Draw,
        crate::slint_generatedAppWindow::MouseType::Rectangle => aim_chart::MouseType::Rectangle,
        crate::slint_generatedAppWindow::MouseType::Oval => aim_chart::MouseType::Oval,
        crate::slint_generatedAppWindow::MouseType::Line => aim_chart::MouseType::Line,
        crate::slint_generatedAppWindow::MouseType::Arrow => aim_chart::MouseType::Arrow,
        crate::slint_generatedAppWindow::MouseType::Ruler => aim_chart::MouseType::Ruler,
        crate::slint_generatedAppWindow::MouseType::Text => aim_chart::MouseType::Text,
        crate::slint_generatedAppWindow::MouseType::VerticalLine => {
            aim_chart::MouseType::VerticalLine
        }
        crate::slint_generatedAppWindow::MouseType::HorizontalLine => {
            aim_chart::MouseType::HorizontalLine
        }
    };

    UiData {
        ticker: ui_data.ticker.to_string(),
        width: ui_data.width,
        height: ui_data.height,
        is_in_update: ui_data.is_in_update,
        is_in_object: ui_data.is_in_object,
        is_clean: ui_data.is_clean,
        is_undo: ui_data.is_undo,
        is_release: ui_data.is_release,
        move_x: ui_data.move_x,
        move_y: ui_data.move_y,
        zoom: ui_data.zoom,
        mouse_type,
        position_x: ui_data.position_x,
        press_x: ui_data.press_x,
        position_y: ui_data.position_y,
        press_y: ui_data.press_y,
        time_frame: ui_data.time_frame.to_string(),
        is_new_time_frame: ui_data.is_new_time_frame,
        is_new_stock: ui_data.is_new_stock,
        color: ui_data.color,
    }
}

/// Spawns a task to handle real-time chart rendering
pub async fn spawn_ui_chart_task(chart: Arc<Mutex<ChartMetaData>>, ui: &AppWindow) -> TaskHandle {
    let chart_clone = Arc::clone(&chart);
    let ui_handle = ui.as_weak();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let task_handle = register_task(
        "chart.ui_chart".to_string(),
        tx,
        "UI Chart Rendering Task".to_string(),
    )
    .await;

    tokio::task::spawn_blocking(move || {
        // This closure runs in a blocking context to avoid blocking the async runtime
        let mut task_status = crate::tasks::task_manager::TaskStatus::Running;
        loop {
            if let Ok(status) = rx.try_recv() {
                if task_status != status {
                    log::info!("Cache storage task status changed to: {:?}", status);
                    task_status = status;
                }
            }
            if task_status != crate::tasks::task_manager::TaskStatus::Running {
                std::thread::sleep(std::time::Duration::from_millis(5));
                continue;
            }
            let chart_clone = Arc::clone(&chart_clone);
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let mut ui_data = ui.get_ui_data();
                let previous_ui_data = ui.get_previous_ui_data();
                let stock_name = ui.get_current_stock().symbol;
                let height = ui.invoke_get_chart_height();
                let width = ui.invoke_get_chart_width();

                if previous_ui_data != ui_data
                    || stock_name != ui_data.ticker
                    || height as i32 != ui_data.height
                    || width as i32 != ui_data.width
                    || ui_data.is_in_update
                {
                    ui_data.height = height as i32;
                    ui_data.width = width as i32;

                    // Render the chart plot
                    let (image, is_in_object) = {
                        ui_data.is_in_update = false;
                        log::info!("Rendering chart for {stock_name}");
                        tokio::task::block_in_place(|| {
                            let mut charts = chart_clone.blocking_lock();
                            if let Some(chart) = charts
                                .data
                                .iter_mut()
                                .find(|chart| stock_name == chart.stock_name)
                            {
                                log::info!("Found existing chart for {stock_name}, rendering...");
                                ui_data.ticker = stock_name.clone();
                                let chart_ui_data = convert_ui_data_to_chart_data(ui_data.clone());
                                chart.render_plot(chart_ui_data)
                            } else {
                                log::warn!("No chart found for {stock_name}, using default");
                                (slint::Image::default(), false)
                            }
                        })
                    };

                    // Update UI with new chart image
                    ui.set_candle_stick_image(image);
                    ui_data.is_in_object = is_in_object;

                    // Reset UI state flags
                    if ui_data.is_clean {
                        ui_data.is_clean = false;
                    }
                    if ui_data.is_undo {
                        ui_data.is_undo = false;
                    }
                    if ui_data.is_release {
                        ui_data.move_x = 0;
                        ui_data.move_y = 0;
                    }
                    ui_data.zoom = 0;
                    ui.set_ui_data(ui_data.clone());
                    ui.set_previous_ui_data(ui_data);
                }
            });
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    });

    task_handle
}
