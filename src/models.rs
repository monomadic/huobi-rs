#![allow(dead_code)]
#![allow(unused_variables)]

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub id: u32,
    #[serde(rename = "type")]
    pub account_type: String,
    pub state: String,
    pub list: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub currency: String,
    #[serde(rename = "type")]
    pub trade_type: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub balance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub source: String,
    pub state: String,

    #[serde(rename = "account-id")]
    pub account_id: u64,

    #[serde(deserialize_with = "string_as_f64")]
    pub amount: f64,

    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,

    #[serde(rename = "created-at")]
    pub created_at: u64,

    #[serde(rename = "type")]
    pub order_type: String,

    #[serde(rename = "field-amount")]
    #[serde(deserialize_with = "string_as_f64")]
    pub field_amount: f64,

    #[serde(rename = "field-cash-amount")]
    #[serde(deserialize_with = "string_as_f64")]
    pub field_cash_amount: f64,

    #[serde(rename = "field-fees")]
    #[serde(deserialize_with = "string_as_f64")]
    pub field_fees: f64,

    #[serde(rename = "finished-at")]
    pub finished_at: u64,

    #[serde(rename = "canceled-at")]
    pub canceled_at: u64,
}

fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(F64Visitor)
}

struct F64Visitor;
impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a f64")
    }
    fn visit_str<E>(self, value: &str) -> Result<f64, E>
    where
        E: de::Error,
    {
        if let Ok(integer) = value.parse::<i32>() {
            Ok(integer as f64)
        } else {
            value.parse::<f64>().map_err(|err| {
                E::invalid_value(Unexpected::Str(value), &"a string representation of a f64")
            })
        }
    }
}
