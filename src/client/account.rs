use models::*;
use client::*;
use error::*;

use serde_json::from_str;

impl Client {

    pub fn balances(&self) -> Result<Vec<Balance>, HuobiError> {
        let data = self.get_signed("/v1/account/balances", "")?;
        let response:APIResponse<Balances> = from_str(data.as_str())?;

        Ok(response.data.balances)
    }

    /// list of all coins and prices
    pub fn symbols(&self) -> Result<Vec<Coin>, HuobiError> {
        let data = self.get_signed("/v1/market/open/symbols", "")?;
        let response:APIResponse<Vec<Coin>> = from_str(data.as_str())?;

        Ok(response.data)
    }
}
