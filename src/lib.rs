// Huobi API
//
// references:
//  - https://github.com/huobiapi/API_Docs_en/wiki/Huobi.pro-API

mod client;
pub mod error;
pub mod models;

pub use crate::client::Client;
pub use crate::models::*;
pub use crate::error::*;
