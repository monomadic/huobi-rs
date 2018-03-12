use reqwest;
use reqwest::{Response, StatusCode};
use reqwest::header::{ContentType, Headers, UserAgent};
use ring::{digest, hmac};

use time;
use hex::encode as hex_encode;

use error::*;
use models::*;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
}

static API_HOST: &'static str = "https://api.kucoin.com/v1";

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
        let url = self.sign_request(endpoint, request);
        let client = reqwest::Client::new();
        let response = client
            .get(url.as_str())
            .headers(self.build_headers(true))
            .send()?;

        self.handler(response)
    }

    fn handler(&self, mut response: Response) -> Result<String, KucoinError> {
        use std::io::Read;

        match response.status() {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
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

    fn sign_request(&self, endpoint: &str, request: &str) -> String {
        let signed_key = hmac::SigningKey::new(&digest::SHA256, self.secret_key.as_bytes());
        let signature = hex_encode(hmac::sign(&signed_key, request.as_bytes()).as_ref());

        let request_body: String = format!("{}&signature={}", request, signature);
        let url: String = format!("{}{}?{}", API_HOST, endpoint, request_body);

        url
    }

    fn build_headers(&self, content_type: bool) -> Headers {
        let mut custom_headers = Headers::new();

        custom_headers.set(UserAgent::new("kucoin-rs"));
        if content_type {
            custom_headers.set(ContentType::form_url_encoded());
        }
        custom_headers.set_raw("KC-API-KEY", self.api_key.as_str());
        custom_headers.set_raw("KC-API-NONCE", time::precise_time_ns().to_string());
        // custom_headers.set_raw("KC-API-SIGNATURE", "".into());

        custom_headers
    }
}
