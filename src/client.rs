use reqwest;
use reqwest::{Response, StatusCode};
use reqwest::header::{ContentType, Headers, UserAgent};
use ring::{digest, hmac};

use time;
use hex::encode as hex_encode;
use base64;

use error::*;
use models::*;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
}

static API_HOST: &'static str = "https://api.kucoin.com";

impl Client {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Client {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }

    pub fn get(&self, endpoint: &str, request: &str) -> Result<String, KucoinError> {
        let mut url: String = format!("{}{}", API_HOST, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let response = reqwest::get(url.as_str()).expect("expected get request to be valid");

        self.handler(response)
    }

    pub fn get_signed(&self, endpoint: &str, request: &str) -> Result<String, KucoinError> {
        let sig = self.calculate_signature(endpoint, request);
        let url = format!("{}{}{}", API_HOST, endpoint, request);

        let client = reqwest::Client::new();
        let response = client
            .get(url.as_str())
            .headers(self.build_headers(&sig))
            .send()?;

        self.handler(response)
    }

    fn handler(&self, mut response: Response) -> Result<String, KucoinError> {
        use std::io::Read;

        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body).expect("read_to_string");
                Ok(body)
            },
            // error_status => Err(error_status),
            // StatusCode::InternalServerError => {
            //     // bail!("Internal Server Error");
            //     // reqwest::Error {
            //     //     kind: reqwest::error::Kind::f,
            //     //     url: "",
            //     // }

            // }
            // StatusCode::ServiceUnavailable => {
            //     // bail!("Service Unavailable");
            // }
            StatusCode::Unauthorized => {
                Err(KucoinError {
                    error_type: KucoinErrorType::Unauthorized,
                    message: format!("Unauthorised request."),
                })
            }
            // StatusCode::BadRequest => {
            //     // bail!(format!("Bad Request: {:?}", response));
            // }
            s => {
                Err(KucoinError {
                    error_type: KucoinErrorType::General,
                    message: format!("Received response: {:?}", s),
                })
            }
        }
    }

    fn nonce() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
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

    fn build_headers(&self, signature: &str) -> Headers {
        let mut custom_headers = Headers::new();

        custom_headers.set_raw("KC-API-KEY", self.api_key.as_str());
        custom_headers.set_raw("KC-API-NONCE", self::Client::nonce());
        custom_headers.set_raw("KC-API-SIGNATURE", signature);

        custom_headers.set_raw("HTTP_ACCEPT_LANGUAGE", "en-US");
        custom_headers.set_raw("Accept-Language", "en-US");
        custom_headers.set_raw("User-Agent", "kucoin-rs");
        custom_headers.set_raw("Accept", "application/json");

        custom_headers
    }
}
