use serde::Deserialize;
use std::collections::HashMap;
pub use switchboard_utils::reqwest;
use rust_decimal::Decimal;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct KrakenTickerResponse {
    pub result: HashMap<String, KrakenTickerInfo>,
}

#[derive(Debug, Deserialize)]
pub struct KrakenOHLCResponse {
    // error: Vec<String>,
    result: HashMap<String, Value>,
}

impl KrakenOHLCResponse {
    pub fn parse(&self, symbol: &str) -> Option<Vec<KrakenOHLC>> {
        for (k, v) in self.result.iter() {
            if k == symbol {
                return serde_json::from_value(v.clone()).unwrap();
            }
        }
        None
    }
}

// https://api.kraken.com/0/public/Ticker
// https://docs.kraken.com/rest/#tag/Market-Data/operation/getTickerInformation
#[derive(Debug, Deserialize, Clone)]
pub struct KrakenTickerInfo {
    #[serde(rename = "a")]
    pub ask: Vec<Decimal>,
    #[serde(rename = "b")]
    pub bid: Vec<Decimal>,
    #[serde(rename = "c")]
    pub close: Vec<Decimal>,
    #[serde(rename = "v")]
    pub volume: Vec<Decimal>,
    #[serde(rename = "p")]
    pub vwap: Vec<Decimal>,
    #[serde(rename = "t")]
    pub trade_count: Vec<i64>,
    #[serde(rename = "l")]
    pub low: Vec<Decimal>,
    #[serde(rename = "h")]
    pub high: Vec<Decimal>,
    #[serde(rename = "o")]
    pub open: Decimal,
}

impl KrakenTickerInfo {
    pub fn price(&self) -> Decimal {
        (self.ask[0] + self.bid[0]) / Decimal::from(2)
    }
}

#[derive(Debug, Deserialize)]
pub struct KrakenOHLC {
    #[serde(rename = "0")]
    pub time: i128,
    #[serde(rename = "1")]
    pub open: Decimal,
    #[serde(rename = "2")]
    pub high: Decimal,
    #[serde(rename = "3")]
    pub low: Decimal,
    #[serde(rename = "4")]
    pub close: Decimal,
    #[serde(rename = "5")]
    pub vwap: Decimal,
    #[serde(rename = "6")]
    pub volume: Decimal,
    #[serde(rename = "7")]
    pub count: i64,
}
