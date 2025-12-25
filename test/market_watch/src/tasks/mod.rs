use std::{fs::File, io::Write, path::PathBuf};

pub mod chart;
pub mod market_watch;

// Re-export the task functions for easier access
use crate::{aim_data::explorer::vci::OrderList, slint_generatedAppWindow};
pub use chart::*;
pub use market_watch::*;

// Import StockData with a more specific name to avoid conflicts
use slint_generatedAppWindow::{
    MarketWatchData as SlintMarketWatchData
};


pub enum DataUpdate {
    MarketWatchData(Vec<SlintMarketWatchData>),
    OrdList(OrderList),
    CustomList(Vec<String>),
}
