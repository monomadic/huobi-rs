use models::*;
use client::*;
use error::*;

use serde_json::from_str;

impl Client {

    /// undocumented call - may be broken at any time.
    pub fn balance(&self) -> Result<Vec<Balance>, KucoinError> {
        let data = self.get_signed("/v1/account/balance", "")?;
        // println!("{:?}", data);
        let response:APIResponse<Vec<Balance>> = from_str(data.as_str())?;
        // println!("\n\n\nSERDDDE ==== {:?}", response);
    
        Ok(response.data)
    }

    pub fn balances(&self) -> Result<Vec<Balance>, KucoinError> {
        let data = self.get_signed("/v1/account/balances", "")?;
        // println!("{:?}", data);
        let response:APIResponse<Balances> = from_str(data.as_str())?;

        Ok(response.data.balances)
    }

    /// list of all coins and prices
    pub fn symbols(&self) -> Result<Vec<Coin>, KucoinError> {
        let data = self.get_signed("/v1/market/open/symbols", "")?;
        // println!("{:?}", data);
        let response:APIResponse<Vec<Coin>> = from_str(data.as_str())?;

        Ok(response.data)
    }
}
