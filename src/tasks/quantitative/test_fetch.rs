/// Test script to fetch and display data from Top 10 Market Cap and ICB Index APIs
/// Run with: cargo run --bin test_fetch (after adding bin target in Cargo.toml)
/// Or run directly: cargo test --test test_fetch

use aim_data::explorer::aim::{fetch_icb_index_data_filtered, fetch_top_10_market_cap_data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("Testing API fetch for Quantitative Analysis...\n");

    // Test 1: Fetch Top 10 Market Cap
    println!("TEST 1: Fetching Top 10 Market Cap from http://103.48.84.52:4040/top-10-market-cap");
    
    match fetch_top_10_market_cap_data().await {
        Ok(data) => {
            println!("Success! Fetched {} items\n", data.len());
            println!("{:<10} {:<15} {:<12} {:<10}", "Mã CK", "Vốn Hóa (tỷ)", "Giá", "% Thay đổi");
            println!("{}", "-".repeat(50));
            for (_i, item) in data.iter().enumerate() {
                let market_cap_billion = item.market_cap as f64 / 1_000_000_000.0;
                println!(
                    "{:<10} {:<15.2} {:<12.2} {:<10.2}%",
                    item.stock_code,
                    market_cap_billion,
                    item.price,
                    item.change_percent
                );
            }
        }
        Err(e) => {
            println!("Error: {}\n", e);
        }
    }

    println!("\n");

    // Test 2: Fetch ICB Index Filtered (industry_code 2 digits)
    println!("TEST 2: Fetching ICB Index Filtered (industry_code 2 digits) from https://103.48.84.52:4443/icb-index");
    
    match fetch_icb_index_data_filtered().await {
        Ok(data) => {
            let total_market_cap: i64 = data.iter().map(|x| x.market_cap).sum();
            let total_volume: i64 = data.iter().map(|x| x.volume).sum();
            let total_value: i64 = data.iter().map(|x| x.value).sum();
            
            println!("Success! Fetched {} items (industry_code 2 digits)\n", data.len());
            println!("TONG HOP:");
            println!("  - Tong Von Hoa: {:.2} ty VND", total_market_cap as f64 / 1_000_000_000.0);
            println!("  - Tong KL Giao dich: {}", total_volume);
            println!("  - Tong GT Giao dich: {:.2} ty VND\n", total_value as f64 / 1_000_000_000.0);
            
            println!("{:<15} {:<30} {:<15} {:<15} {:<15}", "Industry Code", "ICB Name", "Market Cap (ty)", "Volume", "Value (ty)");
            println!("{}", "-".repeat(90));
            for item in data.iter().take(10) {
                println!(
                    "{:<15} {:<30} {:<15.2} {:<15} {:<15.2}",
                    item.industry_code,
                    if item.icb_name.len() > 28 { &item.icb_name[..28] } else { &item.icb_name },
                    item.market_cap as f64 / 1_000_000_000.0,
                    item.volume,
                    item.value as f64 / 1_000_000_000.0
                );
            }
            if data.len() > 10 {
                println!("... va {} items khac", data.len() - 10);
            }
        }
        Err(e) => {
            println!("Error: {}\n", e);
        }
    }

    println!("\nTest completed!\n");

    Ok(())
}

