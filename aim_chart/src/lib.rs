mod cache;
mod chart;
mod company_info;
mod draw;
pub mod mini_chart;
pub use chart::Chart;
use chrono::{DateTime, Utc};
pub use company_info::CompanyInfo;

use aim_data::Candlestick;
use slint::Color;

#[derive(Debug, Clone)]
pub enum MouseType {
    Move,
    Draw,
    Rectangle,
    Oval,
    Line,
    Arrow,
    Ruler,
    Text,
    HorizontalLine,
    VerticalLine,
}

#[derive(Debug, Clone)]
pub struct UiData {
    pub ticker: String,
    pub mouse_type: MouseType,
    pub move_x: i32,
    pub move_y: i32,
    pub position_x: i32,
    pub press_x: i32,
    pub position_y: i32,
    pub press_y: i32,
    pub zoom: i32,
    pub is_release: bool,
    pub is_clean: bool,
    pub width: i32,
    pub height: i32,
    pub time_frame: String,
    pub is_new_time_frame: bool,
    pub is_new_stock: bool,
    pub is_in_object: bool,
    pub is_undo: bool,
    pub is_in_update: bool,
    pub color: Color,
}

impl Default for UiData {
    fn default() -> Self {
        Self {
            ticker: String::new(),
            mouse_type: MouseType::Move,
            move_x: 0,
            move_y: 0,
            position_x: 0,
            press_x: 0,
            position_y: 0,
            press_y: 0,
            zoom: 0,
            is_release: false,
            is_clean: false,
            width: 0,
            height: 0,
            time_frame: String::new(),
            is_new_time_frame: false,
            is_new_stock: false,
            is_in_object: false,
            is_undo: false,
            is_in_update: false,
            color: Color::default(),
        }
    }
}

type CandleDataVec = Vec<CandleData>;
#[derive(Debug, Clone)]
pub struct CandleData {
    num: f32,
    time: DateTime<Utc>,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: f32,
}
impl CandleData {
    pub fn close(&self) -> f32 {
        self.close
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn write_to_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.num.to_le_bytes());
        let timestamp = self.time.timestamp();
        buf.extend_from_slice(&timestamp.to_le_bytes());
        buf.extend_from_slice(&self.open.to_le_bytes());
        buf.extend_from_slice(&self.high.to_le_bytes());
        buf.extend_from_slice(&self.low.to_le_bytes());
        buf.extend_from_slice(&self.close.to_le_bytes());
        buf.extend_from_slice(&self.volume.to_le_bytes());
    }

    pub fn read_from_bytes(data: &[u8]) -> Option<(Self, usize)> {
        if data.len() < 4 + 8 + 4 * 6 {
            return None;
        }
        let mut pos = 0;
        let num = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let timestamp = i64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let time = chrono::DateTime::from_timestamp(timestamp, 0)?;
        let open = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let high = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let low = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let close = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let volume = f32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        pos += 4;
        Some((
            Self {
                num,
                time,
                open,
                high,
                low,
                close,
                volume,
            },
            pos,
        ))
    }
}

pub fn convert_candlesticks(is_stock: bool, data: Vec<Candlestick>) -> CandleDataVec {
    if is_stock {
        data.into_iter()
            .enumerate()
            .map(|(i, c)| CandleData {
                num: i as f32,
                time: c.timestamp,
                open: c.open as f32 / 1000.0,
                high: c.high as f32 / 1000.0,
                low: c.low as f32 / 1000.0,
                close: c.close as f32 / 1000.0,
                volume: c.volume as f32,
            })
            .collect()
    } else {
        data.into_iter()
            .enumerate()
            .map(|(i, c)| CandleData {
                num: i as f32,
                time: c.timestamp,
                open: c.open as f32,
                high: c.high as f32,
                low: c.low as f32,
                close: c.close as f32,
                volume: c.volume as f32,
            })
            .collect()
    }
}
