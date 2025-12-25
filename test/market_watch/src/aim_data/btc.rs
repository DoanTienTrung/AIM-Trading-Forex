// https://api.kraken.com/0/public/Trades?pair=BTCUSD

pub async fn get_btc_price() -> Result<f32, reqwest::Error> {
    let url = "https://api.kraken.com/0/public/Trades?pair=BTCUSD";
    let response = reqwest::get(url).await?.error_for_status()?;
    let data: serde_json::Value = response.json().await?;
    let price = data["result"]["XXBTZUSD"][0][0].as_str().unwrap_or("0.0");
    Ok(price.parse::<f32>().unwrap_or(0.0))
}
