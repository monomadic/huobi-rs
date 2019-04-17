#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt::Display;
use std::str::FromStr;

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse<R> {
    pub status: String,
    pub data: R,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIErrorResponse {
    pub status: String,

    #[serde(rename = "err-code")]
    pub err_code: Option<String>,

    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub state: String,

    #[serde(rename = "id")]
    pub account_id: u32,

    #[serde(rename = "type")]
    pub account_type: String,

    #[serde(rename = "subtype")]
    pub account_subtype: String,
}

pub type Currency = Vec<String>;
pub type Timestamp = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pair {
    #[serde(rename = "base-currency")]
    pub base_currency: String, // "eth", "btc"

    #[serde(rename = "quote-currency")]
    pub quote_currency: String, // "eth", "btc"

    #[serde(rename = "price-precision")]
    pub price_precision: u32,

    #[serde(rename = "amount-precision")]
    pub amount_precision: u32,

    #[serde(rename = "symbol-partition")]
    pub symbol_partition: String,

    #[serde(rename = "symbol")] // "edubtc", "linkusdt"
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kline {
    pub id: u32,
    pub amount: f64,
    pub count: u32,
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub vol: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub amount: f64,
    pub count: u32,
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub vol: f64,
    pub symbol: String,
}
