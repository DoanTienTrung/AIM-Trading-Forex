use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::str::FromStr;

pub mod aim;
pub mod vci;
pub use vci::VCIExplorer;

// Default headers that are common across all data sources
static DEFAULT_HEADERS: &[(&str, &str)] = &[
    ("Accept", "application/json, text/plain, */*"),
    ("Accept-Language", "vi"),
    ("Connection", "keep-alive"),
    ("Content-Type", "application/json"),
    ("Cache-Control", "no-cache"),
    ("Sec-Fetch-Dest", "empty"),
    ("Sec-Fetch-Mode", "cors"),
    ("Sec-Fetch-Site", "same-site"),
    ("DNT", "1"),
    ("Pragma", "no-cache"),
    ("sec-ch-ua-platform", "\"Windows\""),
    ("sec-ch-ua-mobile", "?0"),
    (
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36",
    ),
];

/// Get headers for a specific data source
///
/// # Arguments
/// * `data_source` - The data source identifier (e.g., "VCI")
///
/// # Returns
/// * `HeaderMap` - The headers configured for the data source
pub fn get_headers(data_source: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    // Add default headers
    for (key, value) in DEFAULT_HEADERS {
        headers.insert(
            HeaderName::from_str(key).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }

    // Source-specific headers
    let mut source_headers: HashMap<&str, (&str, &str)> = HashMap::new();
    source_headers.insert(
        "VCI",
        (
            "https://trading.vietcap.com.vn/",
            "https://trading.vietcap.com.vn/",
        ),
    );

    if let Some((referer, origin)) = source_headers.get(&data_source.to_uppercase()[..]) {
        headers.insert("Referer", HeaderValue::from_str(referer).unwrap());
        headers.insert("Origin", HeaderValue::from_str(origin).unwrap());
    }

    headers
}
