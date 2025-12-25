use chrono::{DateTime, Utc};
use company_info::CompanyInfo;
use market_watch::{MarketWatchResponse, VCIMarketWatch};
use ohlc::OHLCResponse;
pub use order_data::{OrderData, OrderList, VCIOderBook};
use reqwest::Error;
use serde_json::json;

use super::get_headers;

pub mod company_info;
pub mod market_watch;
pub mod ohlc;
mod order_data;

pub struct VCIExplorer {
    client: reqwest::Client,
}

impl VCIExplorer {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn get_quote(
        &self,
        symbol: &[&str],
        time_frame: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<OHLCResponse, Error> {
        let url = "https://trading.vietcap.com.vn/api/chart/OHLCChart/gap";
        let headers = get_headers("VCI");

        // Use provided times or default to 30-day range
        let now = chrono::Utc::now();
        let from = start_time.map(|t| t.timestamp()).unwrap_or_else(|| 0);
        let to = end_time
            .map(|t| t.timestamp())
            .unwrap_or_else(|| now.timestamp());

        // Build JSON payload with the provided symbol
        let payload = json!({
            "timeFrame": time_frame,
            "symbols": symbol,
            "from": from,
            "to": to
        });

        let response = self
            .client
            .request(reqwest::Method::POST, url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        let res = response.json::<OHLCResponse>().await?;

        Ok(res)
    }

    pub async fn get_market_watch(&self, symbols: &[&str]) -> Result<MarketWatchResponse, Error> {
        let url = "https://trading.vietcap.com.vn/api/price/symbols/getList";
        let headers = get_headers("VCI");

        // Build JSON payload with the provided symbol
        let payload = json!({
            "symbols": symbols,
        });
        log::info!("Requesting market watch for symbols: {symbols:?}");
        let response = self
            .client
            .request(reqwest::Method::POST, url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        let data: Vec<VCIMarketWatch> = response.json().await?;
        Ok(MarketWatchResponse(data))
    }

    pub async fn get_company_info(&self, symbol: &str, period: &str) -> Result<CompanyInfo, Error> {
        let url = "https://trading.vietcap.com.vn/data-mt/graphql";
        let headers = get_headers("VCI");

        // Build JSON payload with the provided symbol
        let payload = json!({
            "query": "fragment Ratios on CompanyFinancialRatio {\n  ticker\n  yearReport\n  lengthReport\n  updateDate\n  revenue\n  revenueGrowth\n  netProfit\n  netProfitGrowth\n  ebitMargin\n  roe\n  roic\n  roa\n  pe\n  pb\n  eps\n  currentRatio\n  cashRatio\n  quickRatio\n  interestCoverage\n  ae\n  netProfitMargin\n  grossMargin\n  ev\n  issueShare\n  ps\n  pcf\n  bvps\n  evPerEbitda\n  __typename\n}\n\nquery Query($ticker: String!, $period: String!) {\n  CompanyFinancialRatio(ticker: $ticker, period: $period) {\n    ratio {\n      ...Ratios\n      __typename\n    }\n    period\n    __typename\n  }\n}",
            "variables": {
                "ticker": symbol,
                "period": period,
            },
        });

        let response = self
            .client
            .request(reqwest::Method::POST, url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        let data: CompanyInfo = response.json().await?;
        Ok(data)
    }

    pub async fn get_order_list(&self, symbol: &str, limit: u32) -> Result<OrderList, Error> {
        let url = "https://trading.vietcap.com.vn/api/market-watch/LEData/getAll";
        let headers = get_headers("VCI");

        // Build JSON payload with the provided symbol
        let payload = json!({
            "symbol": symbol,
            "limit": limit
        });

        let response = self
            .client
            .request(reqwest::Method::POST, url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        let data: Vec<OrderData> = response.json().await?;
        let converted_data = data
            .into_iter()
            .map(|od| VCIOderBook {
                id: od.id,
                timestamp: od.trunc_time,
                price: od.match_price,
                volume: od.match_vol as i64,
                match_type: od.match_type,
            })
            .collect();
        Ok(converted_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_order_list() {
        let client = Client::new();
        let explorer = VCIExplorer::new(client);

        // Use a known symbol and a small limit for testing
        let symbol = "API";
        let limit = 10000;

        let result = explorer.get_order_list(symbol, limit).await;
        assert!(result.is_ok(), "API call failed: {result:?}");

        let orders = result.unwrap();
        println!("output: {orders:?}");
        assert!(!orders.is_empty(), "Order list should not be empty");
        for order in &orders {
            assert!(order.price > 0.0);
            assert!(order.volume > 0);
        }
    }
}
