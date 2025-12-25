// pub mod btc;
pub mod explorer;

// pub use btc::get_btc_price;
use chrono::{DateTime, Utc};
use explorer::vci::VCIOderBook;
use reqwest::Client;

/// Get stock quote data for a given symbol
///
/// # Arguments
/// * `symbol` - The stock symbol to fetch data for
/// * `start_time` - Optional start time for the data range
/// * `end_time` - Optional end time for the data range
///
/// # Returns
/// * `Result<OHLCResponse, reqwest::Error>` - The OHLC data or an error
///
/// # Example
/// ```
/// use aim_stock;
/// use chrono::{DateTime, Utc};
///
/// #[tokio::main]
/// async fn main() {
///     aim_stock::init();
///     let data = aim_stock::get_quote("VCI", None, None).await.unwrap();
///     
///     // Access the first (and only) symbol's data
///     if let Some(ohlc) = data.0.first() {
///         println!("Symbol: {}", ohlc.symbol);
///         println!("Open prices: {:?}", ohlc.o);
///         println!("High prices: {:?}", ohlc.h);
///         println!("Low prices: {:?}", ohlc.l);
///         println!("Volumes: {:?}", ohlc.v);
///         println!("Timestamps: {:?}", ohlc.t);
///     }
/// }
/// ```
pub async fn get_quote(
    symbol: &[&str],
    time_frame: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
) -> Result<explorer::vci::ohlc::OHLCResponse, reqwest::Error> {
    let client = Client::new();
    let explorer = explorer::VCIExplorer::new(client);
    explorer
        .get_quote(symbol, time_frame, start_time, end_time)
        .await
}

pub async fn get_market_watch(
    symbols: &[&str],
) -> Result<explorer::vci::market_watch::MarketWatchResponse, reqwest::Error> {
    let client = Client::new();
    let explorer = explorer::VCIExplorer::new(client);
    explorer.get_market_watch(symbols).await
}

pub async fn get_company_info(
    symbols: &str,
) -> Result<explorer::vci::company_info::CompanyInfo, reqwest::Error> {
    let client = Client::new();
    let explorer = explorer::VCIExplorer::new(client);
    explorer.get_company_info(symbols, "Y").await
}

#[allow(dead_code)]
pub async fn get_order_list(symbol: &str) -> Result<Vec<VCIOderBook>, reqwest::Error> {
    let client = Client::new();
    let explorer = explorer::VCIExplorer::new(client);
    explorer.get_order_list(symbol, 30000).await
}

/// Re-export types for direct usage
pub use explorer::vci::ohlc::{Candlestick, OHLCData};
pub use explorer::*;

// mod test {
//     #[cfg(test)]
//     mod tests {
//         use crate::aim_data::{get_btc_price, get_company_info};
//         use tokio;

//         #[tokio::test]
//         async fn test_get_company_info() {
//             let symbol = "VCI";
//             let result = get_company_info(symbol).await;
//             println!("{:?}", result);
//             assert!(result.is_ok(), "Failed to get data for VCI");
//         }

//         #[tokio::test]
//         async fn test_get_btc_price() {
//             let result = get_btc_price().await;
//             println!("{:?}", result);
//             assert!(result.is_ok(), "Failed to get price of BTC");
//         }
//     }
// }
