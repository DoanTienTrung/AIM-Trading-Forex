use crate::{
    CandleData, Chart, CompanyInfo, UiData,
    chart::{BackupChartOffset, ChartData},
    draw::DrawObject,
};

impl Chart {
    // Manual serialization: write Chart to bytes
    pub fn write_to_bytes(&self, buf: &mut Vec<u8>) {
        // stock_name: String
        let name_bytes = self.stock_name.as_bytes();
        let name_len = name_bytes.len() as u32;
        buf.extend_from_slice(&name_len.to_le_bytes());
        buf.extend_from_slice(name_bytes);

        // company_info: CompanyInfo
        buf.extend_from_slice(&self.company_info.roe.to_le_bytes());
        buf.extend_from_slice(&self.company_info.roa.to_le_bytes());
        buf.extend_from_slice(&self.company_info.pe.to_le_bytes());
        buf.extend_from_slice(&self.company_info.pb.to_le_bytes());
        buf.extend_from_slice(&self.company_info.eps.to_le_bytes());

        // candle_data: CandleDataVec (assume Vec<CandleData>)
        let candle_count = self.candle_data.len() as u32;
        buf.extend_from_slice(&candle_count.to_le_bytes());
        for _candle in &self.candle_data {
            _candle.write_to_bytes(buf);
        }

        // chart_data: ChartData
        self.chart_data.write_to_bytes(buf);

        // current_draw_data: DrawObject
        self.current_draw_data.write_to_bytes(buf);

        // moving_object: DrawObject
        self.moving_object.write_to_bytes(buf);

        // all_draw_data: Vec<DrawObject>
        let draw_count = self.all_draw_data.len() as u32;
        buf.extend_from_slice(&draw_count.to_le_bytes());
        for _draw in &self.all_draw_data {
            _draw.write_to_bytes(buf);
        }

        // backup_points: Vec<Point>
        let point_count = self.backup_points.len() as u32;
        buf.extend_from_slice(&point_count.to_le_bytes());
        for point in &self.backup_points {
            buf.extend_from_slice(&point.0.to_le_bytes());
            buf.extend_from_slice(&point.1.to_le_bytes());
        }

        // backup_chart_offset: Option<BackupChartOffset>
        match &self.backup_chart_offset {
            Some(offset) => {
                buf.push(1);
                buf.extend_from_slice(&offset.y_offset_min.to_le_bytes());
                buf.extend_from_slice(&offset.y_offset_max.to_le_bytes());
                buf.extend_from_slice(&offset.x_offset_min.to_le_bytes());
                buf.extend_from_slice(&offset.x_offset_max.to_le_bytes());
            }
            None => buf.push(0),
        }

        // is_in_object: (usize, bool, bool)
        buf.extend_from_slice(&(self.is_in_object.0 as u64).to_le_bytes());
        buf.push(self.is_in_object.1 as u8);
        buf.push(self.is_in_object.2 as u8);

        // delta: (f32, f32)
        buf.extend_from_slice(&self.delta.0.to_le_bytes());
        buf.extend_from_slice(&self.delta.1.to_le_bytes());
    }

    // Manual deserialization: read Chart from bytes
    pub fn read_from_bytes(data: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;
        // stock_name: String
        if data.len() < pos + 4 {
            return None;
        }
        let name_len =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        if data.len() < pos + name_len {
            return None;
        }
        let stock_name = String::from_utf8_lossy(&data[pos..pos + name_len]).to_string();
        pos += name_len;

        // company_info: CompanyInfo
        if data.len() < pos + 8 * 5 {
            return None;
        }
        let roe = f64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let roa = f64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let pe = f64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let pb = f64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let eps = f64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let company_info = CompanyInfo {
            roe,
            roa,
            pe,
            pb,
            eps,
        };

        // candle_data: CandleDataVec
        if data.len() < pos + 4 {
            return None;
        }
        let candle_count =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        let mut candle_data = Vec::with_capacity(candle_count);
        for _ in 0..candle_count {
            if let Some((candle, used)) = CandleData::read_from_bytes(&data[pos..]) {
                candle_data.push(candle);
                pos += used;
            } else {
                return None;
            }
        }

        // chart_data: ChartData
        let chart_data = match ChartData::read_from_bytes(&data[pos..]) {
            Some((chart_data, used)) => {
                pos += used;
                chart_data
            }
            None => return None,
        };

        // current_draw_data: DrawObject
        let (current_draw_data, used) = match DrawObject::read_from_bytes(&data[pos..]) {
            Some((obj, used)) => (obj, used),
            None => return None,
        };
        pos += used;

        // moving_object: DrawObject
        let (moving_object, used) = match DrawObject::read_from_bytes(&data[pos..]) {
            Some((obj, used)) => (obj, used),
            None => return None,
        };
        pos += used;

        // all_draw_data: Vec<DrawObject>
        if data.len() < pos + 4 {
            return None;
        }
        let draw_count =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        let mut all_draw_data = Vec::with_capacity(draw_count);
        for _ in 0..draw_count {
            if let Some((draw, used)) = DrawObject::read_from_bytes(&data[pos..]) {
                all_draw_data.push(draw);
                pos += used;
            } else {
                return None;
            }
        }

        // backup_points: Vec<Point>
        if data.len() < pos + 4 {
            return None;
        }
        let point_count =
            u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        let mut backup_points = Vec::with_capacity(point_count);
        for _ in 0..point_count {
            if data.len() < pos + 8 {
                return None;
            }
            let x = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let y = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            backup_points.push((x, y));
        }

        // backup_chart_offset: Option<BackupChartOffset>
        if data.len() < pos + 1 {
            return None;
        }
        let has_offset = data[pos] != 0;
        pos += 1;
        let backup_chart_offset = if has_offset {
            if data.len() < pos + 16 {
                return None;
            }
            let y_offset_min = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let y_offset_max = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let x_offset_min = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let x_offset_max = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
            pos += 4;
            Some(BackupChartOffset {
                y_offset_min,
                y_offset_max,
                x_offset_min,
                x_offset_max,
            })
        } else {
            None
        };

        // is_in_object: (usize, bool, bool)
        if data.len() < pos + 10 {
            return None;
        }
        let idx =
            usize::try_from(u64::from_le_bytes(data[pos..pos + 8].try_into().unwrap())).unwrap();
        pos += 8;
        let b1 = data[pos] != 0;
        pos += 1;
        let b2 = data[pos] != 0;
        pos += 1;
        let is_in_object = (idx, b1, b2);

        // delta: (f32, f32)
        if data.len() < pos + 8 {
            return None;
        }
        let dx = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let dy = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let delta = (dx, dy);

        Some((
            Self {
                stock_name,
                company_info,
                candle_data,
                chart_data,
                current_draw_data,
                moving_object,
                all_draw_data,
                backup_points,
                backup_chart_offset,
                is_in_object,
                delta,
            },
            pos,
        ))
    }

    // Helper for hashing: serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.write_to_bytes(&mut buf);
        buf
    }
}

// Manual serialization for ChartData
impl ChartData {
    // Manual deserialization: read ChartData from bytes
    pub fn read_from_bytes(data: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;
        if data.len() < pos + 4 * 11 {
            return None;
        }
        let y_offset_min = f32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let y_offset_max = f32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let x_offset_min = f32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let x_offset_max = f32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let range_x = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let range_y = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let width = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let height = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let candle_distance = f32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let zoom_x = f32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let zoom_y = f32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        // Deserialize ui_data (basic fields only, must match write_to_bytes)
        if data.len() < pos + 4 * 8 + 3 {
            return None;
        }
        let position_x = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let position_y = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let press_x = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let press_y = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let move_x = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let move_y = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let zoom = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let is_clean = data[pos] != 0;
        pos += 1;
        let is_undo = data[pos] != 0;
        pos += 1;
        let is_release = data[pos] != 0;
        pos += 1;
        let height_ui = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        let width_ui = i32::from_le_bytes(data[pos..pos + 4].try_into().ok()?);
        pos += 4;
        // Color and type can be added if needed
        let ui_data = UiData {
            position_x,
            position_y,
            press_x,
            press_y,
            move_x,
            move_y,
            zoom,
            is_clean,
            is_undo,
            is_release,
            height: height_ui,
            width: width_ui,
            ..UiData::default()
        };
        Some((
            Self {
                y_offset_min,
                y_offset_max,
                x_offset_min,
                x_offset_max,
                range_x,
                range_y,
                width,
                height,
                candle_distance,
                zoom_x,
                zoom_y,
                ui_data,
            },
            pos,
        ))
    }
    pub fn write_to_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.y_offset_min.to_le_bytes());
        buf.extend_from_slice(&self.y_offset_max.to_le_bytes());
        buf.extend_from_slice(&self.x_offset_min.to_le_bytes());
        buf.extend_from_slice(&self.x_offset_max.to_le_bytes());
        buf.extend_from_slice(&self.range_x.to_le_bytes());
        buf.extend_from_slice(&self.range_y.to_le_bytes());
        buf.extend_from_slice(&self.width.to_le_bytes());
        buf.extend_from_slice(&self.height.to_le_bytes());
        buf.extend_from_slice(&self.candle_distance.to_le_bytes());
        buf.extend_from_slice(&self.zoom_x.to_le_bytes());
        buf.extend_from_slice(&self.zoom_y.to_le_bytes());
        // Serialize ui_data (basic fields only, expand as needed)
        buf.extend_from_slice(&self.ui_data.position_x.to_le_bytes());
        buf.extend_from_slice(&self.ui_data.position_y.to_le_bytes());
        buf.extend_from_slice(&self.ui_data.press_x.to_le_bytes());
        buf.extend_from_slice(&self.ui_data.press_y.to_le_bytes());
        buf.extend_from_slice(&self.ui_data.move_x.to_le_bytes());
        buf.extend_from_slice(&self.ui_data.move_y.to_le_bytes());
        buf.extend_from_slice(&self.ui_data.zoom.to_le_bytes());
        buf.push(self.ui_data.is_clean as u8);
        buf.push(self.ui_data.is_undo as u8);
        buf.push(self.ui_data.is_release as u8);
        buf.extend_from_slice(&self.ui_data.height.to_le_bytes());
        buf.extend_from_slice(&self.ui_data.width.to_le_bytes());
        // Color and type can be added if needed
    }
}
