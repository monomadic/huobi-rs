use crate::{error::*, models::*};
use core::fmt::Debug;
use hex::encode as hex_encode;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use ring::{digest, hmac};
use std::collections::BTreeMap;

mod account;
mod market;

use self::account::*;
use self::market::*;

// re-exports
pub use self::account::*;
pub use self::market::*;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
}

#[derive(Clone)]
pub struct APIKey {
    api_key: String,
    secret_key: String,
}

static API_HOST: &'static str = "api.huobi.pro";

impl Client {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Client {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }

    pub fn get(&self, endpoint: &str, request: &str) -> APIResult<String> {
        let mut url: String = format!("{}{}", API_HOST, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let result = reqwest::get(url.as_str())?.text()?;
        Ok(result)
    }

    pub fn get_signed(&self, endpoint: &str, params: &str) -> APIResult<String> {
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("AccessKeyId".to_string(), self.api_key.clone());
        params.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());
        params.insert("SignatureVersion".to_string(), "2".to_string());
        params.insert("Timestamp".to_string(), get_timestamp());

        let params = build_query_string(params);
        let signature = sign_hmac_sha256_base64(
            &self.secret_key,
            &format!("{}\n{}\n{}\n{}", "GET", API_HOST, endpoint, params,),
        )
        .to_string();

        let request = format!(
            "https://{}{}?{}&Signature={}",
            API_HOST,
            endpoint,
            params,
            percent_encode(&signature.clone())
        );

        let mut response = reqwest::get(request.as_str())?;
        let body = response.text()?;

        // check for errors
        let err_response: APIErrorResponse = serde_json::from_str(body.as_str())?;

        if err_response.status == "error" {
            if let Some(err_msg) = err_response.err_msg {
                return Err(Box::new(HuobiError::ApiError(err_msg)));
            } else {
                return Err(Box::new(HuobiError::ApiError(format!(
                    "result dump: {:?}",
                    err_response
                ))));
            }
        }

        Ok(body)
    }
}

/// Compiles query string parameters into a http query string, in byte-order.
///
/// ```rust
/// let mut params = ::std::collections::BTreeMap::new();
/// params.insert("AccessKeyId".to_string(), "e2xxxxxx".to_string());
/// params.insert("order-id".to_string(), "1234567890".to_string());
/// params.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());
/// params.insert("SignatureVersion".to_string(), "2".to_string());
/// params.insert("Timestamp".to_string(), "2017-05-11T15:19:30".to_string());
///
/// assert_eq!(huobi::client::build_query_string(params), "AccessKeyId=e2xxxxxx&SignatureMethod=HmacSHA256&SignatureVersion=2&Timestamp=2017-05-11T15%3A19%3A30&order-id=1234567890");
/// ```
pub fn build_query_string(parameters: BTreeMap<String, String>) -> String {
    parameters
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, percent_encode(&value.clone())))
        .collect::<Vec<String>>()
        .join("&")
}

/// Compiles query string parameters into a http query string, in byte-order.
///
/// ```rust
/// assert_eq!(huobi::client::sign_hmac_sha256_base64("b0xxxxxx-c6xxxxxx-94xxxxxx-dxxxx",
///     "GET\napi.huobi.pro\n/v1/order/orders\nAccessKeyId=e2xxxxxx-99xxxxxx-84xxxxxx-7xxxx&SignatureMethod=HmacSHA256&SignatureVersion=2&Timestamp=2017-05-11T15%3A19%3A30&order-id=1234567890"),
///     "Nmd8AU8uAe0mkFpxNbiava0aeZzBEtYjCdie1ZYZjoM=");
/// ```
pub fn sign_hmac_sha256_base64(secret: &str, digest: &str) -> String {
    use data_encoding::BASE64;

    let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
    let signature = hmac::sign(&signed_key, digest.as_bytes());
    let b64_encoded_sig = BASE64.encode(signature.as_ref());

    b64_encoded_sig
}

/// Percent encode parameter values for url distribution.
///
/// ```rust
/// assert_eq!(huobi::client::percent_encode("2017-05-11T15:19:30"), "2017-05-11T15%3A19%3A30");
/// assert_eq!(huobi::client::percent_encode("WyZoIcQwHFT/Y9pALN/PYSDoyqmmIBp4w9D+k/NnSo4="), "WyZoIcQwHFT%2FY9pALN%2FPYSDoyqmmIBp4w9D%2Bk%2FNnSo4%3D");
/// ```
pub fn percent_encode(source: &str) -> String {
    use percent_encoding::{define_encode_set, utf8_percent_encode, USERINFO_ENCODE_SET};

    define_encode_set! {
        pub CUSTOM_ENCODE_SET = [USERINFO_ENCODE_SET] | { '+' }
    }

    let signature = utf8_percent_encode(&source, CUSTOM_ENCODE_SET).to_string();

    signature
}

pub fn get_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let utc_time = chrono::Utc::now();
    let formatted_time = utc_time.format("%Y-%m-%dT%H:%M:%S").to_string();

    formatted_time
}
