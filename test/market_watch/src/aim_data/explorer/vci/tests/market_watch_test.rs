use crate::aim_data::explorer::vci::{VCIExplorer, models::MarketWatchResponse};
use reqwest::Client;
use tokio;

#[tokio::test]
async fn test_get_market_watch_single_symbol() {
    let client = Client::new();
    let explorer = VCIExplorer::new(client);
    let symbols = vec!["VNM"];
    
    let result = explorer.get_market_watch(&symbols).await;
    assert!(result.is_ok(), "Failed to get market watch data for VNM");
    
    let market_watch = result.unwrap();
    assert_eq!(market_watch.0.len(), 1, "Expected exactly one stock in response");
    
    let stock = &market_watch.0[0];
    assert_eq!(stock.listing_info.symbol, "VNM", "Symbol mismatch");
}

#[tokio::test]
async fn test_get_market_watch_multiple_symbols() {
    let client = Client::new();
    let explorer = VCIExplorer::new(client);
    let symbols = vec!["VNM", "TCB", "VPB"];
    
    let result = explorer.get_market_watch(&symbols).await;
    println!("{:?}", result);
    assert!(result.is_ok(), "Failed to get market watch data for multiple symbols");
    
    let market_watch = result.unwrap();
    assert_eq!(market_watch.0.len(), 3, "Expected exactly three stocks in response");
    
    // Verify each symbol is present in the response
    let symbols_in_response: Vec<&str> = market_watch.0.iter()
        .map(|stock| stock.listing_info.symbol.as_str())
        .collect();
    
    assert!(symbols_in_response.contains(&"VNM"), "VNM not found in response");
    assert!(symbols_in_response.contains(&"TCB"), "TCB not found in response");
    assert!(symbols_in_response.contains(&"VPB"), "VPB not found in response");
}

#[tokio::test]
async fn test_get_market_watch_invalid_symbol() {
    let client = Client::new();
    let explorer = VCIExplorer::new(client);
    let symbols = vec!["INVALID_SYMBOL"];
    
    let result = explorer.get_market_watch(&symbols).await;
    assert!(result.is_err(), "Expected error for invalid symbol");
}

#[tokio::test]
async fn test_get_market_watch_empty_symbols() {
    let client = Client::new();
    let explorer = VCIExplorer::new(client);
    let symbols: Vec<&str> = vec![];
    
    let result = explorer.get_market_watch(&symbols).await;
    assert!(result.is_ok(), "Expected OK response for empty symbols list");
    
    let market_watch = result.unwrap();
    assert_eq!(market_watch.0.len(), 0, "Expected empty response for empty symbols list");
}

#[tokio::test]
async fn test_get_market_watch_data_structure() {
    let client = Client::new();
    let explorer = VCIExplorer::new(client);
    let symbols = vec!["VNM"];
    
    let result = explorer.get_market_watch(&symbols).await;
    assert!(result.is_ok(), "Failed to get market watch data");
    
    let market_watch = result.unwrap();
    let stock = &market_watch.0[0];
    
    // Verify the structure of the response
    assert!(!stock.listing_info.symbol.is_empty(), "Symbol should not be empty");
    assert!(stock.match_price.match_price >= 0.0, "Price should be non-negative");
    assert!(stock.bid_ask.bid_prices.iter().all(|v| v.price >= 0.0), "Bid volumes should be non-negative");
    assert!(stock.bid_ask.ask_prices.iter().all(|v| v.price >= 0.0), "Ask volumes should be non-negative");
}