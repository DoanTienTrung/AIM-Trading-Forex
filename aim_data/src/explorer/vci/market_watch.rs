use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketWatchResponse(pub Vec<VCIMarketWatch>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VCIMarketWatch {
    #[serde(rename = "listingInfo")]
    pub listing_info: ListingInfo,
    #[serde(rename = "bidAsk")]
    pub bid_ask: BidAsk,
    #[serde(rename = "matchPrice")]
    pub match_price: MatchPrice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidAsk {
    pub code: String,
    pub symbol: String,
    #[serde(rename = "bidPrices")]
    pub bid_prices: Vec<PriceVolume>,
    #[serde(rename = "askPrices")]
    pub ask_prices: Vec<PriceVolume>,
    // pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceVolume {
    pub price: f64,
    pub volume: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingInfo {
    pub code: String,
    pub symbol: String,
    pub ceiling: f64,
    pub floor: f64,
    #[serde(rename = "refPrice")]
    pub ref_price: f64,
    pub board: String,
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
    // pub accumulated_value: f64,
    // #[serde(rename = "avgMatchPrice")]
    // pub avg_match_price: f64,
    pub highest: f64,
    pub lowest: f64,
    // pub time: DateTime<Utc>,
    // pub session: String,
    // #[serde(rename = "matchType")]
    // pub match_type: String,
    // #[serde(rename = "foreignSellVolume")]
    // pub foreign_sell_volume: i64,
    // #[serde(rename = "foreignBuyVolume")]
    // pub foreign_buy_volume: i64,
    // #[serde(rename = "currentRoom")]
    // pub current_room: i64,
    #[serde(rename = "referencePrice")]
    pub reference_price: f64,
}
