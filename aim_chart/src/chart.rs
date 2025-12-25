use aim_data::OHLCData;
use chrono::{DateTime, Duration, Utc};
use log::debug;

use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    coord::types::RangedCoordf32,
    drawing::IntoDrawingArea,
    element::{CandleStick, PathElement, Polygon},
    prelude::{Cartesian2d, ChartContext, Rectangle, Text},
    series::LineSeries,
    style::{BLUE, Color, IntoFont, RED, RGBColor, ShapeStyle, WHITE, full_palette::GREY},
};
use slint::SharedPixelBuffer;

const UP_COLOR: RGBColor = RGBColor(0x2E, 0xBD, 0x85);
const DOWN_COLOR: RGBColor = RGBColor(0xF6, 0x46, 0x5D);
const NO_CHANGE_COLOR: RGBColor = RGBColor(0xFF, 0xEB, 0x3B);
const THREAD_HOLD: f32 = 2.0; // in pixel
const DEFAULT_CANDLE_NUMER: usize = 300; // default number of candles to show

use crate::{MouseType, UiData};

use super::{
    CandleData, CandleDataVec,
    company_info::CompanyInfo,
    convert_candlesticks,
    draw::{DrawObject, DrawType, Point},
};

#[derive(Debug, Clone)]
pub struct BackupChartOffset {
    pub y_offset_min: f32,
    pub y_offset_max: f32,
    pub x_offset_min: f32,
    pub x_offset_max: f32,
}

#[derive(Debug, Clone)]
pub struct ChartData {
    pub y_offset_min: f32,
    pub y_offset_max: f32,
    pub x_offset_min: f32,
    pub x_offset_max: f32,
    pub range_x: i32,         // range value of x axis
    pub range_y: i32,         // range value of y axis
    pub width: u32,           // the width of the chart in pixels
    pub height: u32,          // the height of the chart in pixels
    pub candle_distance: f32, // distance between 2 candle sticks
    pub zoom_x: f32,          // zoom factor in x direction
    pub zoom_y: f32,          // zoom factor in y direction
    pub ui_data: UiData,
}

impl Default for ChartData {
    fn default() -> Self {
        Self {
            y_offset_min: 0.0,
            y_offset_max: 0.0,
            x_offset_min: 0.0,
            x_offset_max: 0.0,
            range_x: 30,
            range_y: 60,
            width: 1200,
            height: 800,
            candle_distance: 0.0,
            zoom_x: 0.0,
            zoom_y: 0.0,
            ui_data: UiData::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chart {
    pub stock_name: String,
    pub company_info: CompanyInfo,
    pub candle_data: CandleDataVec, // Candle sticks data
    pub chart_data: ChartData,
    pub current_draw_data: DrawObject,
    pub moving_object: DrawObject,
    pub all_draw_data: Vec<DrawObject>,
    pub backup_points: Vec<Point>,
    pub backup_chart_offset: Option<BackupChartOffset>,
    pub is_in_object: (usize, bool, bool),
    pub delta: (f32, f32),
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
impl Chart {
    // new_dafault to set value default for inititalization
    pub fn new_default(stock_name: String, stock: OHLCData, company_info: CompanyInfo) -> Self {
        let data = convert_candlesticks(stock.symbol.len() == 3, stock.to_candlesticks());
        let (x_offset_min, data_slice) = if data.len() < DEFAULT_CANDLE_NUMER {
            (0 as f32, &data[0..(data.len() - 1)])
        } else {
            (
                (data.len() - DEFAULT_CANDLE_NUMER) as f32,
                &data[(data.len() - DEFAULT_CANDLE_NUMER)..(data.len() - 1)],
            )
        };
        let chart_data = ChartData {
            x_offset_min,
            x_offset_max: (data.len() - 1) as f32,
            y_offset_min: Self::get_y_min(data_slice.to_vec()),
            y_offset_max: Self::get_y_max(data_slice.to_vec()),
            ..Default::default()
        };

        Self {
            stock_name,
            company_info,
            candle_data: data.clone(),
            chart_data,
            moving_object: DrawObject::default(),
            current_draw_data: DrawObject::default(),
            all_draw_data: Vec::default(),
            backup_points: Vec::default(),
            backup_chart_offset: None,
            is_in_object: (0, false, false),
            delta: (0.0, 0.0),
        }
    }

    pub fn update_candle_data(&mut self, stock: OHLCData) {
        self.candle_data =
            convert_candlesticks(self.stock_name.len() == 3, stock.to_candlesticks());
    }

    pub fn update_company_info(&mut self, info: CompanyInfo) {
        self.company_info = info;
    }

    /// main function for rendering the chart with plotter
    pub fn render_plot(&mut self, ui_data: UiData) -> (slint::Image, bool) {
        // Check if candle_data is empty and return empty image
        if self.candle_data.is_empty() {
            log::warn!(
                "Cannot render chart: candle_data is empty for stock {}",
                self.stock_name
            );
            return (slint::Image::default(), false);
        }

        self.chart_data.ui_data = ui_data;
        // clean all object on the chart
        if self.chart_data.ui_data.is_clean {
            self.all_draw_data.clear();
            self.current_draw_data.clear();
        }

        if self.chart_data.ui_data.is_undo {
            self.all_draw_data.pop();
        }

        if self.chart_data.candle_distance < 1.5
            && self.chart_data.ui_data.zoom > 0
            && self.chart_data.ui_data.position_x < self.chart_data.width as i32 - 60
        {
            self.chart_data.ui_data.zoom = 0;
        }

        // update data after a mouse release
        if self.chart_data.ui_data.is_release
            && let Some(backup_chart_offset) = self.backup_chart_offset.clone()
        {
            self.chart_data.y_offset_max = backup_chart_offset.y_offset_max;
            self.chart_data.y_offset_min = backup_chart_offset.y_offset_min;
            self.chart_data.x_offset_min = backup_chart_offset.x_offset_min;
            self.chart_data.x_offset_max = backup_chart_offset.x_offset_max;
            self.backup_chart_offset = None;
        }

        // calculate zoom_x, zoom_y and candle distance following the position of cursor:
        // if the cursor is in y-axis field -> zoom_y
        // else -> zoom_x
        let (candle_distance, zoom_x, zoom_y) =
            // hold press in y-axis field to zoom
            if self.chart_data.ui_data.press_x >= self.chart_data.width as i32 - 60 && !self.chart_data.ui_data.is_release {
                let y_diff = self.chart_data.ui_data.position_y - self.chart_data.ui_data.press_y;
                let zoom_factor = y_diff / 10;
                (self.candle_distance(0), 0.0, zoom_factor)
            // zoom in y-axis
            } else if self.is_in_y_field() {
                (self.candle_distance(0), 0.0, self.chart_data.ui_data.zoom)
            // zoom in x-axis
            } else if self.chart_data.x_offset_max - self.chart_data.x_offset_min + 2.0 * self.chart_data.ui_data.zoom as f32 <= 10.0 {
                (self.candle_distance(0), 0.0, 0)
            } else {
            // zoom in x-axis
                (self.candle_distance(self.chart_data.ui_data.zoom), self.chart_data.ui_data.zoom as f32, 0)
            };
        self.chart_data.zoom_x = zoom_x;
        self.chart_data.zoom_y = zoom_y as f32;
        let (mouse_x, mouse_y) = self.get_mouse_position();
        if self.is_in_object.2 && !self.chart_data.ui_data.is_release {
            let dx = self.chart_data.ui_data.move_x as f32 / self.chart_data.candle_distance;
            let dy = -(self.chart_data.ui_data.move_y as f32)
                * (3.0 * (self.chart_data.y_offset_max - self.chart_data.y_offset_min))
                / (self.chart_data.height - self.chart_data.range_x as u32) as f32;

            // Move the object
            debug!("moving - dx: {dx}, dy: {dy}");
            let mut updated_object = self.moving_object.clone();
            updated_object.translate(dx, dy);
            self.all_draw_data[self.is_in_object.0] = updated_object;
            self.chart_data.ui_data.move_x = 0;
            self.chart_data.ui_data.move_y = 0;
        } else {
            for (idx, draw_obj) in self.all_draw_data.iter().enumerate() {
                let x_thresh_hold = (self.chart_data.x_offset_max - self.chart_data.x_offset_min)
                    * THREAD_HOLD
                    / (self.chart_data.width as f32 - 60.0);
                let y_thresh_hold = (3.0
                    * (self.chart_data.y_offset_max - self.chart_data.y_offset_min))
                    * THREAD_HOLD
                    / (self.chart_data.height as f32 - 60.0);

                if draw_obj.hit_cursor((mouse_x, mouse_y), x_thresh_hold, y_thresh_hold) {
                    if self.chart_data.ui_data.is_release {
                        // If the object is not moving, set it to be moving
                        self.is_in_object = (idx, true, false);
                    } else {
                        self.is_in_object = (idx, true, true);
                        // Calculate the delta in chart coordinates
                        self.moving_object = draw_obj.clone();
                    }
                    self.chart_data.ui_data.move_x = 0;
                    self.chart_data.ui_data.move_y = 0;
                    break;
                }
                if !self.is_in_object.2 {
                    self.is_in_object = (0, false, false);
                }
            }
        }

        if self.is_in_object.2 {
            self.chart_data.ui_data.move_x = 0;
            self.chart_data.ui_data.move_y = 0;
        }

        self.chart_data.candle_distance = candle_distance;
        // Update width and height of chart when users resize the window
        if self.chart_data.ui_data.height != 0 && self.chart_data.ui_data.width != 0 {
            self.chart_data.width = self.chart_data.ui_data.width as u32;
            self.chart_data.height = self.chart_data.ui_data.height as u32;
        }

        // Init data for plotters
        let mut pixel_buffer =
            SharedPixelBuffer::new(self.chart_data.width, self.chart_data.height);
        let size = (pixel_buffer.width(), pixel_buffer.height());
        let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
        let root = backend.into_drawing_area();
        // background: 181c27
        root.fill(&RGBColor(0x19, 0x19, 0x1C))
            .expect("error filling drawing area");

        // get candle data to show
        let (space, max_y, min_y) = self.update_y_axis_after_moving();
        let (min_x, max_x) = self.update_x_axis_after_moving();

        // Init the first candle chart with x,y range
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(self.chart_data.range_x)
            .right_y_label_area_size(self.chart_data.range_y)
            .build_cartesian_2d(min_x..max_x, min_y..max_y)
            .expect("error building coordinate system");

        self.delta = (
            (max_x - min_x) / (10.0 * self.chart_data.height as f32),
            (max_y - min_y) / (10.0 * self.chart_data.width as f32),
        );
        // Draw stock name watermark in the background
        self.chart_draw_stock_name_watermark(&mut chart);

        // Handle candle sticks on the chart
        self.chart_update_candle_sticks(&mut chart);

        // Handle drawing on the chart
        // Convert x axis to date time unit
        self.chart_custom_x_axis(&mut chart);

        // Show candle information at the position of mouse
        self.chart_show_candle_info(&mut chart);

        // show company information
        self.chart_show_company_info(&mut chart);

        // Draw all user's objects on the chart
        self.update_current_draw();

        // Draw all user's objects on the chart
        self.chart_draw_objects(&mut chart);

        self.chart_draw_moving_averages(&mut chart);

        self.chart_draw_labels(&mut chart, max_x);

        // update data after a mouse release
        if self.chart_data.ui_data.is_release {
            self.chart_data.y_offset_max = max_y - space;
            self.chart_data.y_offset_min = max_y - 2.0 * space;
            self.chart_data.x_offset_min = min_x;
            self.chart_data.x_offset_max = max_x;
            self.is_in_object.2 = false;
            if !self.current_draw_data.is_empty() {
                if let DrawType::Ruler(_) = self.current_draw_data.draw_type {
                    // nothing to do
                } else {
                    self.all_draw_data.push(self.current_draw_data.clone());
                    self.current_draw_data.clear();
                    self.backup_points.clear();
                }
            }
        } else if self.chart_data.ui_data.press_x >= self.chart_data.width as i32 - 60
            && !self.chart_data.ui_data.is_release
        {
            self.backup_chart_offset = Some(BackupChartOffset {
                y_offset_min: max_y - 2.0 * space,
                y_offset_max: max_y - space,
                x_offset_min: min_x,
                x_offset_max: max_x,
            });
            debug!("save backup_chart_offset {:?}", self.backup_chart_offset);
        }

        root.present().expect("error presenting");
        drop(chart);
        drop(root);

        (slint::Image::from_rgb8(pixel_buffer), self.is_in_object.1)
    }

    /// Handle drawing all candle sticks on the chart
    fn chart_update_candle_sticks(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) {
        let (min_candle_x, max_candle_x) = self.get_min_max_of_candle_after_moving();
        // draw candle stick on the chart
        let chart_candles = &self.candle_data[min_candle_x..max_candle_x];

        // Calculate candles size based on the candle_distance to ensure consistency
        // This ensures the candle size remains the same regardless of how many candles are displayed
        let candles_size = self.candle_distance(0) * 0.8;

        // Draw candlesticks
        chart
            .draw_series(chart_candles.iter().map(|x| {
                if x.open == x.close {
                    let (_, max_y, min_y) = self.update_y_axis_after_moving();
                    let candle_height = self.pixels_to_y_distance(1.0, max_y - min_y);
                    CandleStick::new(
                        x.num,
                        x.open - candle_height,
                        x.high,
                        x.low,
                        x.close,
                        UP_COLOR.filled(),
                        DOWN_COLOR.filled(),
                        candles_size as u32,
                    )
                } else {
                    CandleStick::new(
                        x.num,
                        x.open,
                        x.high,
                        x.low,
                        x.close,
                        UP_COLOR.filled(),
                        DOWN_COLOR.filled(),
                        candles_size as u32,
                    )
                }
            }))
            .unwrap();

        // Draw volume data
        self.draw_volume_data(chart, chart_candles, candles_size);
    }

    /// Draw volume data into the chart
    fn draw_volume_data(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
        chart_candles: &[CandleData],
        candles_size: f32,
    ) {
        let (space, _, min_y) = self.update_y_axis_after_moving();
        let scaled_x6 = self.get_volume_max(chart_candles.to_vec());

        chart
            .draw_series(chart_candles.iter().map(|x| {
                let volume_color = if x.open < x.close {
                    UP_COLOR.filled()
                } else {
                    DOWN_COLOR.filled()
                };
                CandleStick::new(
                    x.num,                                             // The x-axis value
                    x.volume / scaled_x6 * 0.25 * 3.0 * space + min_y, // The scaled open value
                    x.volume / scaled_x6 * 0.25 * 3.0 * space + min_y, // The scaled close value
                    min_y,                                             // The low value
                    min_y,                                             // The high value
                    volume_color,                                      // Up candle color
                    volume_color,                                      // Down candle color
                    candles_size as u32,                               // Width of the candlestick
                )
            }))
            .unwrap();
    }

    /// Handle drawing all user's objects on the chart
    fn chart_draw_objects(
        &mut self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) {
        let mut draw_all_data = self.all_draw_data.clone();

        // Handle drawing feature
        let (x_position, y_position) = self.get_mouse_position();

        // Draw cursor lines following the mouse position
        self.draw_cursor_lines(chart, x_position, y_position);
        let (_, max_y, min_y) = self.update_y_axis_after_moving();
        let (min_x, max_x) = self.update_x_axis_after_moving();

        // Draw all saved lines but don't add new ones
        draw_all_data.push(self.current_draw_data.clone());
        for object in draw_all_data {
            let mut new_object = object.clone();
            new_object.clipped(min_x, max_x, min_y, max_y);
            let draw_points = new_object.to_vec((min_x, max_y), (max_x, min_y));
            match &object.draw_type {
                DrawType::Rectangle((start, end)) => {
                    let (x1, y1) = *start;
                    let (x2, y2) = *end;
                    let min_x = x1.min(x2);
                    let max_x = x1.max(x2);
                    let min_y = y1.min(y2);
                    let max_y = y1.max(y2);

                    // Draw filled rectangle with alpha (simulate blur)
                    let fill_color = object.color.mix(0.2); // 0.2 = 20% opacity
                    chart
                        .draw_series(std::iter::once(Rectangle::new(
                            [(min_x, min_y), (max_x, max_y)],
                            fill_color.filled(),
                        )))
                        .unwrap();

                    // Draw the border as usual
                    chart
                        .draw_series(std::iter::once(Rectangle::new(
                            [(min_x, min_y), (max_x, max_y)],
                            object.color,
                        )))
                        .unwrap();
                }
                DrawType::Oval((start, end)) => {
                    // Use original object coordinates, not clipped ones
                    let (x1, y1) = *start;
                    let (x2, y2) = *end;
                    let center_x = (x1 + x2) / 2.0;
                    let center_y = (y1 + y2) / 2.0;
                    let rx = (x2 - x1).abs() / 2.0;
                    let ry = (y2 - y1).abs() / 2.0;

                    // Draw filled ellipse with alpha (simulate blur) similar to Rectangle
                    let fill_color = object.color.mix(0.2); // Use original object color

                    // For an ellipse, we'll approximate it using a polygon with many points
                    let num_points = 128; // Balanced for performance and quality
                    let mut ellipse_points = Vec::new();

                    // Generate points for a complete ellipse
                    for i in 0..=num_points {
                        let angle = 2.0 * std::f32::consts::PI * i as f32 / num_points as f32;
                        let x = center_x + rx * angle.cos();
                        let y = center_y + ry * angle.sin();
                        ellipse_points.push((x, y));
                    }

                    // Draw filled polygon (ellipse approximation)
                    let fill_points: Vec<(f32, f32)> = ellipse_points[0..num_points].to_vec();
                    if fill_points.len() > 2 {
                        chart
                            .draw_series(std::iter::once(Polygon::new(
                                fill_points,
                                fill_color.filled(),
                            )))
                            .unwrap();
                    }

                    // Draw the ellipse outline - filter points to visible area to avoid artifacts
                    let visible_points: Vec<(f32, f32)> = ellipse_points
                        .into_iter()
                        .filter(|(x, y)| *x >= min_x && *x <= max_x && *y >= min_y && *y <= max_y)
                        .collect();

                    if visible_points.len() > 1 {
                        chart
                            .draw_series(LineSeries::new(
                                visible_points,
                                object.color.stroke_width(1),
                            ))
                            .unwrap();
                    }
                }
                DrawType::Ruler((end, start)) => {
                    if start.0 == end.0 || start.1 == end.1 {
                        // If the start and end points are the same, skip drawing
                        continue;
                    }
                    let (x1, y1) = *start;
                    let (x2, y2) = *end;

                    let first_arrow_start = ((start.0 + end.0) / 2.0, start.1);
                    let first_arrow_end = ((start.0 + end.0) / 2.0, end.1);
                    let second_arrow_start = (start.0, (start.1 + end.1) / 2.0);
                    let second_arrow_end = (end.0, (start.1 + end.1) / 2.0);

                    let x_width = max_x - min_x;
                    let y_width = max_y - min_y;
                    let min_x = x1.min(x2);
                    let max_x = x1.max(x2);
                    let min_y = y1.min(y2);
                    let max_y = y1.max(y2);
                    let percent = (end.1 - start.1) * 100.0 / start.1;
                    let percent_string = if percent > 0.0 {
                        format!("+{:.02} (+{:.02}%)", (end.1 - start.1), percent)
                    } else {
                        format!("{:.02} ({:.02}%)", (end.1 - start.1), percent)
                    };

                    let color = if percent >= 0.0 { BLUE } else { RED };

                    self.draw_arrow(
                        chart,
                        &first_arrow_start,
                        &first_arrow_end,
                        color,
                        max_x,
                        min_x,
                        max_y,
                        min_y,
                    );
                    self.draw_arrow(
                        chart,
                        &second_arrow_start,
                        &second_arrow_end,
                        color,
                        max_x,
                        min_x,
                        max_y,
                        min_y,
                    );

                    // Draw filled rectangle with alpha (simulate blur)
                    let fill_color = color.mix(0.2); // 0.2 = 20% opacity
                    chart
                        .draw_series(std::iter::once(Rectangle::new(
                            [(min_x, min_y), (max_x, max_y)],
                            fill_color.filled(),
                        )))
                        .unwrap();
                    let label_style = ShapeStyle {
                        color: color.into(),
                        filled: true,
                        stroke_width: 0,
                    };
                    // Calculate the top middle position of the rectangle
                    let mid_x = (x1 + x2) / 2.0;
                    let top_y = y1.max(y2);
                    // Rectangle for the label (centered at mid_x, above the rectangle)
                    let label_box_width = self.pixels_to_x_distance(140.0, x_width);
                    let label_box_height = self.pixels_to_y_distance(20.0, y_width);
                    let rect_top_left = (
                        first_arrow_start.0 - label_box_width / 2.0,
                        top_y + 2.0 * label_box_height,
                    );
                    let rect_bottom_right = (
                        mid_x + label_box_width / 2.0,
                        top_y + label_box_height / 2.0,
                    );

                    let rectangle_price =
                        Rectangle::new([rect_top_left, rect_bottom_right], label_style);
                    let _ = chart.plotting_area().draw(&rectangle_price);
                    let _ = chart.plotting_area().draw(&Text::new(
                        percent_string,
                        (
                            rect_top_left.0 + label_box_width / 10.0,
                            rect_top_left.1 - label_box_height / 2.25,
                        ),
                        ("Arial-Bold", 16).into_font().color(&WHITE),
                    ));
                }
                DrawType::Arrow((end, start)) => {
                    // Draw the main line
                    self.draw_arrow(chart, start, end, object.color, max_x, min_x, max_y, min_y);
                }
                _ => {
                    chart
                        .draw_series(LineSeries::new(draw_points, object.color.stroke_width(1)))
                        .unwrap()
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x, y)], object.color));
                }
            }
        }
    }

    fn draw_arrow(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
        start: &(f32, f32),
        end: &(f32, f32),
        color: RGBColor,
        max_x: f32,
        min_x: f32,
        max_y: f32,
        min_y: f32,
    ) {
        // Draw the main line
        chart
            .draw_series(LineSeries::new(vec![*start, *end], color.stroke_width(1)))
            .unwrap();

        // Draw the arrowhead
        let arrow_length_x = self.pixels_to_x_distance(20.0, max_x - min_x); // in pixels
        let arrow_length_y = self.pixels_to_y_distance(20.0, max_y - min_y); // in pixels
        let arrow_angle = std::f32::consts::PI / 7.0; // angle of the arrowhead

        let dx = self.x_distance_to_pixels(end.0 - start.0, max_x - min_x);
        let dy = self.y_distance_to_pixels(end.1 - start.1, max_y - min_y);
        let angle = dy.atan2(dx);

        // Calculate the two points for the arrowhead
        let arrow_point1 = (
            end.0 - arrow_length_x * (angle - arrow_angle).cos(),
            end.1 - arrow_length_y * (angle - arrow_angle).sin(),
        );
        let arrow_point2 = (
            end.0 - arrow_length_x * (angle + arrow_angle).cos(),
            end.1 - arrow_length_y * (angle + arrow_angle).sin(),
        );

        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![*end, arrow_point1],
                color.stroke_width(1),
            )))
            .unwrap();
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![*end, arrow_point2],
                color.stroke_width(1),
            )))
            .unwrap();
    }

    /// Draw cursor lines for mouse position
    fn draw_cursor_lines(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
        x_position: f32,
        y_position: f32,
    ) {
        // Check if cursor is inside a candle for vertical line only
        let mut x_pos = x_position;
        let (min_candle_x, max_candle_x) = self.get_min_max_of_candle_after_moving();
        let candle_idx = x_position.round() as usize;
        if candle_idx >= min_candle_x
            && candle_idx < max_candle_x
            && candle_idx < self.candle_data.len()
        {
            // Cursor is inside a candle, set vertical line to middle of candle
            let candle = &self.candle_data[candle_idx];
            x_pos = candle.num;
        }
        let x_line = self.cursor_vertical_line(x_pos);
        let y_line = self.cursor_horizontal_line(y_position);
        // Draw the vertical and horizontal line of mouse position
        chart
            .draw_series(LineSeries::new(x_line, GREY.stroke_width(1)))
            .unwrap()
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        chart
            .draw_series(LineSeries::new(y_line, GREY.stroke_width(1)))
            .unwrap()
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    }

    /// Handle different drawing types
    fn update_current_draw(&mut self) {
        // If cursor is in y-axis field, don't add drawing points
        if self.is_in_y_field() {
            return;
        }

        let (x, y) = self.get_mouse_position();
        // Save the current mouse position
        let press_point = self.get_press_position();
        let color = RGBColor(
            self.chart_data.ui_data.color.red(),
            self.chart_data.ui_data.color.green(),
            self.chart_data.ui_data.color.blue(),
        );
        if !self.chart_data.ui_data.is_release
            && self.chart_data.ui_data.press_x < self.chart_data.width as i32 - 60
        {
            match self.chart_data.ui_data.mouse_type {
                MouseType::Line => {
                    self.current_draw_data =
                        DrawObject::new(DrawType::Line(((x, y), press_point)), color);
                }
                MouseType::Arrow => {
                    self.current_draw_data =
                        DrawObject::new(DrawType::Arrow(((x, y), press_point)), color);
                }
                MouseType::Rectangle => {
                    self.current_draw_data =
                        DrawObject::new(DrawType::Rectangle(((x, y), press_point)), color);
                }
                MouseType::Oval => {
                    self.current_draw_data =
                        DrawObject::new(DrawType::Oval(((x, y), press_point)), color);
                }
                MouseType::VerticalLine => {
                    self.current_draw_data = DrawObject::new(DrawType::VerticalLine(x), color);
                }
                MouseType::HorizontalLine => {
                    self.current_draw_data = DrawObject::new(DrawType::HorizontalLine(y), color);
                }
                MouseType::Draw => {
                    self.backup_points.push((x, y));
                    let smoothed =
                        interpolate_min_distance(&self.backup_points, self.delta.0, self.delta.1);
                    self.backup_points = smoothed;
                    self.current_draw_data =
                        DrawObject::new(DrawType::Pen(self.backup_points.clone()), color);
                }
                MouseType::Ruler => {
                    self.current_draw_data =
                        DrawObject::new(DrawType::Ruler(((x, y), press_point)), color);
                }
                _ => {}
            }
        }
    }

    /// Draw all labels of the chart:
    ///   - Latest price label
    ///   - latest volume label
    ///   - cursor price label
    ///   - cursor date label
    fn chart_draw_labels(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
        max_x: f32,
    ) {
        let (min_candle_x, max_candle_x) = self.get_min_max_of_candle_after_moving();

        let space = self.chart_data.y_offset_max - self.chart_data.y_offset_min;
        let (_, _, min) = self.update_y_axis_after_moving();
        let (_, y_position) = self.get_mouse_position();
        let chart_candles = &self.candle_data[min_candle_x..max_candle_x];
        let max_volume = self.get_volume_max(chart_candles.to_vec());
        let last_volume = self.get_last_volume();
        let last_volume_map_to_y = last_volume / max_volume * 0.25 * 3.0 * space + min;

        // Define styles
        let cursor_label_style = ShapeStyle {
            color: RGBColor(0x3A, 0x36, 0x45).into(),
            filled: true,
            stroke_width: 0,
        };
        let green_label_style = ShapeStyle {
            color: RGBColor(0x23, 0xBA, 0x75).into(),
            filled: true,
            stroke_width: 0,
        };

        // Draw cursor labels
        self.draw_cursor_labels(chart, max_x, y_position, min, cursor_label_style);

        // Draw latest price label
        self.draw_latest_price_label(chart, max_x, space);

        // Draw latest volume label
        self.draw_latest_volume_label(chart, max_x, last_volume_map_to_y, space, green_label_style);
    }

    /// Draw cursor labels
    fn draw_cursor_labels(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
        max_x: f32,
        y_position: f32,
        min: f32,
        cursor_label_style: ShapeStyle,
    ) {
        let space = self.chart_data.y_offset_max - self.chart_data.y_offset_min;
        let (x_position, _) = self.get_mouse_position();

        // Draw the label for the cursor date
        let mut cursor_rectangle = Rectangle::new(
            [
                (max_x + 10.0, y_position - 0.7 / 20.0 * space),
                (max_x + 100.0, y_position + 0.7 / 20.0 * space),
            ],
            cursor_label_style,
        );
        cursor_rectangle.set_margin(0, 0, 50, 0);

        // Show the price of current mouse position
        let y_position_str = format!("   {y_position:.02}");
        let _ = chart.plotting_area().draw(&cursor_rectangle);
        let _ = chart.plotting_area().draw(&Text::new(
            y_position_str,
            (
                max_x,
                y_position
                    + (7.0 * (self.chart_data.y_offset_max - self.chart_data.y_offset_min)
                        / (self.chart_data.height - self.chart_data.range_x as u32) as f32),
            ),
            ("Arial-Bold", 14).into_font().color(&WHITE),
        ));

        // Draw the label for the cursor date
        // Calculate the date based on cursor position, including future dates
        let cursor_index = x_position.round() as i32;
        let cursor_date = if cursor_index < 0 {
            // Past dates - go backwards from the first data point
            let days_back = (-cursor_index) as i64;
            self.candle_data[0].time - Duration::days(days_back)
        } else if cursor_index >= self.candle_data.len() as i32 {
            // Future dates - calculate forward from the last data point
            let days_forward = (cursor_index - (self.candle_data.len() as i32 - 1)) as i64;
            self.candle_data[self.candle_data.len() - 1].time + Duration::days(days_forward)
        } else {
            // Within data range - use actual data
            self.candle_data[cursor_index as usize].time
        };

        let mut rectangle_cursor_x =
            Rectangle::new([(x_position, min), (x_position, min)], cursor_label_style);
        rectangle_cursor_x.set_margin(15, 0, 60, 5);
        let output = &cursor_date.to_string()[0..11];
        let _ = chart.plotting_area().draw(&rectangle_cursor_x);
        let _ = chart.plotting_area().draw(&Text::new(
            output.to_string(),
            (x_position, min - 2.0),
            ("Arial-Bold", 14).into_font().color(&WHITE),
        ));
    }

    /// Draw latest price label
    fn draw_latest_price_label(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
        max_x: f32,
        space: f32,
    ) {
        let last_price = self.get_last_price();
        let last_price_str = format!("    {last_price:.02}");

        // Determine the color based on the last candle
        let is_up = self.is_last_candle_up();
        let candle_color = if is_up { UP_COLOR } else { DOWN_COLOR };

        // Create a label style with the appropriate color
        let label_style = ShapeStyle {
            color: candle_color.into(),
            filled: true,
            stroke_width: 0,
        };

        let mut rectangle_price = Rectangle::new(
            [
                (max_x + 10.0, last_price - 1.0 / 20.0 * space),
                (max_x + 100.0, last_price + 0.7 / 20.0 * space),
            ],
            label_style,
        );
        rectangle_price.set_margin(0, 0, 50, 0);
        let _ = chart.plotting_area().draw(&rectangle_price);
        let _ = chart.plotting_area().draw(&Text::new(
            last_price_str,
            (
                max_x,
                last_price
                    + (7.0 * (self.chart_data.y_offset_max - self.chart_data.y_offset_min)
                        / (self.chart_data.height - self.chart_data.range_x as u32) as f32),
            ),
            ("Arial-Bold", 14).into_font().color(&WHITE),
        ));

        // Draw a horizontal broken line at the latest price
        // Create a dashed line effect by drawing multiple small line segments
        const DASH_LENGTH_PX: f32 = 2.0; // Length of each dash in pixels
        const GAP_LENGTH_PX: f32 = 2.0; // Length of gap between dashes in pixels

        // Convert pixel lengths to chart coordinates
        let dash_length = DASH_LENGTH_PX / self.chart_data.candle_distance;
        let gap_length = GAP_LENGTH_PX / self.chart_data.candle_distance;
        let total_length = self.chart_data.x_offset_max - self.chart_data.x_offset_min;
        let num_segments = (total_length / (dash_length + gap_length)).ceil() as i32;

        for i in 0..num_segments {
            let start_x = max_x - total_length + i as f32 * (dash_length + gap_length);
            let end_x = (start_x + dash_length).min(self.chart_data.x_offset_max);

            if start_x < max_x {
                let segment = vec![(start_x, last_price), (end_x, last_price)];

                chart
                    .draw_series(LineSeries::new(segment, candle_color.stroke_width(1)))
                    .unwrap();
            }
        }
    }

    /// Draw latest volume label
    fn draw_latest_volume_label(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
        max_x: f32,
        last_volume_map_to_y: f32,
        space: f32,
        green_label_style: ShapeStyle,
    ) {
        let last_volume = self.get_last_volume();
        let y_price = if last_volume > 1000000.0 {
            format!("    {:.2} M", last_volume / 1000000.0)
        } else {
            format!("    {:.1} K", last_volume / 1000.0)
        };

        // Calculate the width and height of the box based on the text length and font size
        let mut rectangle = Rectangle::new(
            [
                (max_x + 10.0, last_volume_map_to_y - 1.0 / 20.0 * space),
                (max_x + 100.0, last_volume_map_to_y + 0.7 / 20.0 * space),
            ],
            green_label_style,
        );

        rectangle.set_margin(0, 0, 50, 0);

        let _ = chart.plotting_area().draw(&rectangle);
        let _ = chart.plotting_area().draw(&Text::new(
            y_price,
            (
                max_x,
                last_volume_map_to_y
                    + (7.0 * (self.chart_data.y_offset_max - self.chart_data.y_offset_min)
                        / (self.chart_data.height - self.chart_data.range_x as u32) as f32),
            ),
            ("Arial-Bold", 14).into_font().color(&WHITE),
        ));
    }

    /// Customize the x value from DateTime<Utc> to index to prevent
    /// the date which has no data of candle stick.
    fn chart_custom_x_axis(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) {
        // Create a mapping of dates to indices
        let date_to_index: Vec<(DateTime<Utc>, usize)> = self
            .candle_data
            .iter()
            .enumerate()
            .map(|(idx, x)| (x.time, idx))
            .collect();

        // Configure the x-axis and y-axis labels with larger font sizes
        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_label_formatter(&|&idx| {
                if idx > 0.0 && idx < date_to_index.len() as f32 {
                    let date = &(date_to_index[idx as usize].0
                        + Duration::days(
                            ((self.chart_data.ui_data.move_x as f32
                                / self.chart_data.candle_distance)
                                as i32)
                                .into(),
                        ));
                    format!("{}", date.format("Tháng %m\n"))
                } else {
                    "".to_string()
                }
            })
            .y_label_formatter(&|&val| format!("{val:.2}")) // Format y-axis labels with 2 decimals
            .label_style(("Arial-Bold", 16).into_font().color(&WHITE)) // Increased font size
            .axis_style(WHITE.stroke_width(1))
            .draw()
            .expect("error drawing mesh");
    }

    /// show the information of candle stick on the left-top-corner
    /// basing on the mouse position: low, high, open, close prices
    fn chart_show_candle_info(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) {
        // get candle data to show
        let (_, max_y, min_y) = self.update_y_axis_after_moving();
        let (min_x, _) = self.update_x_axis_after_moving();
        let y_stock_text = max_y - (max_y - min_y) * (20.0 / self.chart_data.height as f32);
        let y_volumn_text = max_y - (max_y - min_y) * (40.0 / self.chart_data.height as f32);
        let y_ma_20_text = max_y - (max_y - min_y) * (60.0 / self.chart_data.height as f32);
        let y_ma_50_text = max_y - (max_y - min_y) * (80.0 / self.chart_data.height as f32);
        let y_ma_200_text = max_y - (max_y - min_y) * (100.0 / self.chart_data.height as f32);

        let (x, _) = self.get_mouse_position();
        // If cursor is exactly at the middle of a candle, show info for that candle
        let candle_idx = x.round() as usize;
        let (candle_data, previous_candle) =
            if candle_idx < self.candle_data.len() && candle_idx > 0 {
                (
                    self.candle_data[candle_idx].clone(),
                    self.candle_data[candle_idx - 1].clone(),
                )
            } else if candle_idx == 0 && self.candle_data.len() > 1 {
                (self.candle_data[0].clone(), self.candle_data[1].clone())
            } else {
                // fallback to previous logic
                if x as usize >= self.candle_data.len() - 1 {
                    (
                        self.candle_data[self.candle_data.len() - 1].clone(),
                        self.candle_data[self.candle_data.len() - 2].clone(),
                    )
                } else {
                    (
                        self.candle_data[x as usize + 1].clone(),
                        self.candle_data[x as usize].clone(),
                    )
                }
            };

        let change = candle_data.close - previous_candle.close;
        let change_percent =
            (candle_data.close - previous_candle.close) / previous_candle.close * 100.0;
        // Format change and change_percent with "+" if the value is positive
        let formatted_change = if change > 0.0 {
            format!("+{change:.02}")
        } else {
            format!("{change:.02}")
        };

        let formatted_change_percent = if change_percent > 0.0 {
            format!("+{change_percent:.2}%")
        } else {
            format!("{change_percent:.2}%")
        };
        let output = format!(
            "  {} - Open: {:.02}, High: {:.02}, Close: {:.02}, Low: {:.02}, {} ({})",
            self.stock_name,
            candle_data.open,
            candle_data.high,
            candle_data.close,
            candle_data.low,
            formatted_change,
            formatted_change_percent
        );
        let color = if change > 0.0 {
            UP_COLOR
        } else if change < 0.0 {
            DOWN_COLOR
        } else {
            NO_CHANGE_COLOR
        };
        let _ = chart.plotting_area().draw(&Text::new(
            output,
            (min_x, y_stock_text),
            ("sans-serif", 15).into_font().color(&color),
        ));

        let volumn = if candle_data.volume > 1000000.0 {
            format!(
                "  Volume - Khối lượng: {:.3} M",
                candle_data.volume / 1000000.0
            )
        } else {
            format!(
                "  Volume - Khối lượng: {:.3} K",
                candle_data.volume / 1000.0
            )
        };
        let _ = chart.plotting_area().draw(&Text::new(
            volumn,
            (min_x, y_volumn_text),
            ("sans-serif", 15).into_font().color(&color),
        ));
        let _ = chart.plotting_area().draw(&Text::new(
            "  MA20    _________",
            (min_x, y_ma_20_text),
            ("sans-serif", 15).into_font().color(&RED),
        ));
        let _ = chart.plotting_area().draw(&Text::new(
            "  MA50    _________",
            (min_x, y_ma_50_text),
            ("sans-serif", 15).into_font().color(&BLUE),
        ));
        let _ = chart.plotting_area().draw(&Text::new(
            "  MA200  _________",
            (min_x, y_ma_200_text),
            ("sans-serif", 15).into_font().color(&WHITE),
        ));
    }

    /// Draw stock name as watermark in the background center of the chart
    fn chart_draw_stock_name_watermark(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) {
        let (_, max_y, min_y) = self.update_y_axis_after_moving();
        let (min_x, max_x) = self.update_x_axis_after_moving();

        // Position the stock name slightly above center for better visibility
        let center_x = min_x + (max_x - min_x) / 2.5;
        let center_y = min_y + (max_y - min_y) * 0.6; // Position at 60% from bottom for better centering

        // Create a very light watermark effect - extremely transparent
        let watermark_color = RGBColor(0x28, 0x28, 0x2D);

        // Draw the stock name with a very large, bold font as watermark
        let _ = chart.plotting_area().draw(&Text::new(
            format!("{}, 1D", self.stock_name), // Add timeframe like in the image
            (center_x, center_y),
            ("Arial-Bold", 120).into_font().color(&watermark_color), // Slightly smaller font
        ));
    }

    /// Show the company information on the right-top-corner
    fn chart_show_company_info(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) {
        // Define the starting position for the text
        let (_, max_y, min_y) = self.update_y_axis_after_moving();
        let (min_x, max_x) = self.update_x_axis_after_moving();
        let position_x = min_x + (max_x - min_x) * 4.0 / 5.0;
        let y_esp = max_y - (max_y - min_y) * (20.0 / self.chart_data.height as f32);
        let y_pe = max_y - (max_y - min_y) * (40.0 / self.chart_data.height as f32);
        let y_pb = max_y - (max_y - min_y) * (60.0 / self.chart_data.height as f32);
        let y_roe = max_y - (max_y - min_y) * (80.0 / self.chart_data.height as f32);
        let y_roa = max_y - (max_y - min_y) * (100.0 / self.chart_data.height as f32);

        // Define the font style
        let font = ("sans-serif", 25).into_font().color(&WHITE);

        // Display EPS
        let eps_text = format!("EPS: {:.2}", self.company_info.eps);
        let _ = chart
            .plotting_area()
            .draw(&Text::new(eps_text, (position_x, y_esp), font.clone()));

        // Display PE
        let pe_text = format!("PE: {:.2}", self.company_info.pe);
        let _ = chart
            .plotting_area()
            .draw(&Text::new(pe_text, (position_x, y_pe), font.clone()));

        // Display PB
        let pb_text = format!("PB: {:.2}", self.company_info.pb);
        let _ = chart
            .plotting_area()
            .draw(&Text::new(pb_text, (position_x, y_pb), font.clone()));

        // Display ROE
        let roe_text = format!("ROE: {:.2}%", self.company_info.roe * 100.0);
        let _ = chart
            .plotting_area()
            .draw(&Text::new(roe_text, (position_x, y_roe), font.clone()));

        // Display ROA
        let roa_text = format!("ROA: {:.2}%", self.company_info.roa * 100.0);
        let _ = chart
            .plotting_area()
            .draw(&Text::new(roa_text, (position_x, y_roa), font.clone()));
    }

    /// this function is to check if the cursor is in y-axis field
    fn is_in_y_field(&self) -> bool {
        self.chart_data.ui_data.position_x >= self.chart_data.width as i32 - self.chart_data.range_y
            && self.chart_data.ui_data.position_y
                <= self.chart_data.height as i32 - self.chart_data.range_x
    }

    /// this function will create a vector of position for vertical line of cursor position
    fn cursor_vertical_line(&self, x_position: f32) -> Vec<(f32, f32)> {
        let space = self.chart_data.y_offset_max - self.chart_data.y_offset_min;
        let converted_move_y = space * 3.0 * (self.chart_data.ui_data.move_y as f32)
            / ((self.chart_data.height as f32) - (self.chart_data.range_x as f32));
        vec![
            (
                x_position,
                self.chart_data.y_offset_min - space + converted_move_y,
            ),
            (
                x_position,
                self.chart_data.y_offset_max + space + converted_move_y,
            ),
        ]
    }

    pub fn calculate_moving_average(
        &self,
        period: usize,
        min_candle_x: usize,
        max_candle_x: usize,
    ) -> Vec<f32> {
        let mut ma_values = Vec::new();
        let mut sum = 0.0;

        for i in (min_candle_x - period)..max_candle_x {
            sum += self.candle_data[i].close; // Add the closing price

            if i - (min_candle_x - period) >= period {
                sum -= self.candle_data[i - period].close; // Remove the price outside the window
            }
            if i - (min_candle_x - period) >= period - 1 {
                ma_values.push(sum / period as f32); // Calculate the moving average
            }
        }
        ma_values
    }

    fn chart_draw_moving_averages(
        &self,
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    ) {
        let (min_candle_x, max_candle_x) = self.get_min_max_of_candle_after_moving();
        // Calculate MA5, MA20, MA100
        let ma20 = self.calculate_moving_average(20, min_candle_x, max_candle_x);
        let ma50 = self.calculate_moving_average(50, min_candle_x, max_candle_x);
        let ma200 = self.calculate_moving_average(200, min_candle_x, max_candle_x);

        // Draw MA5
        let ma20_points: Vec<(f32, f32)> = self
            .candle_data
            .iter()
            .enumerate()
            .enumerate()
            .filter_map(|(i, _)| ma20.get(i).map(|&ma| ((i + min_candle_x) as f32, ma)))
            .collect();
        chart
            .draw_series(LineSeries::new(ma20_points, RED.stroke_width(1)))
            .expect("Error drawing MA20");

        // Draw MA20
        let ma50_points: Vec<(f32, f32)> = self
            .candle_data
            .iter()
            .enumerate()
            .filter_map(|(i, _)| ma50.get(i).map(|&ma| ((i + min_candle_x) as f32, ma)))
            .collect();
        chart
            .draw_series(LineSeries::new(ma50_points, BLUE.stroke_width(1)))
            .expect("Error drawing MA50");

        // Draw MA100
        let ma200_points: Vec<(f32, f32)> = self
            .candle_data
            .iter()
            .enumerate()
            .filter_map(|(i, _)| ma200.get(i).map(|&ma| ((i + min_candle_x) as f32, ma)))
            .collect();
        chart
            .draw_series(LineSeries::new(ma200_points, WHITE.stroke_width(1)))
            .expect("Error drawing MA200");
    }

    fn cursor_horizontal_line(&self, y_position: f32) -> Vec<(f32, f32)> {
        let converted_move_x =
            self.chart_data.ui_data.move_x as f32 / self.chart_data.candle_distance;

        vec![
            (
                self.chart_data.x_offset_min - self.chart_data.zoom_x - converted_move_x,
                y_position,
            ),
            (
                self.chart_data.x_offset_max + self.chart_data.zoom_x - converted_move_x,
                y_position,
            ),
        ]
    }

    fn candle_distance(&self, zoom: i32) -> f32 {
        (self.chart_data.width - self.chart_data.range_y as u32) as f32
            / (self.chart_data.x_offset_max + zoom as f32
                - (self.chart_data.x_offset_min - zoom as f32))
    }

    fn update_y_axis_after_moving(&self) -> (f32, f32, f32) {
        let zoom_ratio = (100.0 + self.chart_data.zoom_y) / 100.0;

        // Calculate the real size of the y-axis based on the zoom ratio
        let real_size =
            3.0 * (self.chart_data.y_offset_max - self.chart_data.y_offset_min) * zoom_ratio;

        // Calculate the difference between the real size and the current size
        let difference = real_size - (self.chart_data.y_offset_max - self.chart_data.y_offset_min);

        // Calculate y_moving based on move_y
        let y_moving = real_size * (self.chart_data.ui_data.move_y as f32)
            / ((self.chart_data.height as i32) - self.chart_data.range_x) as f32;

        // Calculate new min and max values for y-axis
        let max = self.chart_data.y_offset_max + difference / 2.0 + y_moving;
        let min = self.chart_data.y_offset_min - difference / 2.0 + y_moving;

        (real_size / 3.0, max, min)
    }

    fn update_x_axis_after_moving(&self) -> (f32, f32) {
        let min_x = self.chart_data.x_offset_min
            - self.chart_data.ui_data.move_x as f32 / self.chart_data.candle_distance
            - self.chart_data.zoom_x;
        let max_x = (self.chart_data.x_offset_max
            - self.chart_data.ui_data.move_x as f32 / self.chart_data.candle_distance)
            + self.chart_data.zoom_x;
        (min_x, max_x)
    }

    fn get_min_max_of_candle_after_moving(&self) -> (usize, usize) {
        // convert move x from pixels to real distance in x axis.
        let converted_move_x =
            self.chart_data.ui_data.move_x as f32 / self.chart_data.candle_distance;
        // calculate the min data of candle sticks that show on the chart.
        let converted_min_x =
            self.chart_data.x_offset_min - converted_move_x - self.chart_data.zoom_x;
        // calculate the max data of candle sticks that show on the chart.
        let converted_max_x =
            self.chart_data.x_offset_max - converted_move_x + self.chart_data.zoom_x;
        // handle boudary conditions of min value
        let min_candle_x = if converted_min_x < 0.0 {
            0
        } else if converted_min_x > self.candle_data.len() as f32 {
            self.candle_data.len()
        } else {
            converted_min_x as usize
        };

        // handle boudary conditions of max value
        let max_candle_x = if converted_max_x > self.candle_data.len() as f32 {
            self.candle_data.len()
        } else if converted_max_x < 0.0 {
            0
        } else {
            converted_max_x as usize
        };
        (min_candle_x, max_candle_x)
    }

    /// get the maximum value of candle stick price array
    fn get_y_max(data: CandleDataVec) -> f32 {
        data.iter()
            .map(|candle| candle.high)
            .fold(0.0, |acc, x| acc.max(x))
    }

    /// get the maximum value of volume array
    fn get_volume_max(&self, data: CandleDataVec) -> f32 {
        data.iter()
            .map(|candle| candle.volume)
            .fold(0.0, |acc, x| acc.max(x))
    }

    /// get the minimum value of candle stick price array
    fn get_y_min(data: CandleDataVec) -> f32 {
        data.iter()
            .map(|candle| candle.low)
            .fold(f32::INFINITY, |acc, x| acc.min(x))
    }

    /// Get the last price -> close price of last candle stick
    fn get_last_price(&self) -> f32 {
        self.candle_data
            .last()
            .map_or(0.0, |lastest_data| lastest_data.close)
    }

    /// Determine if the last candle is up (green) or down (red)
    fn is_last_candle_up(&self) -> bool {
        if let Some(last_candle) = self.candle_data.last() {
            // In candlestick data, index 2 is open, index 5 is close
            // If close > open, it's an up candle (green)
            last_candle.close > last_candle.open
        } else {
            false // Default to down if no data
        }
    }

    /// This function will return the position of the mouse on the chart
    /// return value as (f32, f32) with x, y in chart values.
    fn get_mouse_position(&self) -> (f32, f32) {
        let (_, max_y, min_y) = self.update_y_axis_after_moving();
        let y_position = min_y
            + ((self.chart_data.height as i32
                - self.chart_data.ui_data.position_y
                - self.chart_data.range_x) as f32
                / (self.chart_data.height - self.chart_data.range_x as u32) as f32)
                * (max_y - min_y);
        let x_position = ((self.chart_data.ui_data.position_x - self.chart_data.ui_data.move_x)
            as f32
            / self.chart_data.candle_distance)
            + self.chart_data.x_offset_min
            - self.chart_data.zoom_x;

        (x_position, y_position)
    }

    fn get_press_position(&self) -> (f32, f32) {
        let (_, max_y, min_y) = self.update_y_axis_after_moving();
        let y_position = min_y
            + ((self.chart_data.height as i32
                - self.chart_data.ui_data.press_y
                - self.chart_data.range_x) as f32
                / (self.chart_data.height - self.chart_data.range_x as u32) as f32)
                * (max_y - min_y);
        let x_position = ((self.chart_data.ui_data.press_x - self.chart_data.ui_data.move_x)
            as f32
            / self.chart_data.candle_distance)
            + self.chart_data.x_offset_min
            - self.chart_data.zoom_x;

        (x_position, y_position)
    }

    /// Get the last volume
    fn get_last_volume(&self) -> f32 {
        self.candle_data
            .last()
            .map_or(0.0, |lastest_data| lastest_data.volume)
    }

    fn pixels_to_y_distance(&self, pixels: f32, y_height: f32) -> f32 {
        (y_height / (self.chart_data.height as f32)) * pixels
    }

    fn pixels_to_x_distance(&self, pixels: f32, x_width: f32) -> f32 {
        (x_width / (self.chart_data.width as f32)) * pixels
    }

    fn x_distance_to_pixels(&self, x_distance: f32, x_width: f32) -> f32 {
        ((self.chart_data.width as f32) / x_width) * x_distance
    }

    fn y_distance_to_pixels(&self, y_distance: f32, y_height: f32) -> f32 {
        ((self.chart_data.height as f32) / y_height) * y_distance
    }
}

/// Interpolates points so that the distance between consecutive points is at most delta_x or delta_y.
pub fn interpolate_min_distance(points: &[Point], delta_x: f32, delta_y: f32) -> Vec<Point> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::new();
    for window in points.windows(2) {
        let (x0, y0) = window[0];
        let (x1, y1) = window[1];
        result.push((x0, y0));
        let dx = x1 - x0;
        let dy = y1 - y0;
        let steps_x = if delta_x > 0.0 {
            (dx.abs() / delta_x).ceil() as usize
        } else {
            0
        };
        let steps_y = if delta_y > 0.0 {
            (dy.abs() / delta_y).ceil() as usize
        } else {
            0
        };
        let steps = steps_x.max(steps_y);
        if steps > 1 {
            for i in 1..steps {
                let t = i as f32 / steps as f32;
                let xi = x0 + dx * t;
                let yi = y0 + dy * t;
                result.push((xi, yi));
            }
        }
    }
    // Always push the last point
    if let Some(&last) = points.last() {
        result.push(last);
    }
    result
}
