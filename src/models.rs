#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;
use std::str::FromStr;
//use serde::de::{self, Deserialize, Deserializer};
use serde::{de, Deserialize, Deserializer, Serialize};

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

    #[serde(rename = "err-code")]
    pub err_code: Option<String>,

    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    account_id: u32,
    user_id: u32,
    account_type: String,
    state: String,
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
