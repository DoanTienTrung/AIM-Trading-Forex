use crate::slint_generatedAppWindow::{SectorRangeSnapshot, SectorTimePoint};
use slint::Model;

/// Số lớp cố định: Tài chính, BĐS, Tiêu dùng, Ngành khác
const LAYERS: usize = 4;

fn total(point: &SectorTimePoint) -> f32 {
    point.finance + point.real_estate + point.consumer + point.others
}

/// Trả về (lower, upper) của layer `layer` tại 1 thời điểm, đã chuẩn hóa theo tổng (0.0–1.0)
fn band_bounds(point: &SectorTimePoint, layer: usize) -> (f32, f32) {
    let sum = total(point).max(1e-6);
    let w_f = point.finance / sum;
    let w_re = point.real_estate / sum;
    let w_c = point.consumer / sum;
    let w_o = point.others / sum;

    // Sắp xếp dải từ dưới lên: Ngành khác, Tiêu dùng, BĐS, Tài chính.
    // Layer index theo thứ tự hiển thị: 0 = Tài chính (trên cùng), 1 = BĐS,
    // 2 = Tiêu dùng, 3 = Ngành khác (dưới cùng).
    match layer {
        // Tài chính: dải trên cùng
        0 => {
            let lower = w_o + w_c + w_re;
            let upper = 1.0;
            (lower, upper)
        }
        // Bất động sản
        1 => {
            let lower = w_o + w_c;
            let upper = w_o + w_c + w_re;
            (lower, upper)
        }
        // Tiêu dùng
        2 => {
            let lower = w_o;
            let upper = w_o + w_c;
            (lower, upper)
        }
        // Ngành khác: dải dưới cùng
        _ => {
            let lower = 0.0;
            let upper = w_o;
            (lower, upper)
        }
    }
}

/// Sinh path cho một lớp, dạng polygon đóng, đã smoothed theo trục X.
pub fn build_layer_path(
    points: &[SectorTimePoint],
    layer: usize,
    width: f32,
    height: f32,
) -> String {
    if points.len() < 2 || layer >= LAYERS {
        return format!(
            "M 0 {} L {} {} L 0 {} Z",
            height, width, height, height
        );
    }

    let step_x = width / (points.len() - 1) as f32;

    // Forward: biên TRÊN (upper) để thể hiện biến động rõ hơn
    let mut path = String::new();
    let mut prev_x = 0.0f32;
    let mut prev_y = 0.0f32;

    for (i, p) in points.iter().enumerate() {
        let (_, upper) = band_bounds(p, layer);
        let x = step_x * i as f32;
        let y = height * (1.0 - upper);

        if i == 0 {
            path.push_str(&format!("M {:.2} {:.2}", x, y));
        } else {
            let ctrl = (prev_x + x) * 0.5;
            path.push_str(&format!(
                " C {:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                ctrl, prev_y, ctrl, y, x, y
            ));
        }

        prev_x = x;
        prev_y = y;
    }

    // Backward: biên DƯỚI (lower) để đóng polygon
    let mut prev_x = step_x * (points.len() - 1) as f32;
    let mut prev_y = height;

    for rev_i in 0..points.len() {
        let i = points.len() - 1 - rev_i;
        let p = &points[i];
        let (lower, _) = band_bounds(p, layer);
        let x = step_x * i as f32;
        let y = height * (1.0 - lower);

        if rev_i == 0 {
            path.push_str(&format!(" L {:.2} {:.2}", x, y));
        } else {
            let ctrl = (prev_x + x) * 0.5;
            path.push_str(&format!(
                " C {:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                ctrl, prev_y, ctrl, y, x, y
            ));
        }

        prev_x = x;
        prev_y = y;
    }

    path.push_str(" Z");
    path
}

/// Từ một `SectorRangeSnapshot` + kích thước chart, sinh ra 4 path stacked 100%.
pub fn build_stream_paths_for_range(
    range: &SectorRangeSnapshot,
    width: f32,
    height: f32,
) -> (String, String, String, String) {
    // Chuyển ModelRc<SectorTimePoint> -> Vec<SectorTimePoint>
    let count = range.timeline.row_count();
    let pts: Vec<SectorTimePoint> = if count == 0 {
        // Nếu chưa có dữ liệu thời gian, tạo 3 điểm phẳng dựa trên phần trăm hiện tại
        let p = SectorTimePoint {
            label: range.label.clone(),
            finance: range.finance_percent,
            real_estate: range.real_estate_percent,
            consumer: range.consumer_percent,
            others: range.others_percent,
        };
        vec![p.clone(), p.clone(), p]
    } else {
        (0..count)
            .filter_map(|i| range.timeline.row_data(i))
            .collect()
    };

    let f = build_layer_path(&pts, 0, width, height);
    let re = build_layer_path(&pts, 1, width, height);
    let c = build_layer_path(&pts, 2, width, height);
    let o = build_layer_path(&pts, 3, width, height);

    (f, re, c, o)
}


