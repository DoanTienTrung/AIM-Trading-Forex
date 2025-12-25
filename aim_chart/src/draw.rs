use plotters::style::RGBColor;

pub type Point = (f32, f32);
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DrawType {
    Line((Point, Point)),
    Arrow((Point, Point)),
    Rectangle((Point, Point)),
    Oval((Point, Point)),
    Ruler((Point, Point)),
    VerticalLine(f32),
    HorizontalLine(f32),
    Pen(Vec<Point>),
    #[default]
    Empty,
}

#[derive(Debug, Clone, Default)]
pub struct DrawObject {
    pub draw_type: DrawType,
    pub color: RGBColor,
}

// Manual serialization for DrawType
pub fn draw_type_write_to_bytes(draw_type: &DrawType, buf: &mut Vec<u8>) {
    match draw_type {
        DrawType::Line((a, b)) => {
            buf.push(1);
            buf.extend_from_slice(&a.0.to_le_bytes());
            buf.extend_from_slice(&a.1.to_le_bytes());
            buf.extend_from_slice(&b.0.to_le_bytes());
            buf.extend_from_slice(&b.1.to_le_bytes());
        }
        DrawType::Arrow((a, b)) => {
            buf.push(2);
            buf.extend_from_slice(&a.0.to_le_bytes());
            buf.extend_from_slice(&a.1.to_le_bytes());
            buf.extend_from_slice(&b.0.to_le_bytes());
            buf.extend_from_slice(&b.1.to_le_bytes());
        }
        DrawType::Rectangle((a, b)) => {
            buf.push(3);
            buf.extend_from_slice(&a.0.to_le_bytes());
            buf.extend_from_slice(&a.1.to_le_bytes());
            buf.extend_from_slice(&b.0.to_le_bytes());
            buf.extend_from_slice(&b.1.to_le_bytes());
        }
        DrawType::Oval((a, b)) => {
            buf.push(4);
            buf.extend_from_slice(&a.0.to_le_bytes());
            buf.extend_from_slice(&a.1.to_le_bytes());
            buf.extend_from_slice(&b.0.to_le_bytes());
            buf.extend_from_slice(&b.1.to_le_bytes());
        }
        DrawType::Ruler((a, b)) => {
            buf.push(5);
            buf.extend_from_slice(&a.0.to_le_bytes());
            buf.extend_from_slice(&a.1.to_le_bytes());
            buf.extend_from_slice(&b.0.to_le_bytes());
            buf.extend_from_slice(&b.1.to_le_bytes());
        }
        DrawType::VerticalLine(x) => {
            buf.push(6);
            buf.extend_from_slice(&x.to_le_bytes());
        }
        DrawType::HorizontalLine(y) => {
            buf.push(7);
            buf.extend_from_slice(&y.to_le_bytes());
        }
        DrawType::Pen(points) => {
            buf.push(8);
            let count = points.len() as u32;
            buf.extend_from_slice(&count.to_le_bytes());
            for p in points {
                buf.extend_from_slice(&p.0.to_le_bytes());
                buf.extend_from_slice(&p.1.to_le_bytes());
            }
        }
        DrawType::Empty => {
            buf.push(0);
        }
    }
}

pub fn draw_type_read_from_bytes(data: &[u8]) -> Option<(DrawType, usize)> {
    if data.is_empty() {
        return None;
    }
    let tag = data[0];
    let mut pos = 1;
    match tag {
        1..=5 => {
            if data.len() < pos + 16 {
                return None;
            }
            let a0 = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let a1 = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let b0 = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let b1 = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let points = ((a0, a1), (b0, b1));
            let dt = match tag {
                1 => DrawType::Line(points),
                2 => DrawType::Arrow(points),
                3 => DrawType::Rectangle(points),
                4 => DrawType::Oval(points),
                5 => DrawType::Ruler(points),
                _ => unreachable!(),
            };
            Some((dt, pos))
        }
        6 => {
            if data.len() < pos + 4 {
                return None;
            }
            let x = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            Some((DrawType::VerticalLine(x), pos))
        }
        7 => {
            if data.len() < pos + 4 {
                return None;
            }
            let y = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            Some((DrawType::HorizontalLine(y), pos))
        }
        8 => {
            if data.len() < pos + 4 {
                return None;
            }
            let count = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;
            let mut points = Vec::with_capacity(count);
            for _ in 0..count {
                if data.len() < pos + 8 {
                    return None;
                }
                let x = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
                pos += 4;
                let y = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
                pos += 4;
                points.push((x, y));
            }
            Some((DrawType::Pen(points), pos))
        }
        0 => Some((DrawType::Empty, pos)),
        _ => None,
    }
}

impl DrawObject {
    pub fn write_to_bytes(&self, buf: &mut Vec<u8>) {
        draw_type_write_to_bytes(&self.draw_type, buf);
        buf.push(self.color.0);
        buf.push(self.color.1);
        buf.push(self.color.2);
    }

    pub fn read_from_bytes(data: &[u8]) -> Option<(Self, usize)> {
        let (draw_type, used1) = draw_type_read_from_bytes(data)?;
        if data.len() < used1 + 3 {
            return None;
        }
        let r = data[used1];
        let g = data[used1 + 1];
        let b = data[used1 + 2];
        let color = RGBColor(r, g, b);
        Some((Self { draw_type, color }, used1 + 3))
    }

    pub fn new(draw_type: DrawType, color: RGBColor) -> Self {
        Self { draw_type, color }
    }

    pub fn hit_cursor(&self, point: Point, x_thresh_hold: f32, y_thresh_hold: f32) -> bool {
        match &self.draw_type {
            DrawType::Line((start, end)) | DrawType::Arrow((start, end)) => {
                let (x1, y1) = *start;
                let (x2, y2) = *end;
                let (px, py) = point;

                let dx = x2 - x1;
                let dy = y2 - y1;
                let length_sq = dx * dx + dy * dy;

                if length_sq == 0.0 {
                    (px - x1).abs() < x_thresh_hold && (py - y1).abs() < y_thresh_hold
                } else {
                    // Project point onto the segment, clamp t to [0, 1]
                    let t = ((px - x1) * dx + (py - y1) * dy) / length_sq;
                    let t = t.clamp(0.0, 1.0);
                    let proj_x = x1 + t * dx;
                    let proj_y = y1 + t * dy;
                    (px - proj_x).abs() < x_thresh_hold && (py - proj_y).abs() < y_thresh_hold
                }
            }
            DrawType::Rectangle((start, end)) => {
                let (x1, y1) = *start;
                let (x2, y2) = *end;
                let (px, py) = point;

                let min_x = x1.min(x2);
                let max_x = x1.max(x2);
                let min_y = y1.min(y2);
                let max_y = y1.max(y2);

                // Check if point is near any of the four edges
                let near_left = (px - min_x).abs() < x_thresh_hold
                    && py >= min_y - y_thresh_hold
                    && py <= max_y + y_thresh_hold;
                let near_right = (px - max_x).abs() < x_thresh_hold
                    && py >= min_y - y_thresh_hold
                    && py <= max_y + y_thresh_hold;
                let near_top = (py - min_y).abs() < y_thresh_hold
                    && px >= min_x - x_thresh_hold
                    && px <= max_x + x_thresh_hold;
                let near_bottom = (py - max_y).abs() < y_thresh_hold
                    && px >= min_x - x_thresh_hold
                    && px <= max_x + x_thresh_hold;

                near_left || near_right || near_top || near_bottom
            }
            DrawType::Oval((start, end)) => {
                let (x1, y1) = *start;
                let (x2, y2) = *end;
                let (px, py) = point;

                // Calculate ellipse center and radii
                let center_x = (x1 + x2) / 2.0;
                let center_y = (y1 + y2) / 2.0;
                let rx = (x2 - x1).abs() / 2.0;
                let ry = (y2 - y1).abs() / 2.0;

                if rx == 0.0 || ry == 0.0 {
                    return false;
                }

                // Check if point is near the ellipse boundary
                let dx = px - center_x;
                let dy = py - center_y;
                let ellipse_eq = (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry);

                // Check if point is close to the ellipse boundary (ellipse_eq ≈ 1)
                let threshold = (x_thresh_hold / rx).max(y_thresh_hold / ry);
                (ellipse_eq - 1.0).abs() < threshold
            }
            DrawType::VerticalLine(x) => {
                let (px, _) = point;
                (*x - px).abs() < x_thresh_hold
            }
            DrawType::HorizontalLine(y) => {
                let (_, py) = point;
                (*y - py).abs() < y_thresh_hold
            }
            DrawType::Pen(points) => {
                // Check if the cursor is close to any segment of the pen path
                points.windows(2).any(|w| {
                    let (x1, y1) = w[0];
                    let (x2, y2) = w[1];
                    let (px, py) = point;

                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let length_sq = dx * dx + dy * dy;
                    if length_sq == 0.0 {
                        // Single point
                        ((px - x1).powi(2) + (py - y1).powi(2)).sqrt() < x_thresh_hold
                    } else {
                        // Project point onto the segment
                        let t = ((px - x1) * dx + (py - y1) * dy) / length_sq;
                        let t = t.clamp(0.0, 1.0);
                        let proj_x = x1 + t * dx;
                        let proj_y = y1 + t * dy;
                        ((px - proj_x).powi(2) + (py - proj_y).powi(2)).sqrt() < y_thresh_hold
                    }
                })
            }
            _ => false,
        }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        match &mut self.draw_type {
            DrawType::Line((start, end)) | DrawType::Arrow((start, end)) => {
                start.0 += dx;
                start.1 += dy;
                end.0 += dx;
                end.1 += dy;
            }
            DrawType::Rectangle((start, end)) | DrawType::Oval((start, end)) => {
                start.0 += dx;
                start.1 += dy;
                end.0 += dx;
                end.1 += dy;
            }
            DrawType::VerticalLine(x) => {
                *x += dx;
            }
            DrawType::HorizontalLine(y) => {
                *y += dy;
            }
            DrawType::Pen(points) => {
                for p in points.iter_mut() {
                    p.0 += dx;
                    p.1 += dy;
                }
            }
            _ => {}
        }
    }

    pub fn clipped(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        match &mut self.draw_type {
            DrawType::Line((start, end)) | DrawType::Arrow((start, end)) => {
                // Cohen–Sutherland line clipping algorithm (simple version)
                let mut p1 = *start;
                let mut p2 = *end;
                if !cohen_sutherland_clip(&mut p1, &mut p2, x_min, x_max, y_min, y_max) {
                    // If the line is completely outside, make it empty
                    self.draw_type = DrawType::Empty;
                } else {
                    *start = p1;
                    *end = p2;
                }
            }
            DrawType::Rectangle((start, end)) => {
                // Clamp rectangle to chart area
                let min_x = start.0.min(end.0).max(x_min);
                let max_x = start.0.max(end.0).min(x_max);
                let min_y = start.1.min(end.1).max(y_min);
                let max_y = start.1.max(end.1).min(y_max);
                if min_x >= max_x || min_y >= max_y {
                    self.draw_type = DrawType::Empty;
                } else {
                    *start = (min_x, min_y);
                    *end = (max_x, max_y);
                }
            }
            DrawType::Oval((start, end)) => {
                // Clamp oval bounding box to chart area
                let min_x = start.0.min(end.0).max(x_min);
                let max_x = start.0.max(end.0).min(x_max);
                let min_y = start.1.min(end.1).max(y_min);
                let max_y = start.1.max(end.1).min(y_max);
                if min_x >= max_x || min_y >= max_y {
                    self.draw_type = DrawType::Empty;
                } else {
                    *start = (min_x, min_y);
                    *end = (max_x, max_y);
                }
            }
            DrawType::VerticalLine(x) => {
                if *x < x_min || *x > x_max {
                    self.draw_type = DrawType::Empty;
                }
            }
            DrawType::HorizontalLine(y) => {
                if *y < y_min || *y > y_max {
                    self.draw_type = DrawType::Empty;
                }
            }
            DrawType::Pen(points) => {
                // Remove points outside the bounds
                points.retain(|&(px, py)| px >= x_min && px <= x_max && py >= y_min && py <= y_max);
                if points.len() < 2 {
                    self.draw_type = DrawType::Empty;
                }
            }
            _ => {}
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.draw_type, DrawType::Empty)
    }

    pub fn clear(&mut self) {
        self.draw_type = DrawType::Empty;
    }

    pub fn to_vec(&self, p1: Point, p2: Point) -> Vec<Point> {
        match &self.draw_type {
            DrawType::Line((start, end)) | DrawType::Arrow((start, end)) => vec![*start, *end],
            DrawType::Rectangle((start, end)) => {
                vec![*start, (end.0, start.1), *end, (start.0, end.1), *start]
            }
            DrawType::Oval((start, end)) => {
                // Generate points for an ellipse
                let center_x = (start.0 + end.0) / 2.0;
                let center_y = (start.1 + end.1) / 2.0;
                let rx = (end.0 - start.0).abs() / 2.0;
                let ry = (end.1 - start.1).abs() / 2.0;

                let mut points = Vec::new();
                let num_points = 64; // Number of points to approximate the ellipse

                for i in 0..=num_points {
                    let angle = 2.0 * std::f32::consts::PI * i as f32 / num_points as f32;
                    let x = center_x + rx * angle.cos();
                    let y = center_y + ry * angle.sin();
                    points.push((x, y));
                }
                points
            }
            DrawType::VerticalLine(x) => vec![(*x, p1.1), (*x, p2.1)],
            DrawType::HorizontalLine(y) => vec![(p1.0, *y), (p2.0, *y)],
            DrawType::Pen(points) => points.clone(),
            _ => vec![],
        }
    }
}

// Helper: Cohen–Sutherland line clipping algorithm for 2D lines
fn cohen_sutherland_clip(
    p1: &mut (f32, f32),
    p2: &mut (f32, f32),
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> bool {
    const INSIDE: u8 = 0;
    const LEFT: u8 = 1;
    const RIGHT: u8 = 2;
    const BOTTOM: u8 = 4;
    const TOP: u8 = 8;

    fn compute_out_code(x: f32, y: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> u8 {
        let mut code = INSIDE;
        if x < x_min {
            code |= LEFT;
        } else if x > x_max {
            code |= RIGHT;
        }
        if y < y_min {
            code |= BOTTOM;
        } else if y > y_max {
            code |= TOP;
        }
        code
    }

    let (mut x0, mut y0) = *p1;
    let (mut x1, mut y1) = *p2;
    let mut out_code0 = compute_out_code(x0, y0, x_min, x_max, y_min, y_max);
    let mut out_code1 = compute_out_code(x1, y1, x_min, x_max, y_min, y_max);

    loop {
        if (out_code0 | out_code1) == 0 {
            // Both points inside
            *p1 = (x0, y0);
            *p2 = (x1, y1);
            return true;
        } else if (out_code0 & out_code1) != 0 {
            // Both points share an outside zone
            return false;
        } else {
            let out_code_out = if out_code0 != 0 { out_code0 } else { out_code1 };
            let (x, y);
            if out_code_out & TOP != 0 {
                x = x0 + (x1 - x0) * (y_max - y0) / (y1 - y0);
                y = y_max;
            } else if out_code_out & BOTTOM != 0 {
                x = x0 + (x1 - x0) * (y_min - y0) / (y1 - y0);
                y = y_min;
            } else if out_code_out & RIGHT != 0 {
                y = y0 + (y1 - y0) * (x_max - x0) / (x1 - x0);
                x = x_max;
            } else {
                y = y0 + (y1 - y0) * (x_min - x0) / (x1 - x0);
                x = x_min;
            }
            if out_code_out == out_code0 {
                x0 = x;
                y0 = y;
                out_code0 = compute_out_code(x0, y0, x_min, x_max, y_min, y_max);
            } else {
                x1 = x;
                y1 = y;
                out_code1 = compute_out_code(x1, y1, x_min, x_max, y_min, y_max);
            }
        }
    }
}
