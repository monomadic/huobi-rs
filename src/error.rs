#![allow(dead_code)]
#![allow(unused_variables)]

use serde_json;

use ::reqwest::StatusCode;
impl From<StatusCode> for KucoinError {
    fn from(error: ::reqwest::StatusCode) -> Self {
        match error {
            StatusCode::Unauthorized => {
                Self {
                    error_type: KucoinErrorType::Unauthorized,
                    message: format!("Unauthorised request."),
                }
            },
            status => {
               Self {
                    error_type: KucoinErrorType::General,
                    message: format!("Received response: {:?}", status),
                }
            }
        }
    }
}

impl From<serde_json::Error> for KucoinError {
    fn from(error: serde_json::Error) -> Self {
        Self {
            error_type: KucoinErrorType::ParseError,
            message: error.to_string(),
        }
    }
}

impl From<::reqwest::Error> for KucoinError {
    fn from(error: ::reqwest::Error) -> Self {
       Self {
            error_type: KucoinErrorType::General,
            message: format!("Received response: {:?}", error),
        }
    }
}

#[derive(Debug)]
pub struct KucoinError {
    pub error_type: KucoinErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum KucoinErrorType {
    General,
    Unauthorized,
    ParseError,
}
