use crate::slint_generatedAppWindow::{
    MarketWatchData as SlintMarketWatchData
};
use chrono::{Datelike, Timelike};

mod data_update;

pub use data_update::*;

use crate::aim_data::explorer::vci::market_watch::VCIMarketWatch;

const VN30_LIST: [&str; 30] = [
    "ACB", "BCM", "BID", "BVH", "CTG", "FPT", "GAS", "GVR", "HDB", "HPG", "LPB", "MBB", "MSN",
    "MWG", "PLX", "SAB", "SHB", "SSB", "SSI", "STB", "TCB", "TPB", "VCB", "VHM", "VIB", "VIC",
    "VJC", "VNM", "VPB", "VRE",
];

fn convert_to_market_data(market_watch: &VCIMarketWatch) -> SlintMarketWatchData {
    // Divide all price values by 1000 to get the actual price
    let price = market_watch.match_price.match_price / 1000.0;
    let ref_price = market_watch.listing_info.ref_price / 1000.0;
    let change = price - ref_price;
    let change_percent = if ref_price != 0.0 {
        (change / ref_price) * 100.0
    } else {
        0.0
    };

    // Get bid/ask prices
    let (ask3_price, ask2_price, ask1_price) = if market_watch.bid_ask.ask_prices.len() >= 3 {
        (
            market_watch.bid_ask.ask_prices[2].price / 1000.0,
            market_watch.bid_ask.ask_prices[1].price / 1000.0,
            market_watch.bid_ask.ask_prices[0].price / 1000.0,
        )
    } else {
        (0.0, 0.0, 0.0)
    };

    let (bid1_price, bid2_price, bid3_price) = if market_watch.bid_ask.bid_prices.len() >= 3 {
        (
            market_watch.bid_ask.bid_prices[0].price / 1000.0,
            market_watch.bid_ask.bid_prices[1].price / 1000.0,
            market_watch.bid_ask.bid_prices[2].price / 1000.0,
        )
    } else {
        (0.0, 0.0, 0.0)
    };

    let volume = market_watch.match_price.match_vol as f64;
    let total_volume = market_watch.match_price.accumulated_volume as f64;
    let ceil_price = market_watch.listing_info.ceiling / 1000.0;
    let floor_price = market_watch.listing_info.floor / 1000.0;
    let high = market_watch.match_price.highest / 1000.0;
    let low = market_watch.match_price.lowest / 1000.0;

    SlintMarketWatchData {
        symbol: market_watch.listing_info.symbol.clone().into(),
        info: market_watch.listing_info.organ_name.clone().into(),
        match_price: price as f32,
        match_volume: volume as f32,
        change: change as f32,
        change_percent: change_percent as f32,
        volume: total_volume as f32,
        high: high as f32,
        low: low as f32,
        ref_price: ref_price as f32,
        ceil_price: ceil_price as f32,
        floor_price: floor_price as f32,
        ask_price1: ask1_price as f32,
        ask_price2: ask2_price as f32,
        ask_price3: ask3_price as f32,
        ask_volume1: if !market_watch.bid_ask.ask_prices.is_empty() {
            market_watch.bid_ask.ask_prices[0].volume as f32
        } else {
            0.0
        },
        ask_volume2: if market_watch.bid_ask.ask_prices.len() > 1 {
            market_watch.bid_ask.ask_prices[1].volume as f32
        } else {
            0.0
        },
        ask_volume3: if market_watch.bid_ask.ask_prices.len() > 2 {
            market_watch.bid_ask.ask_prices[2].volume as f32
        } else {
            0.0
        },
        bid_price1: bid1_price as f32,
        bid_price2: bid2_price as f32,
        bid_price3: bid3_price as f32,
        bid_volume1: if !market_watch.bid_ask.bid_prices.is_empty() {
            market_watch.bid_ask.bid_prices[0].volume as f32
        } else {
            0.0
        },
        bid_volume2: if market_watch.bid_ask.bid_prices.len() > 1 {
            market_watch.bid_ask.bid_prices[1].volume as f32
        } else {
            0.0
        },
        bid_volume3: if market_watch.bid_ask.bid_prices.len() > 2 {
            market_watch.bid_ask.bid_prices[2].volume as f32
        } else {
            0.0
        },
    }
}

pub fn is_trading_hours() -> bool {
    let now = chrono::Utc::now();
    // Convert to Vietnam time (UTC+7)
    let vietnam_time = now + chrono::Duration::hours(7);
    let current_hour = vietnam_time.hour();
    let current_minute = vietnam_time.minute();

    // Trading hours: 9:00 AM to 3:00 PM (15:00)
    let start_time = 9 * 60; // 9:00 AM in minutes
    let end_time = 15 * 60; // 3:00 PM in minutes
    let current_time_minutes = current_hour * 60 + current_minute;

    // Also check if it's a weekday (Monday = 1, Sunday = 7)
    let is_weekday = vietnam_time.weekday().num_days_from_monday() < 5;
    log::info!(
        "Current time: {}:{} - Trading hours: {} to {} - Is weekday: {}",
        current_hour,
        current_minute,
        start_time / 60,
        end_time / 60,
        is_weekday
    );

    is_weekday && current_time_minutes >= start_time && current_time_minutes < end_time
}


pub fn sort_market_watch(
    market_data: &[SlintMarketWatchData],
    sort_column: i32,
    sort_ascending: bool,
    show_percentage: bool,
) -> Vec<SlintMarketWatchData> {
    let mut sorted_data = market_data.to_vec();

    match sort_column {
        0 => {
            // Sort by symbol (alphabetical)
            if sort_ascending {
                sorted_data.sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
            } else {
                sorted_data.sort_by(|a, b| b.symbol.to_string().cmp(&a.symbol.to_string()));
            }
        }
        1 => {
            // Sort by ref price
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ref_price
                        .partial_cmp(&b.ref_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ref_price
                        .partial_cmp(&a.ref_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        2 => {
            // Sort by ceil price
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ceil_price
                        .partial_cmp(&b.ceil_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ceil_price
                        .partial_cmp(&a.ceil_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        3 => {
            // Sort by floor price
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.floor_price
                        .partial_cmp(&b.floor_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.floor_price
                        .partial_cmp(&a.floor_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        4 => {
            // Sort by bid price 3
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_price3
                        .partial_cmp(&b.bid_price3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_price3
                        .partial_cmp(&a.bid_price3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        5 => {
            // Sort by bid volume 3
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_volume3
                        .partial_cmp(&b.bid_volume3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_volume3
                        .partial_cmp(&a.bid_volume3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        6 => {
            // Sort by bid price 2
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_price2
                        .partial_cmp(&b.bid_price2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_price2
                        .partial_cmp(&a.bid_price2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        7 => {
            // Sort by bid volume 2
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_volume2
                        .partial_cmp(&b.bid_volume2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_volume2
                        .partial_cmp(&a.bid_volume2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        8 => {
            // Sort by bid price 1
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_price1
                        .partial_cmp(&b.bid_price1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_price1
                        .partial_cmp(&a.bid_price1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        9 => {
            // Sort by bid volume 1
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_volume1
                        .partial_cmp(&b.bid_volume1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_volume1
                        .partial_cmp(&a.bid_volume1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        10 => {
            // Sort by match price
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.match_price
                        .partial_cmp(&b.match_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.match_price
                        .partial_cmp(&a.match_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        11 => {
            // Sort by volume
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.match_volume
                        .partial_cmp(&b.match_volume)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.match_volume
                        .partial_cmp(&a.match_volume)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        12 => {
            // Sort by change (percentage or absolute depending on show_percentage mode)
            if show_percentage {
                // Sort by change percentage
                if sort_ascending {
                    sorted_data.sort_by(|a, b| {
                        a.change_percent
                            .partial_cmp(&b.change_percent)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    sorted_data.sort_by(|a, b| {
                        b.change_percent
                            .partial_cmp(&a.change_percent)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
            } else {
                // Sort by absolute change
                if sort_ascending {
                    sorted_data.sort_by(|a, b| {
                        a.change
                            .partial_cmp(&b.change)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    sorted_data.sort_by(|a, b| {
                        b.change
                            .partial_cmp(&a.change)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
            }
        }
        13 => {
            // Sort by ask price 1
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_price1
                        .partial_cmp(&b.ask_price1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_price1
                        .partial_cmp(&a.ask_price1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        14 => {
            // Sort by ask volume 1
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_volume1
                        .partial_cmp(&b.ask_volume1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_volume1
                        .partial_cmp(&a.ask_volume1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        15 => {
            // Sort by ask price 2
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_price2
                        .partial_cmp(&b.ask_price2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_price2
                        .partial_cmp(&a.ask_price2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        16 => {
            // Sort by ask volume 2
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_volume2
                        .partial_cmp(&b.ask_volume2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_volume2
                        .partial_cmp(&a.ask_volume2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        17 => {
            // Sort by ask price 3
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_price3
                        .partial_cmp(&b.ask_price3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_price3
                        .partial_cmp(&a.ask_price3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        18 => {
            // Sort by ask volume 3
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_volume3
                        .partial_cmp(&b.ask_volume3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_volume3
                        .partial_cmp(&a.ask_volume3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        19 => {
            // Sort by total volume
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.volume
                        .partial_cmp(&b.volume)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.volume
                        .partial_cmp(&a.volume)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        20 => {
            // Sort by high
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.high
                        .partial_cmp(&b.high)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.high
                        .partial_cmp(&a.high)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        21 => {
            // Sort by low
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.low
                        .partial_cmp(&b.low)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.low
                        .partial_cmp(&a.low)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        _ => {
            // No sorting for unknown columns
        }
    }

    sorted_data
}
