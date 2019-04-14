use reqwest::{Response, StatusCode};
use ring::{digest, hmac};
use hex::encode as hex_encode;
use crate::{ error::*, models::* };
//use base64;
//use time;

mod market;

#[derive(Clone)]
pub struct Client {
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

        println!("{:#?}", result);

        Ok(result)
    }

    pub fn get_signed(&self, endpoint: &str, request: &str) -> APIResult<String> {
        let sig = self.calculate_signature(endpoint, request);
        let url = format!("{}{}{}", API_HOST, endpoint, request);
        let client = reqwest::ClientBuilder::new().default_headers(self.build_headers(&sig)?).build()?;

        Ok(reqwest::get(url.as_str())?.text()?)
    }

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

    fn calculate_signature(&self, endpoint: &str, query_string: &str) -> String {
        let url = format!("{}/{}/{}", endpoint, self::Client::nonce(), query_string);

        let base64_url = base64::encode(&url.as_bytes());
        let signed_key = hmac::SigningKey::new(&digest::SHA256, self.secret_key.as_bytes());
        let signature = hex_encode(hmac::sign(&signed_key, base64_url.as_bytes()));
        signature
    }

    fn build_headers(&self, signature: &str) -> APIResult<reqwest::header::HeaderMap> {
        let mut custom_headers = reqwest::header::HeaderMap::new();

        custom_headers.insert("KC-API-KEY", reqwest::header::HeaderValue::from_str(&self.api_key)?);



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
