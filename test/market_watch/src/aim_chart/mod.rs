pub mod mini_chart;
use chrono::{DateTime, Utc};

use crate::aim_data::Candlestick;

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
