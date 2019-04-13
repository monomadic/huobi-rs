use models::*;
use client::*;
use error::*;

use serde_json::from_str;

impl Client {

    /// undocumented call - may be broken at any time.
    pub fn balance(&self) -> Result<Vec<Balance>, HuobiError> {
        let data = self.get_signed("/v1/account/balance", "")?;
        let response:APIResponse<Vec<Balance>> = from_str(data.as_str())?;
    
        Ok(response.data.into_iter().filter(|b| b.balance > 0.0).collect())
    }

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
