use crate::{CandleData, CandleDataVec};
use plotters::{
    backend::BitMapBackend,
    drawing::IntoDrawingArea,
    prelude::Text,
    style::{IntoFont, RGBColor, WHITE},
};
use slint::SharedPixelBuffer;

/// main function for rendering the chart with plotter
pub fn mini_chart_render(ref_price: f32, data: CandleDataVec) -> slint::Image {
    // Chart size
    let width = 320u32;
    let height = 100u32;
    let mut pixel_buffer = SharedPixelBuffer::new(width, height);

    // Create a scope where all plotters objects will be dropped
    {
        let size = (pixel_buffer.width(), pixel_buffer.height());
        let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);
        let root = backend.into_drawing_area();
        root.fill(&RGBColor(0, 0, 0)).ok();

        if data.is_empty() {
            // Draw "No Data" message
            root.draw(&Text::new(
                "No Data Available",
                (width as i32 / 2, height as i32 / 2),
                ("sans-serif", 16).into_font().color(&WHITE),
            ))
            .ok();
            root.present().ok();
        } else {
            render_simple_trading_chart(&root, &data, width, height, ref_price);
            root.present().ok();
        }
    } // All plotters objects (backend, root, chart) are dropped here

    slint::Image::from_rgb8(pixel_buffer)
}

fn render_simple_trading_chart(
    root: &plotters::drawing::DrawingArea<plotters::backend::BitMapBackend, plotters::coord::Shift>,
    data: &[CandleData],
    width: u32,
    height: u32,
    ref_price: f32,
) {
    use chrono::{Local, NaiveTime};
    use plotters::prelude::*;

    if data.is_empty() {
        return;
    }

    // Trading sessions: 9:00-11:30 and 13:00-14:45
    let morning_start = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
    let morning_end = NaiveTime::from_hms_opt(11, 30, 0).unwrap();
    let afternoon_start = NaiveTime::from_hms_opt(13, 0, 0).unwrap();
    let afternoon_end = NaiveTime::from_hms_opt(14, 45, 0).unwrap();

    // Separate data into trading sessions
    let mut morning_data = Vec::new();
    let mut afternoon_data = Vec::new();

    for candle in data {
        let time = candle.time.with_timezone(&Local).time();
        if time >= morning_start && time <= morning_end {
            morning_data.push(candle);
        } else if time >= afternoon_start && time <= afternoon_end {
            afternoon_data.push(candle);
        }
    }

    if morning_data.is_empty() && afternoon_data.is_empty() {
        return;
    }

    let all_data = [&morning_data[..], &afternoon_data[..]].concat();
    let min_price = all_data.iter().map(|c| c.low).fold(f32::INFINITY, f32::min);
    let max_price = all_data
        .iter()
        .map(|c| c.high)
        .fold(f32::NEG_INFINITY, f32::max);
    // let max_volume = all_data.iter().map(|c| c.volume).fold(0.0f32, f32::max);

    // Reserve space for volume at bottom (18 pixels) and time labels (12 pixels)
    let volume_height = 18;
    let _price_chart_height = height - volume_height - 17;

    // Create price chart area with proper margins
    let chart_area = root.margin(5, 5, 30, 10);
    let mut chart = ChartBuilder::on(&chart_area)
        .build_cartesian_2d(0f32..390f32, min_price..max_price)
        .unwrap();

    // Draw dashed reference line (previous day's close price)
    let dash_length = 5.0f32;
    let gap_length = 3.0f32;
    let mut x = 0.0f32;
    let mut dash_segments = Vec::new();

    while x < 390.0f32 {
        let end_x = (x + dash_length).min(390.0f32);
        dash_segments.push(PathElement::new(
            vec![(x, ref_price), (end_x, ref_price)],
            RGBColor(255, 255, 255).stroke_width(1),
        ));
        x += dash_length + gap_length;
    }

    chart.draw_series(dash_segments).ok();

    // Create price line with proper lunch break gap and color coding
    let mut green_segments = Vec::new();
    let mut red_segments = Vec::new();
    let mut all_price_points = Vec::new();
    let mut all_data_points = Vec::new(); // Store (x, price, volume) for each data point

    // Morning session: map to 0-150 (representing 9:00-11:30)
    for (i, candle) in morning_data.iter().enumerate() {
        let x = (i as f32 / morning_data.len().max(1) as f32) * 150.0;
        all_price_points.push((x, candle.close));
        all_data_points.push((x, candle.close, candle.volume));
    }

    // Add lunch break connection (straight line from 11:30 to 13:00)
    if !morning_data.is_empty() && !afternoon_data.is_empty() {
        let last_morning_price = morning_data.last().unwrap().close;
        let first_afternoon_price = afternoon_data.first().unwrap().close;

        // Connect 11:30 to 13:00 with straight line (no volume data for connecting line)
        all_price_points.push((150.0, last_morning_price));
        all_price_points.push((240.0, first_afternoon_price));
    }

    // Afternoon session: map to 240-390 (representing 13:00-14:45)
    for (i, candle) in afternoon_data.iter().enumerate() {
        let x = 240.0 + (i as f32 / afternoon_data.len().max(1) as f32) * 150.0;
        all_price_points.push((x, candle.close));
        all_data_points.push((x, candle.close, candle.volume));
    }

    // Separate points into green (above ref_price) and red (below ref_price) segments
    if all_price_points.len() > 1 {
        for i in 0..all_price_points.len() - 1 {
            let current = all_price_points[i];
            let next = all_price_points[i + 1];

            // Determine color based on price relative to reference
            if current.1 >= ref_price && next.1 >= ref_price {
                // Both points above reference - green
                green_segments.push(vec![current, next]);
            } else if current.1 < ref_price && next.1 < ref_price {
                // Both points below reference - red
                red_segments.push(vec![current, next]);
            } else {
                // Points cross reference line - need to split
                let intersection_x = current.0
                    + (next.0 - current.0) * (ref_price - current.1) / (next.1 - current.1);
                let intersection_point = (intersection_x, ref_price);

                if current.1 >= ref_price {
                    green_segments.push(vec![current, intersection_point]);
                    red_segments.push(vec![intersection_point, next]);
                } else {
                    red_segments.push(vec![current, intersection_point]);
                    green_segments.push(vec![intersection_point, next]);
                }
            }
        }
    }

    // Draw green segments (above reference price)
    for segment in green_segments {
        chart
            .draw_series(LineSeries::new(
                segment,
                RGBColor(0, 255, 0).stroke_width(1),
            ))
            .ok();
    }

    // Draw red segments (below reference price)
    for segment in red_segments {
        chart
            .draw_series(LineSeries::new(
                segment,
                RGBColor(255, 0, 0).stroke_width(1),
            ))
            .ok();
    }

    // Draw latest price in center
    let latest_price = all_data.last().unwrap().close;
    root.draw(&Text::new(
        format!("{latest_price:.1}"),
        (width as i32 / 2, (height / 2) as i32),
        ("sans-serif", 14).into_font().color(&WHITE),
    ))
    .ok();
}
