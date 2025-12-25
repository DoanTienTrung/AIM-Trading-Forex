use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VCIQuote {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: i64,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingInfo {
    pub code: String,
    pub symbol: String,
    pub ceiling: f64,
    pub floor: f64,
    #[serde(rename = "refPrice")]
    pub ref_price: f64,
    #[serde(rename = "stockType")]
    pub stock_type: String,
    pub board: String,
    pub r#type: String,
    #[serde(rename = "enOrganName")]
    pub en_organ_name: String,
    #[serde(rename = "enOrganShortName")]
    pub en_organ_short_name: String,
    #[serde(rename = "organName")]
    pub organ_name: String,
    #[serde(rename = "organShortName")]
    pub organ_short_name: String,
    pub ticker: String,
    #[serde(rename = "tradingDate")]
    pub trading_date: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPrice {
    pub code: String,
    pub symbol: String,
    #[serde(rename = "matchPrice")]
    pub match_price: f64,
    #[serde(rename = "matchVol")]
    pub match_vol: i64,
    #[serde(rename = "accumulatedVolume")]
    pub accumulated_volume: i64,
    #[serde(rename = "accumulatedValue")]
    pub accumulated_value: f64,
    #[serde(rename = "avgMatchPrice")]
    pub avg_match_price: f64,
    pub highest: f64,
    pub lowest: f64,
    pub time: DateTime<Utc>,
    pub session: String,
    #[serde(rename = "matchType")]
    pub match_type: String,
    #[serde(rename = "foreignSellVolume")]
    pub foreign_sell_volume: i64,
    #[serde(rename = "foreignBuyVolume")]
    pub foreign_buy_volume: i64,
    #[serde(rename = "currentRoom")]
    pub current_room: i64,
    #[serde(rename = "referencePrice")]
    pub reference_price: f64,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VCICompanyProfile {
    pub symbol: String,
    pub company_name: String,
    pub industry: String,
    pub website: Option<String>,
    pub description: Option<String>,
    pub listing_date: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct VCIFinancialReport {
    pub symbol: String,
    pub period: String,
    pub year: i32,
    pub revenue: f64,
    pub profit: f64,
    pub eps: f64,
    pub pe: f64,
    pub roe: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OHLCData {
    pub symbol: String,
    pub o: Vec<Option<f64>>, // Open prices
    pub h: Vec<Option<f64>>, // High prices
    pub l: Vec<Option<f64>>, // Low prices
    pub c: Vec<Option<f64>>, // Close prices
    pub v: Vec<Option<i64>>, // Volumes
    pub t: Vec<String>,      // Timestamps
    #[serde(rename = "accumulatedVolume")]
    pub accumulated_volume: Vec<Option<i64>>,
    #[serde(rename = "accumulatedValue")]
    pub accumulated_value: Vec<Option<f64>>,
    #[serde(rename = "minBatchTruncTime")]
    pub min_batch_trunc_time: String,
}

#[derive(Debug, Clone)]
pub struct Candlestick {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

impl OHLCData {
    pub fn to_candlesticks(&self) -> Vec<Candlestick> {
        let mut candlesticks = Vec::new();

        for i in 0..self.o.len() {
            // Parse timestamp as Unix timestamp (seconds)
            let timestamp = if let Ok(ts) = self.t[i].parse::<i64>() {
                // Convert from seconds to DateTime<Utc>
                DateTime::from_timestamp(ts, 0)
                    .unwrap_or_else(Utc::now)
                    .with_timezone(&Utc)
            } else {
                // If parsing fails, use current time
                eprintln!("Failed to parse timestamp: {}", &self.t[i]);
                Utc::now()
            };

            // Handle nullable values with defaults
            let open = self.o[i].unwrap_or(0.0);
            let high = self.h[i].unwrap_or(0.0);
            let low = self.l[i].unwrap_or(0.0);
            let close = self.c[i].unwrap_or(0.0);
            let volume = self.v[i].unwrap_or(0);

            candlesticks.push(Candlestick {
                timestamp,
                open,
                high,
                low,
                close,
                volume,
            });
        }

        candlesticks
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OHLCResponse(pub Vec<OHLCData>);
