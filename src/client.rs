use crate::{error::*, models::*};
use hex::encode as hex_encode;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use ring::{digest, hmac};
//use base64;
//use time;
use core::fmt::Debug;
use std::collections::BTreeMap;

mod account;
mod market;

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

static API_HOST: &'static str = "https://api.huobi.pro";

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

    pub fn get_signed(&self, endpoint: &str, request: &str) -> APIResult<String> {
        let url = format!("{}{}{}", API_HOST, endpoint, request);
        let client = reqwest::ClientBuilder::new().build()?;

        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("AccessKeyId".to_string(), self.api_key.clone());
        params.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());
        params.insert("SignatureVersion".to_string(), "2".to_string());

        // time
        use percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
        use std::time::{SystemTime, UNIX_EPOCH};

        let utc_time = chrono::Utc::now();
        let formatted_time = utc_time.format("%Y-%m-%dT%H%%3A%M%%3A%S").to_string();
        println!("timestamp: {}", formatted_time.clone());
        params.insert("Timestamp".to_string(), formatted_time);
        // end time

        let query_string = build_query_string(params);
        let signature = sign_hmac_sha256_base64(&self.secret_key, &query_string);
        let request = format!(
            "{}{}?{}&Signature={}",
            API_HOST, endpoint, query_string, signature
        );

        println!("request = {}", request.clone());
        let mut response = reqwest::get(request.as_str())?;
        let body = response.text()?;

        // check for errors
        let err_response: APIResponse<Option<String>> = serde_json::from_str(body.as_str())?;
        println!("err_response: {:?}", err_response);

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

        println!("{:#?}", response);
        let body = response.text()?;
        Ok(body)
    }

    //    fn handler(&self, mut response: Response) -> APIResult<String> {}

    //    if result. == "error".to_string() {
    //    if let Some(err_msg) = result.err_code {
    //    return Err(Box::new(HuobiError::ApiError(err_msg)));
    //    } else {
    //    return Err(Box::new(HuobiError::ApiError(format!(
    //    "result dump: {:?}",
    //    result
    //    ))));
    //    }
    //    }
    //    fn handler(&self, mut response: Response) -> APIResult<String> {
    //        if response.status().is_success() {
    //            Ok(response.text().unwrap())
    //        } else {
    //            Err(Box::new(response.error_for_status()))
    //        }
    //
    //        StatusCode
    //
    //
    ////        match response.status().as_u16() {
    ////            StatusCode::from_u16(200) => {
    ////                let mut body = String::new();
    ////                response.read_to_string(&mut body).unwrap();
    ////                Ok(body)
    ////            }
    ////            StatusCode::InternalServerError => {
    ////                bail!("Internal Server Error");
    ////            }
    ////            StatusCode::ServiceUnavailable => {
    ////                bail!("Service Unavailable");
    ////            }
    ////            StatusCode::Unauthorized => {
    ////                bail!("Unauthorized");
    ////            }
    ////            StatusCode::BadRequest => {
    ////                bail!(format!("Bad Request: {:?}", response));
    ////            }
    ////            s => {
    ////                bail!(format!("Received response: {:?}", s));
    ////            }
    ////        }
    //    }

    fn nonce() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let nonce = since_the_epoch.as_secs() * 1000; // +

        nonce.to_string()
    }

    //    fn calculate_signature(&self, endpoint: &str, query_string: &str) -> String {}

    fn build_headers(&self, signature: &str) -> APIResult<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        //        custom_headers.insert("KC-API-KEY", HeaderValue::from_str(&self.api_key)?);

        //                              HeaderValue::new() self.api_key.as_str());
        //        custom_headers.set_raw("KC-API-KEY", self.api_key.as_str());
        //        custom_headers.set_raw("KC-API-NONCE", self::Client::nonce());
        //        custom_headers.set_raw("KC-API-SIGNATURE", signature);
        //
        //        custom_headers.set_raw("HTTP_ACCEPT_LANGUAGE", "en-US");
        //        custom_headers.set_raw("Accept-Language", "en-US");
        //        custom_headers.set_raw("User-Agent", "kucoin-rs");
        //        custom_headers.set_raw("Accept", "application/json");

        Ok(custom_headers)
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
/// params.insert("Timestamp".to_string(), "2017-05-11T15%3A19%3A30".to_string());
///
/// assert_eq!(huobi::client::build_query_string(params), "AccessKeyId=e2xxxxxx&SignatureMethod=HmacSHA256&SignatureVersion=2&Timestamp=2017-05-11T15%3A19%3A30&order-id=1234567890".to_string());
/// ```
pub fn build_query_string(mut parameters: BTreeMap<String, String>) -> String {
    //    parameters.insert(
    //        "AccessKeyId".to_string(),
    //        "e2xxxxxx-99xxxxxx-84xxxxxx-7xxxx".to_string(),
    //    );
    //    parameters.insert("SignatureVersion".to_string(), "2".to_string());
    //    parameters.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());

    parameters
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

///// Compiles query string parameters into a http query string, in byte-order.
/////
///// ```rust
///// assert_eq!(huobi::client::compile_request("GET", "api.huobi.pro", "/v1/order/orders", "AccessKeyId=e2xxxxxx-99xxxxxx-84xxxxxx-7xxxx&order-id=1234567890&SignatureMethod=HmacSHA256&SignatureVersion=2&Timestamp=2017-05-11T15%3A19%3A30", "b0xxxxxx-c6xxxxxx-94xxxxxx-dxxxx"), "https://api.huobi.pro/v1/order/orders?AccessKeyId=e2xxxxxx-99xxxxxx-84xxxxxx-7xxxx&order-id=1234567890&SignatureMethod=HmacSHA256&SignatureVersion=2&Timestamp=2017-05-11T15%3A19%3A30&Signature=4F65x5A2bLyMWVQj3Aqp%2BB4w%2BivaA7n5Oi2SuYtCJ9o%3D".to_string());
///// ```
//pub fn compile_request(
//    method: &str,
//    host: &str,
//    endpoint: &str,
//    params: &str,
//    secret: &str,
//) -> String {
//    let url = format!("{}\n{}\n{}\n{}", method, host, endpoint, params);
//    let base64_url = base64::encode(&url.as_bytes());
//    let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
//    let signature = hex_encode(hmac::sign(&signed_key, base64_url.as_bytes()));
//
//    format!(
//        "https://{}{}?{}&Signature={}",
//        host, endpoint, params, base64_url
//    )
//}

/// Compiles query string parameters into a http query string, in byte-order.
///
/// ```rust
/// assert_eq!(huobi::client::sign_hmac_sha256_base64("b0xxxxxx-c6xxxxxx-94xxxxxx-dxxxx", "GET\napi.huobi.pro\n/v1/order/orders\nAccessKeyId=e2xxxxxx-99xxxxxx-84xxxxxx-7xxxx&SignatureMethod=HmacSHA256&SignatureVersion=2&Timestamp=2017-05-11T15%3A19%3A30&order-id=1234567890"), "Nmd8AU8uAe0mkFpxNbiava0aeZzBEtYjCdie1ZYZjoM=");
/// ```
pub fn sign_hmac_sha256_base64(secret: &str, params: &str) -> String {
    //    let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
    //    //    hmac::sign(&signed_key, params.as_bytes());
    //    //    let signature = hex_encode(hmac::sign(&signed_key, params.as_bytes()));
    //    let signature = hmac::sign(&signed_key, params.as_bytes());
    //    //    println!("  - {:02x?}", signature);
    //    //    let signature = format!("{:02x?}", signature);
    //    let base64_url = base64::encode(&signature.as_bytes());
    //    hex_encode(base64_url)

    //    use crypto::digest::Digest;
    //    use crypto::sha2::Sha256;
    //    use serialize::base64::{ToBase64, STANDARD};
    //
    //    let mut sha = Sha256::new();
    //    sha.input_str(secret);
    //    //    Vec::from_elem(sha.output_bytes(), 0u8);
    //    Vec::from_iter(repeat(0u8).take(sha.output_bytes()));
    //
    //    sha.result(params.as_mut_slice());
    //    format!("{}", bytes.to_base64(STANDARD))

    use data_encoding::BASE64;

    //    let secret_key = "b0xxxxxx-c6xxxxxx-94xxxxxx-dxxxx";
    //    let payload = "GET\napi.huobi.pro\n/v1/order/orders\nAccessKeyId=e2xxxxxx-99xxxxxx-84xxxxxx-7xxxx&SignatureMethod=HmacSHA256&SignatureVersion=2&Timestamp=2017-05-11T15%3A19%3A30&order-id=1234567890";

    let signed_key = hmac::SigningKey::new(&digest::SHA256, secret.as_bytes());
    let signature = hmac::sign(&signed_key, params.as_bytes());
    let b64_encoded_sig = BASE64.encode(signature.as_ref());

    b64_encoded_sig
}
