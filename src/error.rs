#![allow(dead_code)]
#![allow(unused_variables)]

//use serde_json;
use core::fmt;
use std::error::Error;

pub type APIResult<T> = Result<T, Box<std::error::Error>>;

#[derive(Debug, Clone)]
pub enum HuobiError {
    ApiError(String),
}

impl fmt::Display for HuobiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            HuobiError::ApiError(why) => write!(f, "ApiError: {}", why),
        }
    }
}

impl Error for HuobiError {
    fn description(&self) -> &str {
        "Huobi Error"
    }
}

//
//use ::reqwest::StatusCode;
//impl From<StatusCode> for HuobiError {
//    fn from(error: ::reqwest::StatusCode) -> Self {
//        match error {
//            StatusCode::Unauthorized => {
//                Self {
//                    error_type: HuobiErrorType::Unauthorized,
//                    message: format!("Unauthorised request."),
//                }
//            },
//            status => {
//               Self {
//                    error_type: HuobiErrorType::General,
//                    message: format!("Received response: {:?}", status),
//                }
//            }
//        }
//    }
//}
//
//impl From<serde_json::Error> for HuobiError {
//    fn from(error: serde_json::Error) -> Self {
//        Self {
//            error_type: HuobiErrorType::ParseError,
//            message: error.to_string(),
//        }
//    }
//}
//
////impl From<reqwest::error::Error> for HuobiError {
////    fn from(error: reqwest::error::Error) -> Self {
////        Self {
////            error_type: HuobiErrorType::ParseError,
////            message: error.to_string(),
////        }
////    }
////}
//
//impl From<::reqwest::Error> for HuobiError {
//    fn from(error: ::reqwest::Error) -> Self {
//       Self {
//            error_type: HuobiErrorType::General,
//            message: format!("Received response: {:?}", error),
//        }
//    }
//}

