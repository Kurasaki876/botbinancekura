use crate::config::Settings;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;

#[derive(Debug, Deserialize, Clone)]
pub struct SymbolInfo {
    pub symbol: String,
    pub quoteAsset: String,
    pub baseAsset: String,
    pub isSpotTradingAllowed: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ticker24h {
    pub symbol: String,
    pub volume: String,
    pub askPrice: String,
    pub bidPrice: String,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    pub balances: Vec<Balance>,
}

pub async fn fetch_symbols(_settings: &Settings) -> Vec<String> {
    let client = reqwest::Client::new();
    let url = "https://api.binance.com/api/v3/exchangeInfo";

    let res = client.get(url).send().await.unwrap();
    let data: serde_json::Value = res.json().await.unwrap();

    let mut valid_symbols: Vec<String> = vec![];

    if let Some(symbols) = data["symbols"].as_array() {
        for s in symbols {
            if s["status"] == "TRADING"
                && s["isSpotTradingAllowed"] == true
                && s["quoteAsset"] == "USDT"
            {
                valid_symbols.push(s["symbol"].as_str().unwrap().to_string());
            }
        }
    }

    valid_symbols
}

pub async fn filter_operable_symbols(settings: &Settings) -> Vec<String> {
    let symbols = fetch_symbols(settings).await;
    let client = reqwest::Client::new();

    let mut filtered = vec![];

    for symbol in symbols {
        let url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", symbol);
        let res = client.get(&url).send().await;

        if let Ok(resp) = res {
            if let Ok(ticker) = resp.json::<Ticker24h>().await {
                let volume: f64 = ticker.volume.parse().unwrap_or(0.0);
                let ask: f64 = ticker.askPrice.parse().unwrap_or(0.0);
                let bid: f64 = ticker.bidPrice.parse().unwrap_or(0.0);
                let spread = if ask > 0.0 { (ask - bid) / ask } else { 1.0 };

                if volume >= settings.core.min_volume && spread <= settings.core.max_spread {
                    filtered.push(symbol);
                }
            }
        }
    }

    filtered
}

pub async fn get_usdt_balance(settings: &Settings) -> f64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let query_string = format!("timestamp={}", timestamp);
    let mut mac = Hmac::<Sha256>::new_from_slice(settings.binance_api_secret.as_bytes()).unwrap();
    mac.update(query_string.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    let full_url = format!(
        "https://api.binance.com/api/v3/account?{}&signature={}",
        query_string, signature
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-MBX-APIKEY",
        HeaderValue::from_str(&settings.binance_api_key).unwrap(),
    );

    let client = reqwest::Client::new();
    let res = client.get(&full_url).headers(headers).send().await.unwrap();
    let info: AccountInfo = res.json().await.unwrap();

    let usdt = info
        .balances
        .into_iter()
        .find(|b| b.asset == "USDT")
        .map(|b| b.free.parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0);

    usdt
}
